//! `settings::billing` — current plan summary + payment method + invoice history.
//!
//! Composes: `card`, `badge`, `button`, `table`.
//!
//! Three stacked sections:
//!   1. Current plan — big card with plan name, price, usage, upgrade CTA
//!   2. Payment method — card brand + last 4 + expiry
//!   3. Invoices — table of past invoices with download links

use maud::{html, Markup};

use crate::primitives::{badge, button, card, table};

#[derive(Clone, Debug, Default)]
pub struct Props {
    pub plan: PlanSummary,
    pub payment_method: Option<PaymentMethod>,
    pub invoices: Vec<Invoice>,

    /// POST URL for "Change plan" button.
    pub change_plan_action: Option<String>,

    /// POST URL for "Cancel plan" (rendered as a subtle link). `None`
    /// hides it.
    pub cancel_plan_action: Option<String>,

    /// URL for the "Update payment method" form.
    pub update_payment_action: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct PlanSummary {
    pub name: String,
    pub price: String,
    pub billing_period: String,
    pub next_charge_date: String,
    pub seats_used: u32,
    pub seats_total: u32,
}

#[derive(Clone, Debug)]
pub struct PaymentMethod {
    /// Short label like "Visa", "Mastercard", "Amex".
    pub brand: String,
    /// Last four digits.
    pub last4: String,
    /// "09/2027".
    pub expiry: String,
}

#[derive(Clone, Debug)]
pub struct Invoice {
    pub date: String,
    pub number: String,
    pub amount: String,
    pub status: InvoiceStatus,
    pub download_url: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvoiceStatus {
    Paid,
    Pending,
    Failed,
}

pub fn render(props: Props) -> Markup {
    let seat_pct = if props.plan.seats_total > 0 {
        (props.plan.seats_used as f64 / props.plan.seats_total as f64 * 100.0).round() as u32
    } else {
        0
    };

    html! {
        div class="mui-block mui-block--settings" {
            // Current plan
            (card::render(card::Props {
                title: Some("Current plan".into()),
                description: Some(format!("Billed {}", props.plan.billing_period)),
                children: html! {
                    div class="mui-block--billing__plan" {
                        div class="mui-block--billing__plan-name" {
                            h3 { (props.plan.name) }
                            span class="mui-block--billing__plan-price" {
                                (props.plan.price) " " span class="mui-block--billing__plan-period" {
                                    "/ " (props.plan.billing_period.to_lowercase())
                                }
                            }
                        }

                        div class="mui-block--billing__meta" {
                            div class="mui-block--billing__meta-item" {
                                span class="mui-block--billing__meta-label" { "Next charge" }
                                span class="mui-block--billing__meta-value" { (props.plan.next_charge_date) }
                            }
                            div class="mui-block--billing__meta-item" {
                                span class="mui-block--billing__meta-label" { "Seats used" }
                                span class="mui-block--billing__meta-value" {
                                    (props.plan.seats_used) " / " (props.plan.seats_total)
                                }
                                div class="mui-block--billing__meta-bar" {
                                    div class="mui-block--billing__meta-bar-fill"
                                        style=(format!("width: {}%;", seat_pct)) {}
                                }
                            }
                        }

                        div class="mui-block--billing__actions" {
                            @if let Some(url) = &props.change_plan_action {
                                form action=(url) method="post" {
                                    (button::render(button::Props {
                                        label: "Change plan".into(),
                                        variant: button::Variant::Primary,
                                        size: button::Size::Md,
                                        button_type: "submit",
                                        ..Default::default()
                                    }))
                                }
                            }
                            @if let Some(url) = &props.cancel_plan_action {
                                form action=(url) method="post" {
                                    (button::render(button::Props {
                                        label: "Cancel plan".into(),
                                        variant: button::Variant::Ghost,
                                        size: button::Size::Md,
                                        button_type: "submit",
                                        ..Default::default()
                                    }))
                                }
                            }
                        }
                    }
                },
                ..Default::default()
            }))

            // Payment method
            @if let Some(pm) = &props.payment_method {
                (card::render(card::Props {
                    title: Some("Payment method".into()),
                    children: html! {
                        div class="mui-block--billing__payment" {
                            div class="mui-block--billing__card" {
                                span class="mui-block--billing__card-brand" { (pm.brand) }
                                span class="mui-block--billing__card-number" {
                                    "\u{2022}\u{2022}\u{2022}\u{2022} "
                                    "\u{2022}\u{2022}\u{2022}\u{2022} "
                                    "\u{2022}\u{2022}\u{2022}\u{2022} "
                                    (pm.last4)
                                }
                                span class="mui-block--billing__card-expiry" {
                                    "Expires " (pm.expiry)
                                }
                            }
                            @if let Some(url) = &props.update_payment_action {
                                form action=(url) method="post"
                                     class="mui-block--billing__payment-update" {
                                    (button::render(button::Props {
                                        label: "Update".into(),
                                        variant: button::Variant::Outline,
                                        size: button::Size::Sm,
                                        button_type: "submit",
                                        ..Default::default()
                                    }))
                                }
                            }
                        }
                    },
                    ..Default::default()
                }))
            }

            // Invoices
            @if !props.invoices.is_empty() {
                (card::render(card::Props {
                    title: Some("Invoices".into()),
                    description: Some(format!("Your last {} invoices.", props.invoices.len())),
                    children: html! {
                        (table::render(table::Props {
                            headers: vec!["Date".into(), "Number".into(), "Amount".into(), "Status".into(), String::new()],
                            rows: Vec::new(),
                            rich_rows: props.invoices.iter().map(|inv| {
                                let status = match inv.status {
                                    InvoiceStatus::Paid => badge::render(badge::Props {
                                        label: "Paid".into(),
                                        variant: badge::Variant::Success,
                                    ..Default::default()
                                    }),
                                    InvoiceStatus::Pending => badge::render(badge::Props {
                                        label: "Pending".into(),
                                        variant: badge::Variant::Warning,
                                    ..Default::default()
                                    }),
                                    InvoiceStatus::Failed => badge::render(badge::Props {
                                        label: "Failed".into(),
                                        variant: badge::Variant::Danger,
                                    ..Default::default()
                                    }),
                                };
                                let download = html! {
                                    a href=(inv.download_url) class="mui-block--billing__download" {
                                        "Download"
                                    }
                                };
                                vec![
                                    table::CellMarkup::text(&inv.date),
                                    table::CellMarkup::text(&inv.number),
                                    table::CellMarkup::text(&inv.amount),
                                    table::CellMarkup::markup(status, false),
                                    table::CellMarkup::markup(download, true),
                                ]
                            }).collect(),
                            footer_row: Vec::new(),
                            right_align_cols: vec![2, 4],
                            striped: false,
                            hoverable: true,
                            compact: false,
                            caption: None,
                        }))
                    },
                    ..Default::default()
                }))
            }
        }
    }
}

pub fn preview() -> Markup {
    render(Props {
        plan: PlanSummary {
            name: "Pro".into(),
            price: "$79".into(),
            billing_period: "Monthly".into(),
            next_charge_date: "May 17, 2026".into(),
            seats_used: 8,
            seats_total: 10,
        },
        payment_method: Some(PaymentMethod {
            brand: "Visa".into(),
            last4: "4242".into(),
            expiry: "09/2027".into(),
        }),
        invoices: vec![
            Invoice {
                date: "Apr 17, 2026".into(),
                number: "INV-2026-0417".into(),
                amount: "$79.00".into(),
                status: InvoiceStatus::Paid,
                download_url: "/invoices/INV-2026-0417.pdf".into(),
            },
            Invoice {
                date: "Mar 17, 2026".into(),
                number: "INV-2026-0317".into(),
                amount: "$79.00".into(),
                status: InvoiceStatus::Paid,
                download_url: "/invoices/INV-2026-0317.pdf".into(),
            },
            Invoice {
                date: "Feb 17, 2026".into(),
                number: "INV-2026-0217".into(),
                amount: "$79.00".into(),
                status: InvoiceStatus::Paid,
                download_url: "/invoices/INV-2026-0217.pdf".into(),
            },
            Invoice {
                date: "Jan 17, 2026".into(),
                number: "INV-2026-0117".into(),
                amount: "$59.00".into(),
                status: InvoiceStatus::Paid,
                download_url: "/invoices/INV-2026-0117.pdf".into(),
            },
        ],
        change_plan_action: Some("/settings/billing/change".into()),
        cancel_plan_action: Some("/settings/billing/cancel".into()),
        update_payment_action: Some("/settings/billing/payment".into()),
    })
}
