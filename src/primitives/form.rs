//! Form component — the `<form>` element itself.
//!
//! No primitive emitted a `<form>` before this: every form in `src/blocks/**`
//! hand-wrote the tag, so the submission contract (method, encoding,
//! validation) was retyped at each site and could drift between them.
//!
//! `form` is deliberately **semantic only** — it applies no layout. A form is
//! almost always a column of fields, and [`stack`](super::stack) already is
//! that column; making `form` a second flex container would mean two ways to
//! express one thing. Compose them: `form` carries the submission contract,
//! `stack` carries the rhythm.

use maud::{html, Markup};

/// HTTP method used to submit the form.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Method {
    /// `GET` — field values are appended to the URL as a query string. Correct
    /// for searches and filters, whose results should be linkable and
    /// bookmarkable.
    Get,
    /// `POST` — field values travel in the request body. The default here.
    #[default]
    Post,
    /// `dialog` — closes the surrounding `<dialog>` and submits nothing over
    /// the network. Only meaningful inside [`dialog`](super::dialog).
    Dialog,
}

impl Method {
    /// The `method` attribute value.
    pub fn as_attr(self) -> &'static str {
        match self {
            Method::Get => "get",
            Method::Post => "post",
            Method::Dialog => "dialog",
        }
    }
}

/// How the form body is encoded on submit. Only meaningful for [`Method::Post`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Enctype {
    /// `application/x-www-form-urlencoded` — the HTML default; the attribute is
    /// omitted for this variant.
    #[default]
    UrlEncoded,
    /// `multipart/form-data` — **required** for any form containing a file
    /// input. Without it the browser submits the filename and not the file.
    Multipart,
    /// `text/plain` — for debugging only; it does not escape reliably.
    TextPlain,
}

impl Enctype {
    /// The `enctype` attribute value, or `None` for the HTML default.
    pub fn as_attr(self) -> Option<&'static str> {
        match self {
            Enctype::UrlEncoded => None,
            Enctype::Multipart => Some("multipart/form-data"),
            Enctype::TextPlain => Some("text/plain"),
        }
    }
}

/// Form rendering properties.
#[derive(Debug, Clone)]
pub struct Props {
    /// Submission target. `None` omits the attribute, which submits to the
    /// current URL — valid HTML and the common case for a page that posts to
    /// itself.
    pub action: Option<String>,
    /// HTTP method. Default [`Method::Post`] — **this diverges from HTML**, see
    /// [`render`].
    pub method: Method,
    /// Body encoding. Default [`Enctype::UrlEncoded`] (attribute omitted).
    pub enctype: Enctype,
    /// Optional `id`, for `aria-labelledby`, label targeting, and submit
    /// buttons placed outside the form via `form="<id>"`.
    pub id: Option<String>,
    /// Accessible name. A `<form>` is only exposed as a `form` landmark when it
    /// has one — see [`render`].
    pub aria_label: Option<String>,
    /// Skip the browser's built-in constraint validation on submit.
    /// Default `false`.
    pub novalidate: bool,
    /// Allow the browser to autofill fields. Default `true`; `false` emits
    /// `autocomplete="off"`.
    pub autocomplete: bool,
    /// The form's contents — fields, and the submit control.
    pub children: Markup,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            action: None,
            method: Method::default(),
            enctype: Enctype::default(),
            id: None,
            aria_label: None,
            novalidate: false,
            autocomplete: true,
            children: html! {},
        }
    }
}

