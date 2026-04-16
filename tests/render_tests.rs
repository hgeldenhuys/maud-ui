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
    alert,
    alert_dialog,
    aspect_ratio,
    avatar,
    badge,
    breadcrumb,
    button,
    button_group,
    card,
    checkbox,
    collapsible,
    context_menu,
    dialog,
    drawer,
    empty_state,
    field,
    fieldset,
    hover_card,
    input,
    input_group,
    input_otp,
    kbd,
    label,
    menu,
    meter,
    native_select,
    number_field,
    pagination,
    popover,
    progress,
    radio,
    radio_group,
    scroll_area,
    select,
    separator,
    skeleton,
    slider,
    spinner,
    switch,
    table,
    tabs,
    textarea,
    toast,
    toggle,
    toggle_group,
    tooltip,
    typography,
);
