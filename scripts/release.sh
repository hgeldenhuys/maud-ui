#!/usr/bin/env bash
# release.sh — rebuild and check maud-ui, commit generated assets, publish the
# crate, push all remotes, and verify the website and crates.io version.
#
# Bump Cargo.toml, write the newest CHANGELOG entry, and commit before running.
# Publishing is irreversible: a crates.io version can never be replaced.
#
# Usage:
#   scripts/release.sh --marker '<css text new in this release>' [--dry-run] [--visual-ok]
#   scripts/release.sh --help
#
# Step 5b screenshots the freshly exported site against the live one (the previous release).
# Any changed page stops the release with a report path; look at it, and if every change is
# intended, re-run with --visual-ok.
#
# --dry-run rebuilds and checks everything, but never commits, publishes or pushes.
# Another session owns docs/night-6-live-events.jsonl. Never stage that log.

set -euo pipefail

fail() { printf 'error: %s\n' "$*" >&2; exit 1; }
step() { printf '==> %s\n' "$*"; }
run() {
  local fix="$1"
  shift
  "$@" || fail "Command '$*' did not succeed: $fix"
}
trap 'fail "Unexpected error at line $LINENO: inspect the command and its output, correct the cause, and check release progress before retrying."' ERR

usage() {
  cat <<'USAGE'
Usage:
  scripts/release.sh --marker '<css text new in this release>' [--dry-run] [--visual-ok]
  scripts/release.sh --help

First bump Cargo.toml, write the newest CHANGELOG entry, and commit.
--marker   Literal CSS text new in this release; must appear in the minified CSS.
--dry-run  Rebuild and run all checks, including cargo publish --dry-run;
           show git status without committing, publishing or pushing.
--visual-ok  You opened the screenshot report and every change in it is intended.
Publishing is irreversible. Normal mode commits assets, publishes, pushes all
four targets, and waits up to 15 minutes for two consecutive website matches.
USAGE
}

marker=''
dry_run=false
visual_ok=false
while [[ $# -gt 0 ]]; do
  case "$1" in
    --help) usage; exit 0 ;;
    --marker)
      [[ $# -ge 2 ]] || fail "--marker needs a value: pass --marker '<css text new in this release>'."
      marker="$2"; shift 2 ;;
    --dry-run) dry_run=true; shift ;;
    --visual-ok) visual_ok=true; shift ;;
    *) fail "Unknown argument '$1': use scripts/release.sh --help for usage." ;;
  esac
done

step '1. Preflight'
[[ -n "$marker" ]] || fail "Missing or empty --marker: pass --marker '<css text new in this release>'."
for command in cargo node bun jq curl git kapable-push-verify; do
  command -v "$command" >/dev/null 2>&1 || fail "Required command '$command' is missing: install it and put it on PATH."
done
cd "$(dirname "${BASH_SOURCE[0]}")/.." || fail 'Cannot enter the repository: restore scripts/release.sh inside the maud-ui checkout.'
ignored='docs/night-6-live-events.jsonl'
status=$(git status --porcelain --untracked-files=all) || fail 'Cannot read git status: repair the checkout and retry.'
dirty=''
while IFS= read -r line; do
  [[ -z "$line" || "${line:3}" == "$ignored" ]] && continue
  dirty="${dirty}${line}"$'\n'
done <<< "$status"
[[ -z "$dirty" ]] || fail "Working tree is dirty:
${dirty}Commit or remove the unrelated changes before releasing; never stage $ignored."
version=$(cargo metadata --no-deps --format-version 1 | jq -er '.packages[] | select(.name == "maud-ui") | .version') ||
  fail 'Cannot read the maud-ui version: fix Cargo.toml or the cargo metadata error and retry.'
registry_version() {
  curl -fsS -A maud-ui-release https://crates.io/api/v1/crates/maud-ui | jq -er .crate.max_version
}
published=$(registry_version) || fail 'Cannot read crates.io max_version: check network access and the crates.io API response, then retry.'

