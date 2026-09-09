# App footer

Optional help/legal links, a version or copyright line, secondary columns and arbitrary content. The footer sits below the entire sidebar/content row, with quiet small text and safe-area padding.

## Import and example
```rust
use maud_ui::blocks::{action::Link, shell::{app_footer, sidebar}};
sidebar::render(sidebar::Props {
    app_footer: app_footer::render(app_footer::Props {
        line: Some("Garden House · Workspace 0.9".into()),
        links: vec![Link { label: "Help".into(), href: "/help".into() }],
        ..Default::default()
    }),
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| links | Vec<action::Link> | empty | Footer navigation links: label/href strings. |
| line | Option<String> | None | Escaped copyright, product or version line. |
| columns | Vec<Column> | empty | Column has title: String and links: Vec<action::Link>. |
| children | Markup | empty | Arbitrary caller-rendered content after the links. |

Default props render nothing. An empty line and empty child markup also emit no footer. Secondary columns use a responsive grid; links wrap as whole items. There is no script requirement. See the sidebar showcase for the full header/footer composition in light and dark.