/// Render a `<form>` with the given properties.
///
/// **`method` defaults to `Post`, not HTML's `GET`.** The divergence is
/// deliberate: `GET` serialises every field into the URL, where it lands in
/// browser history, server logs, and the `Referer` header sent to third
/// parties. A login form whose author forgot to set `method` should not leak
/// the password that way. Set [`Method::Get`] explicitly for searches and
/// filters, where a linkable result URL is the point.
///
/// Accessibility: a `<form>` is exposed as a `form` landmark **only** when it
/// has an accessible name, so pass `aria_label` on any page carrying more than
/// one form. A single unnamed form is fine — it simply is not a landmark, which
/// is the correct outcome when there is nothing to distinguish it from.
pub fn render(props: Props) -> Markup {
    html! {
        form class="mui-form"
             action=[props.action.as_deref()]
             method=(props.method.as_attr())
             enctype=[props.enctype.as_attr()]
             id=[props.id.as_deref()]
             aria-label=[props.aria_label.as_deref()]
             novalidate[props.novalidate]
             autocomplete=[if props.autocomplete { None } else { Some("off") }] {
            (props.children)
        }
    }
}

/// A `POST` form wrapping its children in a vertical [`stack`](super::stack) —
/// the shape almost every form takes, in one call.
pub fn stacked(action: impl Into<String>, children: Markup) -> Markup {
    use super::stack;
    render(Props {
        action: Some(action.into()),
        children: stack::render(stack::Props {
            gap: stack::Space::Lg,
            children,
            ..Default::default()
        }),
        ..Default::default()
    })
}