# Compare numeric components without sort -V (not portable to macOS).
# Prereleases follow SemVer precedence; build metadata does not affect ordering.
comparison=$(node - "$version" "$published" <<'JS'
const pattern = /^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?$/;
function parse(v) {
  const m = pattern.exec(v);
  if (!m) throw new Error(`Invalid semver: ${v}`);
  const pre = m[4] === undefined ? [] : m[4].split('.');
  if (pre.some(s => /^0[0-9]+$/.test(s))) throw new Error(`Invalid prerelease: ${v}`);
  return { core: v.split('-')[0].split('+')[0].split('.'), pre };
}
const numeric = (a, b) => a.length !== b.length ? Math.sign(a.length - b.length) : a === b ? 0 : a > b ? 1 : -1;
function compare(a, b) {
  for (let i = 0; i < 3; i++) { const n = numeric(a.core[i], b.core[i]); if (n) return n; }
  if (!a.pre.length || !b.pre.length) return Number(!a.pre.length) - Number(!b.pre.length);
  for (let i = 0; i < Math.max(a.pre.length, b.pre.length); i++) {
    if (i === a.pre.length) return -1;
    if (i === b.pre.length) return 1;
    const x = a.pre[i], y = b.pre[i];
    if (x === y) continue;
    const xn = /^[0-9]+$/.test(x), yn = /^[0-9]+$/.test(y);
    return xn && yn ? numeric(x, y) : xn !== yn ? (xn ? -1 : 1) : x > y ? 1 : -1;
  }
  return 0;
}
// stdout.write, not console.log: FORCE_COLOR (set on this Mac) makes console.log wrap a number
// in ANSI colour codes, and "\e[33m1\e[39m" != "1" refused every release (2026-09-23).
process.stdout.write(String(compare(parse(process.argv[2]), parse(process.argv[3]))) + '\n');
JS
) || fail 'Cannot compare release versions as semver: correct Cargo.toml or investigate the crates.io version response.'
[[ "$comparison" == 1 ]] || fail "Cargo.toml says $version but crates.io already has $published: bump the version in Cargo.toml and add a CHANGELOG entry, then commit."
heading=''
[[ -r CHANGELOG.md ]] || fail 'CHANGELOG.md is unreadable: restore it and add the release entry, then commit.'
while IFS= read -r line; do
  if [[ "$line" == '## ['* ]]; then heading="$line"; break; fi
done < CHANGELOG.md
[[ "$heading" == "## [$version]" || "$heading" == "## [$version] "* ]] ||
  fail "First CHANGELOG release heading is '$heading', expected ## [$version]: put this release entry first and commit."

step '2. Build bundles'
run 'Fix the asset build error and retry.' node examples/build-assets.mjs
step '3. Copy bundles to dist/'
for file in maud-ui.css maud-ui.min.css maud-ui.js maud-ui.min.js; do
  run 'Restore dist/ and fix the missing or unreadable static bundle, then retry.' cp "static/$file" "dist/$file"
done
step '4. Check the release stylesheet marker'
grep -F -- "$marker" static/maud-ui.min.css >/dev/null ||
  fail "The release's own stylesheet does not contain the marker (or cannot be read): either the marker is wrong or the new CSS rule did not reach the bundle; correct the marker or asset build."
step '5. Regenerate the website'
run 'Fix the static export error so public/ is regenerated, then retry.' bun run build:static
step '5b. Compare screenshots with the live site'
# Serve public/ exactly as the site will, on a free port, and diff it against the live site.
visual_port=$(node -e "const s=require('net').createServer().listen(0,'127.0.0.1',()=>{process.stdout.write(String(s.address().port));s.close()})") ||
  fail 'Cannot find a free local port for the screenshot check: free a port and retry.'
PORT="$visual_port" bun server.ts >/dev/null 2>&1 &
visual_server=$!
trap 'kill "$visual_server" 2>/dev/null || true' EXIT
ready=false
for _ in $(seq 1 60); do
  if curl -fsS -o /dev/null "http://127.0.0.1:$visual_port/"; then ready=true; break; fi
  sleep 0.5
done
[[ "$ready" == true ]] || fail "The exported site did not start on port $visual_port within 30 s: run 'PORT=$visual_port bun server.ts' by hand to see why."
visual_dir=$(mktemp -d "${TMPDIR:-/tmp}/maud-ui-release-visual.XXXXXX") || fail 'Cannot create a screenshot folder: fix temporary-directory permissions or free disk space.'
visual_status=0
node scripts/visual-check.mjs --candidate "http://127.0.0.1:$visual_port" --out "$visual_dir" || visual_status=$?
kill "$visual_server" 2>/dev/null || true
case "$visual_status" in
  0) ;;
  1) [[ "$visual_ok" == true ]] ||
       fail "Pages look different from the live site: open $visual_dir/index.html. If every change is intended, re-run with --visual-ok; otherwise fix the regression." ;;
  *) fail "The screenshot check could not run (status $visual_status): read its ERROR lines above; report folder $visual_dir." ;;
