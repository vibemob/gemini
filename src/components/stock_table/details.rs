use dioxus::prelude::*;
use super::types::{StockQuoteData, format_opt};
use super::chart::PriceChart;

/// Helper to format a price range.
fn format_range(low: Option<f64>, high: Option<f64>) -> String {
    match (low, high) {
        (Some(l), Some(h)) => format!("{:.2} - {:.2}", l, h),
        _ => "N/A".to_string(),
    }
}

#[component]
pub fn StockDetailTabs(stock: StockQuoteData) -> Element {
    let mut active_tab = use_signal(|| "Overview".to_string());

    rsx! {
        div {
            // Tab navigation
            div {
                class: "flex border-b border-gray-300",
                for tab_name in ["Overview", "Dividends", "Profitability", "Income Statement", "Balance Sheet"].iter() {
                    button {
                        class: "px-4 py-2 -mb-px font-semibold rounded-t-lg text-black",
                        class: if *active_tab.read() == *tab_name {
                            "bg-[#e6f7ff] border-l border-t border-r border-gray-300"
                        } else {
                            ""
                        },
                        onclick: move |_| {
                            active_tab.set(tab_name.to_string());
                        },
                        "{tab_name}"
                    }
                }
            }

            // Tab content
            {
                match active_tab.read().as_str() {
                    "Overview" => rsx!{
                        OverviewTab { stock: stock.clone() }
                        PriceChart { stock: stock.clone() }
                    },
                    "Dividends" => rsx!{ DividendsTab { stock: stock.clone() } },
                    "Profitability" => rsx!{ ProfitabilityTab { stock: stock.clone() } },
                    "Income Statement" => rsx!{ IncomeStatementTab { stock: stock.clone() } },
                    "Balance Sheet" => rsx!{ BalanceSheetTab { stock: stock.clone() } },
                    _ => rsx!{ div { "Unknown tab" } }
                }
            }
        }
    }
}

#[component]
fn OverviewTab(stock: StockQuoteData) -> Element {
    rsx! {
        div { class: "mt-4 mb-2",
            table { class: "min-w-full text-sm table-fixed",
                thead {
                    tr { class: "text-left text-gray-500 border-b",
                        th { class: "w-1/5 pb-1 font-medium uppercase text-xs tracking-wider", "Open" }
                        th { class: "w-1/5 pb-1 font-medium uppercase text-xs tracking-wider", "Prev Close" }
                        th { class: "w-1/5 pb-1 font-medium uppercase text-xs tracking-wider", "Day Range" }
                        th { class: "w-1/5 pb-1 font-medium uppercase text-xs tracking-wider", "52 Week Range" }
                        th { class: "w-1/5 pb-1 font-medium uppercase text-xs tracking-wider", "Beta" }
                    }
                }
                tbody {
                    tr {
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.open.map(|v| format!("{:.2}", v)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.previous_close.map(|v| format!("{:.2}", v)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_range(stock.day_low, stock.day_high)} }
                        td { class: "pt-1 font-medium text-gray-900", {format_range(stock.year_low, stock.year_high)} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.beta.map(|v| format!("{:.2}", v)))} }
                    }
                }
            }
        }
    }
}

#[component]
fn DividendsTab(stock: StockQuoteData) -> Element {
    rsx! {
        div { class: "mt-4 mb-2",
            table { class: "min-w-full text-sm table-fixed",
                thead {
                    tr { class: "text-left text-gray-500 border-b",
                        th { class: "w-1/3 pb-1 font-medium uppercase text-xs tracking-wider", "Dividend / Share" }
                        th { class: "w-1/3 pb-1 font-medium uppercase text-xs tracking-wider", "Dividend Yield" }
                        th { class: "w-1/3 pb-1 font-medium uppercase text-xs tracking-wider", "Payout Ratio" }
                    }
                }
                tbody {
                    tr {
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.dividend_per_share.map(|v| format!("${:.2}", v)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.dividend_yield.map(|v| format!("{:.2}%", v)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.payout_ratio.map(|v| format!("{:.2}%", v)))} }
                    }
                }
            }
        }
    }
}

