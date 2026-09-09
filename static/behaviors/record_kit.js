(function () {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = new WeakSet();
  // Calendar ordinals avoid local timezone, DST and Date's special handling of years 0–99.
  function ordinal(value) {
    if (!/^\d{4}-\d{2}-\d{2}$/.test(value)) return null;
    const [year, month, day] = value.split('-').map(Number);
    if (!year || month < 1 || month > 12) return null;
    const leap = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
    const months = [31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if (!day || day > months[month - 1]) return null;
    const previous = year - 1;
    return previous * 365 + Math.floor(previous / 4) - Math.floor(previous / 100) + Math.floor(previous / 400) + months.slice(0, month - 1).reduce((a, b) => a + b, 0) + day;
  }
  function bounded(input, value) {
    const min = ordinal(input.min || ''), max = ordinal(input.max || '');
    return value !== null && (min === null || value >= min) && (max === null || value <= max);
  }
  ui.behaviors['date-range'] = range => {
    if (initialized.has(range)) return;
    const start = range.querySelector('[data-range-field="start"]');
    const end = range.querySelector('[data-range-field="end"]');
    const output = range.querySelector('output');
    if (!start || !end || !output) return;
    initialized.add(range);
    function update() {
      const first = ordinal(start.value), last = ordinal(end.value);
      const incomplete = !start.value || !end.value;
      const invalid = !incomplete && (!bounded(start, first) || !bounded(end, last) || last <= first);
      const count = last - first;
      output.textContent = incomplete ? range.getAttribute('data-incomplete')
        : invalid ? range.getAttribute('data-invalid')
        : count + ' ' + range.getAttribute(count === 1 ? 'data-night' : 'data-nights');
      output.setAttribute('data-range-invalid', String(invalid));
      for (const input of [start, end]) {
        if (invalid) input.setAttribute('aria-invalid', 'true'); else input.removeAttribute('aria-invalid');
      }
      // Native required/min/max still work alone. This adds cross-field interval validation.
      end.setCustomValidity(invalid && !range.disabled && !end.disabled && !end.readOnly ? range.getAttribute('data-invalid') : '');
    }
    range.addEventListener('input', update);
    range.addEventListener('change', update);
    // Listen once per owner document, without retaining a form that HTMX may replace.
    update();
    range.__muiUpdateDateRange = update;
  };
  if (!window.__muiRecordKitResetBound) {
    window.__muiRecordKitResetBound = true;
    document.addEventListener('reset', event => {
      // Reset's default action restores values after the event. A cancelled reset changes nothing.
      setTimeout(() => {
        if (event.defaultPrevented) return;
        document.querySelectorAll('[data-mui="date-range"]').forEach(range => {
          if (range.querySelector('[data-range-field="start"]')?.form === event.target || range.querySelector('[data-range-field="end"]')?.form === event.target) range.__muiUpdateDateRange?.();
        });
      }, 0);
    });
  }
  ui.behaviors['attention-banner'] = banner => {
    if (initialized.has(banner)) return;
    const dismiss = banner.querySelector('.mui-attention-banner__dismiss');
    if (!dismiss) return;
    initialized.add(banner);
    dismiss.hidden = false;
    dismiss.addEventListener('click', () => {
      const controls = Array.from(document.querySelectorAll('a[href], button, input, select, textarea, summary, [tabindex]'));
      const usable = node => node && !banner.contains(node) && !node.disabled && node.getAttribute('aria-disabled') !== 'true' && node.getClientRects().length;
      const tabbable = node => usable(node) && (node.getAttribute('tabindex') === null || Number(node.getAttribute('tabindex')) >= 0);
      const explicit = document.getElementById(dismiss.getAttribute('data-dismiss-focus'));
      const index = controls.indexOf(dismiss);
      const target = usable(explicit) ? explicit : controls.slice(index + 1).find(tabbable) || controls.slice(0, Math.max(0, index)).reverse().find(tabbable);
      banner.hidden = true;
      if (target) target.focus();
      else {
        // With no adjacent control, retain a meaningful focus position on the parent region.
        const parent = banner.parentElement;
        if (parent) {
          const previous = parent.getAttribute('tabindex');
          parent.setAttribute('tabindex', '-1'); parent.focus();
          if (previous === null) parent.removeAttribute('tabindex'); else parent.setAttribute('tabindex', previous);
        }
      }
    });
  };
  ui.init();
})();
