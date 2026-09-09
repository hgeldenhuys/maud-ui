# Record page

The generic detail archetype for every record type: title, one muted kind/reference line, action row, then content with a guaranteed library gap. The default header uses H1; the block adds no main landmark because the surrounding shell owns it.

## Import and example

```rust
use maud::html;
use maud_ui::blocks::{action::{Action, Heading}, record::{facts, header, page, related_list}};
page::render(page::Props {
    header: header::Props {
        title: "Sofia Patel".into(), kind: Some("Guest".into()), reference: Some("UI-G5".into()),
        heading: Heading::H1,
        primary_action: Some(Action::link("Edit guest", "/ui/guests/UI-G5/edit")),
        secondary_actions: vec![Action::link("Graph view", "/ui/guests/UI-G5/graph")],
        ..Default::default()
    },
    children: html! {
        (facts::render(facts::Props {
            groups: vec![facts::Group { title: "Identity".into(), facts: vec![
                facts::Fact::new("Full name", "Sofia Patel"),
                facts::Fact::new("Email", "sofia.patel@example.com"),
            ] }], ..Default::default()
        }))
        (related_list::render(related_list::Props {
            title: "Bookings".into(), empty_message: "No bookings yet.".into(), ..Default::default()
        }))
    },
    ..Default::default()
});
```

## Props and action hierarchy

`header: record::header::Props` carries identity and the main verb. `children: Markup` contains admitted blocks. `state: State` supports the shared presentation states; Absent renders nothing, while Loading/Empty/Error replace the record and Disabled keeps it inert with an external reason.

Use one main verb, such as Edit guest, Check in or Approve. Developer views belong in `secondary_actions`, behind native `More ▾`. Back to list belongs in the shell breadcrumb, preserving any collection filters in its href. Do not duplicate it in the action row. The library cannot infer an application's main verb, role access or translated schema labels; that mapping remains in the generic adapter.

`kind` and `reference` render one subtitle (`Guest · UI-G5`), taking precedence over the older `subtitle` prop. Do not add a second “Guest Profile View” subtitle. Existing canonical title/status markup slots retain precedence. When providing your own `header` literal, set its heading to H1; use H2/H3 for embedded records.

## One stack, sections versus cards

`.mui-record-page` is a `.mui-stack`: vertical flex, `gap: var(--mui-stack-gap)` (16px), with child block margins reset. Shell content, card bodies, record blocks, grouped worklists and archetype examples use the same stack contract. `.mui-page-stack` remains compatible. `stack::vertical` and bare `.mui-stack` use this rhythm too.

Group facts about the **same record** into one [Record facts](record-facts.md) surface. Independent tasks/entities may use separate cards. Wrap those cards in `stack::vertical` or `.mui-stack`; never add sibling margins as well. Arbitrary caller-owned wrappers must also adopt the library stack: the library does not rewrite unknown application DOM. Explicit stack gap modifiers are intentional compact-layout overrides.

## Accessibility and verification

Reading and keyboard order follow title → subtitle → main verb → More → content. Native links, forms, definition lists and lists retain their semantics. No new JavaScript is needed for this page or the More disclosure. Standard theme, density, touch-target and reduced-motion contracts continue to apply.

`examples/record_page_fixture.rs` generates six guest/long-label pages over three densities plus a three-card stack and a minimal record. The gallery serves `/fixtures/record-page-{compact,comfortable,spacious}-{guest,long}.html`, `/fixtures/record-page-three-cards.html` and `/fixtures/record-page-minimal.html`. Use 1280×900 and 390×900 in light/dark; source/DOM tests are distinct from supervisor browser measurements.