#[component]
fn ProfitabilityTab(stock: StockQuoteData) -> Element {
    rsx! {
        div { class: "mt-4 mb-2",
            table { class: "min-w-full text-sm table-fixed",
                thead {
                    tr { class: "text-left text-gray-500 border-b",
                        th { class: "w-1/3 pb-1 font-medium uppercase text-xs tracking-wider", "Net Margin TTM" }
                        th { class: "w-1/3 pb-1 font-medium uppercase text-xs tracking-wider", "ROA TTM" }
                        th { class: "w-1/3 pb-1 font-medium uppercase text-xs tracking-wider", "ROE TTM" }
                    }
                }
                tbody {
                    tr {
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.net_margin.map(|v| format!("{:.2}%", v)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.return_on_assets.map(|v| format!("{:.2}%", v)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.return_on_equity.map(|v| format!("{:.2}%", v)))} }
                    }
                }
            }
        }
    }
}

#[component]
fn IncomeStatementTab(stock: StockQuoteData) -> Element {
    rsx! {
        div { class: "mt-4 mb-2",
            table { class: "min-w-full text-sm table-fixed",
                thead {
                    tr { class: "text-left text-gray-500 border-b",
                        th { class: "w-1/5 pb-1 font-medium uppercase text-xs tracking-wider", "Revenue TTM" }
                        th { class: "w-1/5 pb-1 font-medium uppercase text-xs tracking-wider", "Revenue Growth" }
                        th { class: "w-1/5 pb-1 font-medium uppercase text-xs tracking-wider", "Gross Profit" }
                        th { class: "w-1/5 pb-1 font-medium uppercase text-xs tracking-wider", "Operating Income" }
                        th { class: "w-1/5 pb-1 font-medium uppercase text-xs tracking-wider", "Net Income" }
                    }
                }
                tbody {
                    tr {
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.revenue_ttm.map(|v| format!("${:.2}B", v / 1_000_000_000.0)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.revenue_growth_ttm.map(|v| format!("{:.2}%", v)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.gross_profit_ttm.map(|v| format!("${:.2}B", v / 1_000_000_000.0)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.operating_income_ttm.map(|v| format!("${:.2}B", v / 1_000_000_000.0)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.net_income_ttm.map(|v| format!("${:.2}B", v / 1_000_000_000.0)))} }
                    }
                }
            }
        }
    }
}

#[component]
fn BalanceSheetTab(stock: StockQuoteData) -> Element {
    rsx! {
        div { class: "mt-4 mb-2",
            table { class: "min-w-full text-sm table-fixed",
                thead {
                    tr { class: "text-left text-gray-500 border-b",
                        th { class: "w-1/4 pb-1 font-medium uppercase text-xs tracking-wider", "Cash on Hand FQ" }
                        th { class: "w-1/4 pb-1 font-medium uppercase text-xs tracking-wider", "Total Debt FQ" }
                        th { class: "w-1/4 pb-1 font-medium uppercase text-xs tracking-wider", "Total Equity FQ" }
                        th { class: "w-1/4 pb-1 font-medium uppercase text-xs tracking-wider", "Debt/Equity FQ" }
                    }
                }
                tbody {
                    tr {
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.cash_on_hand_fq.map(|v| format!("${:.2}B", v / 1_000_000_000.0)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.total_debt_fq.map(|v| format!("${:.2}B", v / 1_000_000_000.0)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.total_equity_fq.map(|v| format!("${:.2}B", v / 1_000_000_000.0)))} }
                        td { class: "pt-1 font-medium text-gray-900", {format_opt(stock.debt_to_equity_fq.map(|v| format!("{:.2}", v)))} }
                    }
                }
            }
        }
    }
}