esac
step '6. Run tests'
test_log=$(mktemp "${TMPDIR:-/tmp}/maud-ui-release-tests.XXXXXX") || fail 'Cannot create a test log: fix temporary-directory permissions or free disk space.'
test_status=0
cargo test --no-fail-fast >"$test_log" 2>&1 || test_status=$?
failed_lines=$(grep -E '^test .* FAILED$' "$test_log") || {
  grep_status=$?
  [[ "$grep_status" == 1 ]] || fail "Cannot inspect test log $test_log: fix log access before releasing."
}
if [[ "$test_status" != 0 || -n "$failed_lines" ]]; then
  [[ -z "$failed_lines" ]] || printf '%s\n' "$failed_lines" >&2
  fail "Tests returned status $test_status or reported FAILED lines; log: $test_log. Fix the test errors before releasing."
fi
step '7. Run clippy'
run 'Fix all clippy diagnostics before releasing.' cargo clippy --all-targets -- -D warnings
step '8. Check the publish package'
run 'Fix package validation or registry authentication before releasing.' cargo publish --dry-run --allow-dirty
if [[ "$dry_run" == true ]]; then
  step '9. Show dry-run changes'
  run 'Repair the checkout so generated changes can be inspected.' git status --short
  printf 'Dry run complete: nothing was committed, published or pushed.\n'
  exit 0
fi

step '10. Stage and commit regenerated assets'
run "Fix git index permissions; never stage $ignored." git add -A -- . ':!docs/night-6-live-events.jsonl'
staged=$(git diff --cached --name-only) || fail 'Cannot inspect staged files: repair the git index before committing.'
while IFS= read -r file; do
  [[ "$file" != "$ignored" ]] || fail "$ignored is staged: unstage that path without changing its contents, then review the index before retrying."
done <<< "$staged"
if [[ -n "$staged" ]]; then
  run 'Fix git identity, hooks or index errors, then review the generated assets before retrying.' git commit -m "chore(release): regenerate assets for $version"
fi
step '11. Publish the crate'
run 'Inspect cargo output and fix authentication or package errors; check crates.io before retrying because publishing is irreversible.' cargo publish
step '12. Push release to all four targets'
run 'The crate is published; fix customer remote access or divergence and retry this push without force.' git push customer HEAD:main
run 'The crate is published; fix customer remote access or divergence and retry this push without force.' git push customer HEAD
run 'The crate is published; fix github remote access or divergence and retry this push without force.' git push github HEAD:master
run 'The crate is published; fix forgejo remote access or divergence and retry this push without force.' git push forgejo HEAD:master
step '13. Verify the deploy fired'
head=$(git rev-parse HEAD) || fail 'The crate is published and pushes completed, but HEAD is unreadable: repair the checkout and verify the deployment.'
run 'The crate is published and pushes completed; inspect the customer deploy and webhook, then rerun kapable-push-verify.' kapable-push-verify customer-herman-engineer/maud-ui "$head"
step '14. Wait for two consecutive website marker matches'
deadline=$((SECONDS + 900))
consecutive=0
while (( SECONDS < deadline )); do
  remaining=$((deadline - SECONDS))
  sample_timeout=20
  (( remaining >= sample_timeout )) || sample_timeout=$remaining
  sample_started=$SECONDS
  if css=$(curl -fsS --max-time "$sample_timeout" https://maudui.herman.engineer/css/maud-ui.min.css) &&
      grep -F -- "$marker" <<< "$css" >/dev/null; then
    consecutive=$((consecutive + 1))
    if (( consecutive == 2 && SECONDS <= deadline )); then break; fi
  else
    consecutive=0
  fi
  remaining=$((deadline - SECONDS))
  (( remaining > 0 )) || break
  pause=$((20 - (SECONDS - sample_started)))
  (( pause <= remaining )) || pause=$remaining
  if (( pause > 0 )); then sleep "$pause"; fi
done
(( consecutive == 2 && SECONDS <= deadline )) ||
  fail 'The push landed and the crate is published, but the site does not serve the marker yet in two consecutive samples: check that public/ was regenerated and committed, then inspect the deployment.'
step '15. Confirm crates.io version'
published=$(registry_version) || fail 'The crate was published and the site verified, but crates.io could not be read: check the API/network and confirm max_version manually.'
[[ "$published" == "$version" ]] || fail "crates.io max_version is $published, expected $version: check registry propagation and the published crate; do not republish an existing version."
printf 'Released maud-ui %s (%s): crate confirmed, four pushes complete, website marker verified twice.\n' "$version" "$head"
