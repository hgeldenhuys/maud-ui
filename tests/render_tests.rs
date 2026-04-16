//! Snapshot tests for component HTML output. Expanded in Wave 4.
//!
//! For now, assert that every component's `showcase()` function compiles and
//! returns non-empty markup. This catches API-level regressions during parallel
//! wave integration.

macro_rules! assert_showcase_renders {
    ($($module:ident),* $(,)?) => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $module() {
                let markup = maud_ui::primitives::$module::showcase();
                let rendered = markup.into_string();
                assert!(!rendered.is_empty(), "showcase() returned empty markup");
            }
        )*
    };
}

assert_showcase_renders!(
    accordion,
    alert_dialog,
    avatar,
    badge,
    button,
    checkbox,
    collapsible,
    context_menu,
    dialog,
    field,
    fieldset,
    input,
    menu,
    meter,
    number_field,
    popover,
    progress,
    radio,
    scroll_area,
    select,
    separator,
    slider,
    switch,
    tabs,
    textarea,
    toast,
    toggle,
    toggle_group,
    tooltip,
);
