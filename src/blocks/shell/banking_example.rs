//! Fictional banking workspace. Amounts use integer cents and retain every decimal.
use super::{app_header, brand_mark, page_header, sidebar};
use crate::{
    blocks::{
        action::{Action, Heading},
        record, task, worklist,
    },
    primitives::{badge, breadcrumb::BreadcrumbItem, data_table, facts_list::Fact, table},
};
use maud::{html, Markup};

fn usd(cents: i64) -> String {
    let amount = cents.unsigned_abs();
    let digits = (amount / 100).to_string();
    let grouped: String = digits
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && (digits.len() - i).is_multiple_of(3) {
                vec![',', c]
            } else {
                vec![c]
            }
        })
        .collect();
    format!(
        "{}${grouped}.{:02}",
        if cents < 0 { "−" } else { "" },
        amount % 100
    )
}
fn column(key: &str, label: &str, numeric: bool) -> data_table::Column {
    data_table::Column {
        key: key.into(),
        label: label.into(),
        sortable: true,
        align: if numeric {
            data_table::Align::Right
        } else {
            data_table::Align::Left
        },
    }
}
pub fn render(id: &str) -> Markup {
    let accounts = [
        ("Operating · 1042", 12_468_035_i64, 11_988_035_i64),
        ("Reserve · 2091", 8_005_010, 8_005_010),
    ];
    let ledger = accounts.iter().map(|a| a.1).sum();
    let available = accounts.iter().map(|a| a.2).sum();
    let ledger_id = format!("{id}-accounts");
    let transactions_id = format!("{id}-transactions");
    let kyc_id = format!("{id}-kyc");
    let cases = [
        (
            "aster",
            "Aster Design LLC",
            "Address document needed",
            "Company address",
            "Updated utility statement",
            "US-402193",
        ),
        (
            "field",
            "Fieldwork Supply Inc.",
            "Ready for review",
            "Owner identity",
            "Identity document received",
            "US-520804",
        ),
    ];
    html! {
        div data-brand="bank" data-mui-density-scope="compact" {
            (sidebar::render(sidebar::Props {
                id: id.into(), embedded: true, brand_mark: brand_mark::preset("bank"),
                app_header: app_header::render(app_header::Props { brand_mark: Some(brand_mark::Props { wordmark: "Northline".into(), ..Default::default() }), actions: Some(html! { span class="mui-caption" { "Operations workspace" } }), ..Default::default() }),
                active_path: format!("#{ledger_id}"),
                nav_groups: vec![sidebar::NavGroup { label: Some("Workspace".into()), items: vec![
                    sidebar::NavItem { label: "Accounts".into(), href: format!("#{ledger_id}"), ..Default::default() },
                    sidebar::NavItem { label: "Transactions".into(), href: format!("#{transactions_id}"), ..Default::default() },
                    sidebar::NavItem { label: "KYC queue".into(), href: format!("#{kyc_id}"), badge: Some("2".into()), ..Default::default() },
                ] }],
                page_header: Some(page_header::Props {
                    breadcrumbs: vec![BreadcrumbItem { label: "Northline".into(), href: Some("#lp-bank-panel".into()) }, BreadcrumbItem { label: "Business operations".into(), href: None }],
                    search: Some(page_header::Search { action: format!("#{transactions_id}"), placeholder: "Search transactions…".into(), ..Default::default() }),
                    switchers: html! { span class="mui-caption" { "USD · 8 September 2026" } }, ..Default::default()
                }),
                children: html! {
                    div class="mui-bank-example" data-mui="banking-demo" {
                        (worklist::header::render(worklist::header::Props { title: "Business operations".into(), count_sentence: "2 accounts · 2 reviews waiting".into(), ..Default::default() }))
                        (record::money::render(record::money::Props { title: "Cash position".into(), currency: "USD".into(), total: record::money::Figure::new("Ledger balance", usd(ledger)), paid: record::money::Figure::new("Available", usd(available)), due: record::money::Figure::new("On hold", usd(ledger - available)), emphasize_due: false, breakdown: Some("Balances across the two accounts below.".into()), ..Default::default() }))
                        section id=(ledger_id) aria-label="Accounts" {
                            h2 class="mui-bank-example__heading" { "Accounts" }
                            (table::render(table::Props { headers: ["Account", "Ledger balance · USD", "Available · USD", "On hold · USD"].into_iter().map(str::to_string).collect(), right_align_cols: vec![1,2,3], caption: Some("Account balances in US dollars".into()), rows: accounts.into_iter().map(|(label, ledger, available)| vec![label.into(), usd(ledger), usd(available), usd(ledger - available)]).collect(), footer_row: vec![table::CellMarkup::text("Total"), table::CellMarkup::right(&usd(ledger)), table::CellMarkup::right(&usd(available)), table::CellMarkup::right(&usd(ledger - available))], ..Default::default() }))
                        }
                        section id=(&transactions_id) aria-label="Transactions" {
                            h2 class="mui-bank-example__heading" { "Transactions" }
                            p class="mui-caption" { "Posted activity · Sort an amount or filter by account and description." }
                            (data_table::render(data_table::Props { id: format!("{id}-transaction-table"), columns: vec![column("date","Date",false), column("description","Description",false), column("account","Account",false), column("amount","Amount · USD",true)], rows: [
                                ("8 Sep", "Client receipt · Harbor Co.", "Operating", 850_000),
                                ("8 Sep", "Supplier payment · Paperworks", "Operating", -120_425),
                                ("8 Sep", "Payroll batch", "Operating", -995_010),
                                ("7 Sep", "Contractor settlement", "Operating", -995_001),
                                ("7 Sep", "Monthly account fee", "Reserve", -2_400),
                            ].into_iter().map(|(date, description, account, amount)| vec![date.into(), description.into(), account.into(), usd(amount)]).collect(), searchable: true, search_placeholder: "Filter transactions…".into(), page_size: 5, ..Default::default() }))
                        }
                        section id=(kyc_id) aria-label="Know your customer review queue" {
                            h2 class="mui-bank-example__heading" { "KYC queue" }
                            (worklist::grouped::render(worklist::grouped::Props { aria_label: "KYC reviews".into(), groups: vec![worklist::grouped::Group { label: "Today · 2 reviews".into(), rows: cases.iter().map(|(key,name,status,field,value,_)| worklist::grouped::Row {
                                identity: (*name).into(), subline: "Business account application".into(), facts: [Fact::new(*field,*value),Fact::new("Submitted","8 Sep · 09:20")], status: badge::Props { label: (*status).into(), variant: if *key == "aster" { badge::Variant::Warning } else { badge::Variant::Info }, ..Default::default() }, action_markup: Some(html! { a class="mui-btn mui-btn--outline mui-btn--row" data-bank-review href=(format!("#{id}-case-{key}")) { "Review" } }), ..Default::default()
                            }).collect(), ..Default::default() }], ..Default::default() }))
                            @for (key,name,status,field,value,registration) in cases {
                                details class="mui-bank-example__case" id=(format!("{id}-case-{key}")) data-bank-case {
                                    summary { (name) " · Review details" }
                                    (record::related_card::render(record::related_card::Props { variant: record::related_card::Variant::Compact, title: name.into(), subtitle: Some(status.into()), facts: vec![Fact::new("Registration",registration),Fact::new(field,value),Fact::new("Review owner","Operations team")], heading: Heading::H3, action_markup: Some(html! { button type="button" class="mui-btn mui-btn--outline mui-btn--row" data-bank-close { "Done reviewing" } }), ..Default::default() }))
                                }
                            }
                        }
                        (task::grid::render(task::grid::Props { tasks: vec![task::grid::Task { title: "Review the next application".into(), description: "Check the supplied documents before completing onboarding.".into(), action: Action::link("Open the queue", format!("#{id}-kyc")) }], ..Default::default() }))
                        p class="mui-caption" { "Fictional accounts and applications · All amounts shown in USD" }
                    }
                }, ..Default::default()
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::usd;
    #[test]
    fn cents_format_without_float_rounding() {
        assert_eq!(usd(20_473_045), "$204,730.45");
        assert_eq!(usd(-995_001), "−$9,950.01");
        assert_eq!(usd(0), "$0.00");
        assert_eq!(usd(9_007_199_254_740_991), "$90,071,992,547,409.91");
    }
}