/// Showcase all form use cases.
pub fn showcase() -> Markup {
    use crate::primitives::{button, checkbox, field, input, label, select, stack, textarea};

    html! {
        div.mui-showcase__grid {
            // 1. The common case
            section {
                h2 { "Stacked POST form" }
                p.mui-showcase__caption {
                    "form::stacked(\u{2026}) \u{2014} the submission contract from form, the rhythm from stack."
                }
                // .mui-field self-caps at 24rem (a readable field measure), so the
                // form is capped to match — otherwise the button row runs to the
                // full container width while the fields stop short of it.
                div style="max-width:24rem;" {
                (stacked("/account/profile", html! {
                    div class="mui-field" {
                        (label::render(label::Props {
                            text: "Display name".into(),
                            html_for: Some("form-name".into()),
                            ..Default::default()
                        }))
                        (input::render(input::Props {
                            name: "display_name".into(),
                            id: "form-name".into(),
                            placeholder: "Ada Lovelace".into(),
                            ..Default::default()
                        }))
                    }
                    div class="mui-field" {
                        (label::render(label::Props {
                            text: "Bio".into(),
                            html_for: Some("form-bio".into()),
                            ..Default::default()
                        }))
                        (textarea::render(textarea::Props {
                            name: "bio".into(),
                            id: "form-bio".into(),
                            placeholder: "A short introduction\u{2026}".into(),
                            ..Default::default()
                        }))
                    }
                    (stack::render(stack::Props {
                        direction: stack::Direction::Horizontal,
                        justify: stack::Justify::End,
                        gap: stack::Space::Sm,
                        children: html! {
                            (button::render(button::Props {
                                label: "Cancel".into(),
                                variant: button::Variant::Outline,
                                ..Default::default()
                            }))
                            (button::render(button::Props {
                                label: "Save profile".into(),
                                variant: button::Variant::Primary,
                                button_type: "submit",
                                ..Default::default()
                            }))
                        },
                        ..Default::default()
                    }))
                }))
                }
            }

            // 2. GET — a search form
            section {
                h2 { "Method::Get \u{2014} a search form" }
                p.mui-showcase__caption {
                    "GET is right when the result URL should be linkable and bookmarkable. It is wrong for anything secret: values land in history, logs, and the Referer header."
                }
                (render(Props {
                    action: Some("/search".into()),
                    method: Method::Get,
                    aria_label: Some("Search the catalogue".into()),
                    children: stack::render(stack::Props {
                        direction: stack::Direction::Horizontal,
                        gap: stack::Space::Sm,
                        align: stack::Align::End,
                        children: html! {
                            div style="flex:1;" {
                                (input::render(input::Props {
                                    name: "q".into(),
                                    id: "form-q".into(),
                                    input_type: input::InputType::Search,
                                    placeholder: "Search\u{2026}".into(),
                                    ..Default::default()
                                }))
                            }
                            (select::render(select::Props {
                                name: "sort".into(),
                                id: "form-sort".into(),
                                options: vec![
                                    select::SelectOption { value: "rel".into(), label: "Relevance".into(), disabled: false },
                                    select::SelectOption { value: "new".into(), label: "Newest".into(), disabled: false },
                                ],
                                ..Default::default()
                            }))
                            (button::render(button::Props {
                                label: "Search".into(),
                                variant: button::Variant::Primary,
                                button_type: "submit",
                                ..Default::default()
                            }))
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }))
            }

            // 3. Multipart — file upload
            section {
                h2 { "Enctype::Multipart \u{2014} file upload" }
                p.mui-showcase__caption {
                    "Required for any form with a file input. Without it the browser submits the filename and not the file."
                }
                (render(Props {
                    action: Some("/avatar".into()),
                    enctype: Enctype::Multipart,
                    aria_label: Some("Upload avatar".into()),
                    children: stack::render(stack::Props {
                        gap: stack::Space::Md,
                        align: stack::Align::Start,
                        children: html! {
                            (field::render(field::Props {
                                label: "Avatar".into(),
                                id: "form-avatar".into(),
                                description: Some("PNG or JPEG, up to 2 MB.".into()),
                                children: html! {
                                    (input::render(input::Props {
                                        name: "avatar".into(),
                                        id: "form-avatar".into(),
                                        input_type: input::InputType::File,
                                        ..Default::default()
                                    }))
                                },
                                ..Default::default()
                            }))
                            (button::render(button::Props {
                                label: "Upload".into(),
                                variant: button::Variant::Primary,
                                button_type: "submit",
                                ..Default::default()
                            }))
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }))
            }

            // 4. novalidate + autocomplete off
            section {
                h2 { "novalidate + autocomplete: false" }
                p.mui-showcase__caption {
                    "Skip the browser's constraint validation (when the server owns the rules), and opt out of autofill."
                }
                // Capped to the same 24rem measure as the other examples: `input`
                // sets width:100%, which wins over align: Start, so an uncapped
                // stack gives a full-bleed field above pocket-sized controls.
                div style="max-width:24rem;" {
                (render(Props {
                    action: Some("/admin/impersonate".into()),
                    novalidate: true,
                    autocomplete: false,
                    aria_label: Some("Impersonate a user".into()),
                    children: stack::render(stack::Props {
                        gap: stack::Space::Md,
                        align: stack::Align::Start,
                        children: html! {
                            (input::render(input::Props {
                                name: "email".into(),
                                id: "form-impersonate".into(),
                                input_type: input::InputType::Email,
                                placeholder: "user@example.com".into(),
                                ..Default::default()
                            }))
                            (checkbox::render(checkbox::Props {
                                name: "confirm".into(),
                                id: "form-confirm".into(),
                                label: "I have an approved ticket for this".into(),
                                ..Default::default()
                            }))
                            (button::render(button::Props {
                                label: "Impersonate".into(),
                                variant: button::Variant::Danger,
                                button_type: "submit",
                                ..Default::default()
                            }))
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }))
                }
            }

            // 5. Method::Dialog
            section {
                h2 { "Method::Dialog" }
                p.mui-showcase__caption {
                    "Inside a <dialog>, method=\"dialog\" closes it and submits nothing over the network."
                }
                (render(Props {
                    method: Method::Dialog,
                    aria_label: Some("Confirm".into()),
                    children: stack::render(stack::Props {
                        direction: stack::Direction::Horizontal,
                        gap: stack::Space::Sm,
                        children: html! {
                            (button::render(button::Props {
                                label: "Dismiss".into(),
                                variant: button::Variant::Outline,
                                button_type: "submit",
                                ..Default::default()
                            }))
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }))
            }
        }
    }
}
