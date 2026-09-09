//! A shared logo, wordmark and optional tagline for application chrome.
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct Props {
    /// Plain text identity. Empty identity emits nothing.
    pub wordmark: String,
    /// Optional destination for the wordmark; the tagline remains outside the link.
    pub href: Option<String>,
    /// Full-color image or custom logo. None uses the brand's CSS logo mask.
    pub logo: Option<Markup>,
    /// Optional caller-owned tagline slot.
    pub tagline: Option<Markup>,
}
pub fn render(props: Props) -> Markup {
    if props.wordmark.trim().is_empty() {
        return html! {};
    }
    html! {
        div class="mui-brand-mark" {
            span class=(if props.logo.is_some() { "mui-brand-mark__logo" } else { "mui-brand-mark__logo mui-brand-mark__logo--mask" }) aria-hidden="true" {
                @if let Some(logo) = props.logo { (logo) }
            }
            div class="mui-brand-mark__text" {
                @if let Some(href) = props.href { a class="mui-brand-mark__wordmark" href=(href) { (props.wordmark) } }
                @else { span class="mui-brand-mark__wordmark" { (props.wordmark) } }
                @if let Some(tagline) = props.tagline { div class="mui-brand-mark__tagline" { (tagline) } }
            }
        }
    }
}
pub fn preset(key: &str) -> Option<Props> {
    crate::brand::BRANDS
        .iter()
        .find(|b| b.key == key)
        .map(|brand| Props {
            wordmark: brand.wordmark.into(),
            tagline: Some(html! { (brand.tagline) }),
            ..Default::default()
        })
}
pub fn preview() -> Markup {
    crate::blocks::kit_preview(|_| {
        html! {
            @for brand in crate::brand::BRANDS {
                div class="mui-brand-example" data-brand=(brand.key) { (render(preset(brand.key).unwrap())) }
            }
            p class="mui-caption" { "Switch the whole interface and export your brand from the theme customiser." }
            a class="mui-btn mui-btn--outline" href="/theme#brand" { "Try the three brands" }
        }
    })
}
