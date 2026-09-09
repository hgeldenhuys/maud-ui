# Related records

Use `record::related_list` for a record's bookings, members, notes or other related/composed collections. Each item has one compact identity link, an optional status chip and a date. It is a list, not a stack of nested cards or disclosure panels.

## Import and example

```rust
use maud_ui::{blocks::record::related_list::{self, Date, Item}, primitives::badge};
related_list::render(related_list::Props {
    title: "Bookings".into(),
    items: vec![Item {
        reference: "UI-B05".into(), title: "Garden suite".into(),
        href: Some("/ui/booking/UI-B05".into()),
        status: Some(badge::Props { label: "Confirmed".into(), variant: badge::Variant::Success, ..Default::default() }),
        date: Some(Date { label: "8 Sep 2026".into(), datetime: Some("2026-09-08".into()) }),
    }],
    empty_message: "No bookings yet.".into(),
    ..Default::default()
});
```

## Props and copy contract

`title` defaults to `Related records`; use a human-facing plural such as `Bookings`. `items: Vec<Item>` stays in caller order. `empty_message` defaults to `No related records.` and renders as one quiet line under the heading, without an empty card or placeholder rows. Items with neither reference nor title are omitted. `heading` defaults to H2. `state` supports the shared Ready/Absent/Loading/Empty/Error/Disabled contract.

An `Item` has `reference`, `title`, optional `href`, optional `badge::Props` in `status`, and optional `Date { label, datetime }`. A missing or blank href renders plain identity text, without a dead link. Date `label` is formatted for the audience; supply a valid machine-readable `datetime` if known. Missing/blank dates leave no placeholder column.

Reference and title appear once, separated by a middle dot when both remain. Repeated leading copies of the same reference are removed from the title, with or without `#`: reference `UI-B05`, title `#UI-B05 #UI-B05` renders only `UI-B05`. Exact boundaries protect different references such as `UI-B050`. Distinct title text is preserved; the component does not rewrite arbitrary prose.

The adapter must map schema names to product labels. Never pass `COMPOSITION`, `1 nested`, `via guest`, relation names, join paths or diagnostics as user-facing titles. There are deliberately no schema/count/path subtitle slots. Business status and dates come from admitted values, never inferred from the relationship.

## Layout and accessibility

Semantic `ul`/`li` gives one row per record. Links use `--mui-link` in visited and normal states, underline and visible keyboard focus. Text in the status chip supplies meaning beyond color. The reference stays visible, the human title may ellipsize, and the link's full title remains available. Date and status stay alongside the identity; touch identity links meet the existing 44px target. Empty content is quiet text, not an alert.

Use [Related card](record-related-card.md) for a **single** entity with additional facts and an action. Use this list for collections. Pair with [Record facts](record-facts.md) and [Record page](record-page.md).
