use crate::components::{StockLineChart, ChartTimeRange};
use crate::data_models::chart_data::StockQuoteData;
use dioxus::prelude::*;
use std::cmp::Ordering;

/// A component that renders a table for stock data.
#[component]
pub fn StockTable() -> Element {
    // Use the `use_resource` hook to fetch data asynchronously.
    // This will run once when the component is mounted.

    let stocks_resource = use_resource(move || async move {
        // --- MOCK DATA ---
        // Using a static vector of mock data for UI development.
        // This avoids the need for live API calls.
        let mock_stock_msft = StockQuoteData {
            symbol: "MSFT".to_string(),
            name: Some("Microsoft Corporation".to_string()),
            price: Some(472.12),
            change_pct: Some(-2.3),
            volume: Some(31_397_484),
            market_cap: Some(3_509_000_000_000.0), // Stored as the full value
            pe: Some(33.56),
            eps: Some(14.07),
            open: Some(475.50),
            previous_close: Some(483.23),
            day_low: Some(470.10),
            day_high: Some(478.00),
            year_low: Some(309.00),
            year_high: Some(490.00),
            beta: Some(0.89),
            dividend_per_share: Some(3.32),
            dividend_yield: Some(0.70),
            payout_ratio: Some(24.5),
            net_margin: Some(36.45),
            return_on_assets: Some(21.23),
            return_on_equity: Some(39.12),
            revenue_ttm: Some(245_120_000_000.0),
            revenue_growth_ttm: Some(15.67),
            gross_profit_ttm: Some(165_340_000_000.0),
            operating_income_ttm: Some(109_450_000_000.0),
            net_income_ttm: Some(88_120_000_000.0),
            cash_on_hand_fq: Some(80_000_000_000.0),
            total_debt_fq: Some(70_000_000_000.0),
            total_equity_fq: Some(230_000_000_000.0),
            debt_to_equity_fq: Some(0.30),
            free_cash_flow: Some(74_000_000_000.0),
        };

        let mock_stock_meta = StockQuoteData {
            symbol: "META".to_string(),
            name: Some("Meta Platforms, Inc.".to_string()),
            price: Some(594.25),
            change_pct: Some(0.85),
            volume: Some(20_524_255),
            market_cap: Some(1_498_000_000_000.0), // Stored as the full value
            pe: Some(26.32),
            eps: Some(22.58),
            open: Some(590.00),
            previous_close: Some(589.20),
            day_low: Some(585.50),
            day_high: Some(600.00),
            year_low: Some(280.00),
            year_high: Some(600.00),
            beta: Some(1.20),
            dividend_per_share: Some(2.00),
            dividend_yield: Some(0.34),
            payout_ratio: Some(9.8),
            net_margin: Some(34.21),
            return_on_assets: Some(19.85),
            return_on_equity: Some(28.54),
            revenue_ttm: Some(150_230_000_000.0),
            revenue_growth_ttm: Some(21.45),
            gross_profit_ttm: Some(120_670_000_000.0),
            operating_income_ttm: Some(60_120_000_000.0),
            net_income_ttm: Some(50_340_000_000.0),
            cash_on_hand_fq: Some(65_000_000_000.0),
            total_debt_fq: Some(37_000_000_000.0),
            total_equity_fq: Some(150_000_000_000.0),
            debt_to_equity_fq: Some(0.25),
            free_cash_flow: Some(49_000_000_000.0),
        };

        let mock_stock_amzn = StockQuoteData {
            symbol: "AMZN".to_string(),
            name: Some("Amazon.com, Inc".to_string()),
            price: Some(220.69),
            change_pct: Some(1.63),
            volume: Some(65_553_588),
            market_cap: Some(2_359_000_000_000.0), // Stored as the full value
            pe: Some(31.17),
            eps: Some(7.08),
            open: Some(218.00),
            previous_close: Some(217.15),
            day_low: Some(215.00),
            day_high: Some(222.00),
            year_low: Some(120.00),
            year_high: Some(230.00),
            beta: Some(1.10),
            dividend_per_share: None,
            dividend_yield: None,
            payout_ratio: None,
            net_margin: Some(6.38),
            return_on_assets: Some(8.12),
            return_on_equity: Some(20.15),
            revenue_ttm: Some(600_450_000_000.0),
            revenue_growth_ttm: Some(11.23),
            gross_profit_ttm: Some(280_120_000_000.0),
            operating_income_ttm: Some(50_670_000_000.0),
            net_income_ttm: Some(40_890_000_000.0),
            cash_on_hand_fq: Some(86_000_000_000.0),
            total_debt_fq: Some(160_000_000_000.0),
            total_equity_fq: Some(200_000_000_000.0),
            debt_to_equity_fq: Some(0.80),
            free_cash_flow: Some(48_000_000_000.0),
        };

        let mock_stock_goog = StockQuoteData {
            symbol: "GOOG".to_string(),
            name: Some("Alphabet, Inc.".to_string()),
            price: Some(299.65),
            change_pct: Some(3.33),
            volume: Some(43_507_900),
            market_cap: Some(3_617_000_000_000.0), // Stored as the full value
            pe: Some(29.58),
            eps: Some(10.13),
            open: Some(290.00),
            previous_close: Some(290.00),
            day_low: Some(288.00),
            day_high: Some(302.00),
            year_low: Some(130.00),
            year_high: Some(305.00),
            beta: Some(1.05),
            dividend_per_share: Some(0.80),
            dividend_yield: Some(0.27),
            payout_ratio: Some(7.9),
            net_margin: Some(26.12),
            return_on_assets: Some(17.54),
            return_on_equity: Some(29.32),
            revenue_ttm: Some(320_560_000_000.0),
            revenue_growth_ttm: Some(13.45),
            gross_profit_ttm: Some(180_230_000_000.0),
            operating_income_ttm: Some(90_120_000_000.0),
            net_income_ttm: Some(80_450_000_000.0),
            cash_on_hand_fq: Some(110_000_000_000.0),
            total_debt_fq: Some(28_000_000_000.0),
            total_equity_fq: Some(280_000_000_000.0),
            debt_to_equity_fq: Some(0.10),
            free_cash_flow: Some(69_000_000_000.0),
        };

        // Create a vector with 8 clones of the mock stock data.
        let stocks: Vec<StockQuoteData> = vec![
            mock_stock_msft,
            mock_stock_amzn,
            mock_stock_goog,
            mock_stock_meta,
        ];

        // The hook expects a Result, so we wrap our mock data in Ok.
        // We explicitly type the error to match what `reqwest` would produce.
        Ok::<_, reqwest::Error>(stocks)
    });

    //     let futures = SYMBOLS.iter().map(|&symbol| {
    //         let client = client.clone();
    //         let api_key = api_key.clone();
    //         async move {
    //             let quote_url = format!(
    //                 "https://financialmodelingprep.com/stable/quote?symbol={}&apikey={}",
    //                 symbol, api_key
    //             );
    //             let ratios_url = format!(
    //                 "https://financialmodelingprep.com/stable/ratios?symbol={}&apikey={}",
    //                 symbol, api_key
    //             );
    //             let earnings_url = format!(
    //                 "https://financialmodelingprep.com/stable/owner-earnings?symbol={}&apikey={}",
    //                 symbol, api_key
    //             );

    //             // Use `try_join!` to run the three API requests concurrently for a single stock.
    //             let (quote_res, ratios_res, earnings_res) = futures::try_join!(
    //                 client.get(&quote_url).send(),
    //                 client.get(&ratios_url).send(),
    //                 client.get(&earnings_url).send()
    //             )?;

    //             // Deserialize the responses. The API returns a `Vec` with one item, so we take the first.
    //             let mut quote_data: Vec<StockQuoteData> = quote_res.json().await?;
    //             let mut ratios_data: Vec<StockQuoteData> = ratios_res.json().await?;
    //             let mut earnings_data: Vec<StockQuoteData> = earnings_res.json().await?;

    //             let mut stock = quote_data.remove(0);
    //             stock.pe = ratios_data.remove(0).pe;
    //             stock.eps = earnings_data.remove(0).eps;

    //             Ok::<_, reqwest::Error>(stock)
    //         }
    //     });

    //     // Execute all tasks concurrently and collect the results.
    //     let results = join_all(futures).await;

    //     tracing::info!("{:?}", results);
    //     let stocks: Vec<_> = results.into_iter().filter_map(Result::ok).collect();

    //     Ok::<_, reqwest::Error>(stocks)
    // });

    rsx! {
        div {
            id: "stocktable",
            class: "w-full p-4 text-gray-300 rounded-lg border border-gray shadow-lg",
            div {
                class: "overflow-x-auto",
                div {
                    class: "min-w-full",
                    table {
                        class: "bg-white min-w-full leading-normal",
                        // Render content based on the state of the resource
                        {
                            match &*stocks_resource.read_unchecked() {
                                // If the resource is still loading, show a loading message
                                None => rsx! {
                                    tbody {
                                        tr {
                                            td {
                                                colspan: "9",
                                                class: "py-8 text-center text-gray-500",
                                                "Loading stock data..."
                                            }
                                        }
                                    }
                                },
                                // If there was an error, display it
                                Some(Err(e)) => rsx! {
                                    tbody {
                                        tr {
                                            td {
                                                colspan: "9",
                                                class: "py-8 text-center text-red-500",
                                                "Error fetching data: {e}"
                                            }
                                        }
                                    }
                                },
                                // If we have data, render the table
                                Some(Ok(stocks)) => rsx! {
                                    TableContents { stocks: stocks.clone() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Helper to format optional numbers, returning "N/A" if None.
fn format_opt<T: std::fmt::Display>(val: Option<T>) -> String {
    val.map(|v| v.to_string())
        .unwrap_or_else(|| "N/A".to_string())
}

/// Helper to format a price range.
fn format_range(low: Option<f64>, high: Option<f64>) -> String {
    match (low, high) {
        (Some(l), Some(h)) => format!("{:.2} - {:.2}", l, h),
        _ => "N/A".to_string(),
    }
}

/// Enum to represent the direction of sorting.
#[derive(Clone, Copy, PartialEq, Debug)]
enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    /// Reverses the sort direction.
    fn reverse(&self) -> Self {
        match self {
            Self::Asc => Self::Desc,
            Self::Desc => Self::Asc,
        }
    }
}

/// A sub-component to render the actual table structure once data is available.
#[component]
fn TableContents(stocks: Vec<StockQuoteData>) -> Element {
    // Signal to hold the current sort state: (column_index, direction)
    let mut sort_by = use_signal(|| None::<(usize, SortDirection)>);
    // Signal to hold the currently expanded row symbol
    let mut expanded_symbol = use_signal(|| None::<String>);
    // Signal to hold which stock has its chart view expanded
    let mut chart_expanded_symbol = use_signal(|| None::<String>);
    // Signal to hold the time range for the chart (1D or 5D)
    let mut chart_time_range = use_signal(|| ChartTimeRange::OneDay);

    // Memoize the sorted stocks. This will re-compute only when `sort_by` changes.
    let sorted_stocks = use_memo(move || {
        let mut stocks = stocks.clone();
        if let Some((col_index, direction)) = *sort_by.read() {
            stocks.sort_by(|a, b| {
                // Generic comparison function for optional numeric values.
                // `None` values are always considered "lesser" and pushed to the bottom.
                fn compare_opt<T: PartialOrd>(a: Option<T>, b: Option<T>) -> Ordering {
                    match (a, b) {
                        (Some(a_val), Some(b_val)) => {
                            a_val.partial_cmp(&b_val).unwrap_or(Ordering::Equal)
                        }
                        (Some(_), None) => Ordering::Less,
                        (None, Some(_)) => Ordering::Greater,
                        (None, None) => Ordering::Equal,
                    }
                }

                let ordering = match col_index {
                    // Symbol
                    0 => a.symbol.cmp(&b.symbol),
                    // Company Name
                    1 => a.name.cmp(&b.name),
                    // Price
                    2 => compare_opt(a.price, b.price),
                    // Change %
                    3 => compare_opt(a.change_pct, b.change_pct),
                    // Volume
                    4 => compare_opt(a.volume, b.volume),
                    // Market Cap
                    5 => compare_opt(a.market_cap, b.market_cap),
                    // PE Ratio
                    6 => compare_opt(a.pe, b.pe),
                    // EPS
                    7 => compare_opt(a.eps, b.eps),
                    // Free Cash Flow
                    8 => compare_opt(a.free_cash_flow, b.free_cash_flow),
                    _ => Ordering::Equal,
                };

                // Reverse order if descending
                if direction == SortDirection::Desc {
                    ordering.reverse()
                } else {
                    ordering
                }
            });
        }
        stocks
    });

    rsx! {
        thead {
            id: "stocks-header",
            tr { class: "border-b border-gray-200",
                for (i, col) in ["Symbol", "Company Name", "Price", "Change % (1 day)", "Volume", "Market Cap (in Billions)", "PE Ratio", "Earnings Per Share", "Free Cash Flow"].iter().enumerate() {
                    {
                        let is_sorted_col = (*sort_by.read()).is_some_and(|(col_idx, _dir)| col_idx == i);
                        let font_weight_class = if is_sorted_col { "font-bold text-black" } else { "font-semibold text-gray-400" };
                        rsx!{
                            th {
                                class: "group px-4 py-3 text-xs uppercase tracking-wider cursor-pointer hover:bg-gray-50 {font_weight_class}",
                                class: if i < 2 { "text-left" } else { "text-right" },
                                onclick: move |_| {
                                    // Read the current sort state and immediately drop the read guard
                                    let new_sort = if let Some((current_col, current_dir)) = *sort_by.read() {
                                        if current_col == i {
                                            // If clicking the same column, reverse the direction
                                            Some((i, current_dir.reverse()))
                                        } else {
                                            // If clicking a new column, set it to descending
                                            Some((i, SortDirection::Desc))
                                        }
                                    } else {
                                        // If no sort is active, set it to descending
                                        Some((i, SortDirection::Desc))
                                    };
                                    sort_by.set(new_sort);
                                },

                                div {
                                    class: "flex items-center",
                                    class: if i >= 2 { "justify-end" } else { "justify-start" },

                                    // Column title
                                    span { "{col}" }

                                    // Sort arrow icon
                                    {
                                        let direction = (*sort_by.read()).map(|(_col_idx, dir)| dir);

                                        // Classes to control visibility and style
                                        let arrow_class = if is_sorted_col {
                                            "font-bold opacity-100"
                                        } else {
                                            "opacity-0 group-hover:opacity-100"
                                        };

                                        rsx! {
                                            svg {
                                                class: "w-4 h-4 ml-1 transition-opacity {arrow_class}",
                                                fill: "none",
                                                stroke: "currentColor",
                                                "stroke-width": "2",
                                                "viewBox": "0 0 24 24",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                // Point arrow up for ascending, down for descending
                                                if direction == Some(SortDirection::Asc) && is_sorted_col {
                                                    path { "stroke-linecap": "round", "stroke-linejoin": "round", d: "M5 15l7-7 7 7" }
                                                } else {
                                                    path { "stroke-linecap": "round", "stroke-linejoin": "round", d: "M19 9l-7 7-7-7" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        tbody {
            {
                sorted_stocks.read().iter().map(|stock| {
                    let symbol = stock.symbol.clone();
                    let is_expanded = expanded_symbol.read().as_ref() == Some(&symbol);
                    let is_chart_expanded = chart_expanded_symbol.read().as_ref() == Some(&symbol);

                    rsx! {
                        tr {
                            class: "hover:bg-gray-50 border-b border-gray-200",
                            key: "{symbol}-row",
                            td {
                                class: "px-3 py-4 text-sm text-blue-600 font-medium text-left cursor-pointer hover:underline",
                                onclick: move |_| {
                                    let current_expanded = expanded_symbol.read().clone();
                                    if current_expanded == Some(symbol.clone()) {
                                        *expanded_symbol.write() = None;
                                        *chart_expanded_symbol.write() = None; // Also close chart view
                                    } else {
                                        *expanded_symbol.write() = Some(symbol.clone());
                                    }
                                },
                                "{stock.symbol}"
                            }
                            td { class: "px-3 py-4 text-sm text-black text-left", {format_opt(stock.name.clone())} }
                            td { class: "px-3 py-4 text-sm text-black text-right", {format_opt(stock.price.map(|p| format!("{:.2}", p)))} }
                            td {
                                class: "px-3 py-4 text-sm text-right",
                                span {
                                    class: if stock.change_pct >= Some(0.0) { "text-green-600" } else { "text-red-600" },
                                    if stock.change_pct >= Some(0.0) { "+" } else { "" },
                                    {format_opt(stock.change_pct.map(|c| format!("{:.2}%", c)))}
                                }
                            }
                            td { class: "px-3 py-4 text-sm text-black text-right", {format_opt(stock.volume)} }
                            td { class: "px-3 py-4 text-sm text-black text-right", {format_opt(stock.market_cap.map(|mc| format!("{:.2}", mc / 1_000_000_000.0)))} }
                            td { class: "px-3 py-4 text-sm text-black text-right", {format_opt(stock.pe.map(|pe| format!("{:.2}", pe)))} }
                            td { class: "px-3 py-4 text-sm text-black text-right", {format_opt(stock.eps.map(|eps| format!("{:.2}", eps)))} }
                            td { class: "px-3 py-4 text-sm text-black text-right", {format_opt(stock.free_cash_flow.map(|fcf| format!("${:.2}B", fcf / 1_000_000_000.0)))} }
                        }

                        if is_expanded {
                            tr {
                                class: "bg-white border-b border-gray-200",
                                key: "{symbol}-details",
                                td {
                                    colspan: "9",
                                    class: "p-0",
                                    if is_chart_expanded {
                                        // Chart Expanded View
                                        div { class: "bg-gray-50 mx-12 my-4 p-6 rounded-lg border border-gray-200 space-y-6",
                                            div { class: "flex flex-col",
                                                // Overview Section (Full Width)
                                                div { class: "w-full",
                                                    h4 { class: "flex justify-between items-center font-bold text-gray-700 mb-3 bg-[#e6f7ff] p-2 rounded",
                                                        "Overview"
                                                        a {
                                                            class: "text-sm font-normal text-blue-600 hover:underline cursor-pointer",
                                                            onclick: move |_| *chart_expanded_symbol.write() = None,
                                                            "X"
                                                        }
                                                    }
                                                    div { class: "grid grid-cols-5 gap-y-3 gap-x-4", // Adjusted to 5 columns
                                                        DetailCell { label: "Open".to_string(), value: format_opt(stock.open.map(|v| format!("{:.2}", v))) }
                                                        DetailCell { label: "Prev Close".to_string(), value: format_opt(stock.previous_close.map(|v| format!("{:.2}", v))) }
                                                        DetailCell { label: "Day Range".to_string(), value: format_range(stock.day_low, stock.day_high) }
                                                        DetailCell { label: "52 Week Range".to_string(), value: format_range(stock.year_low, stock.year_high) }
                                                        DetailCell { label: "Beta".to_string(), value: format_opt(stock.beta.map(|v| format!("{:.2}", v))) }
                                                    }
                                                }
                                                // Chart Content Section
                                                div {
                                                    class: "chart-content",
                                                    StockLineChart { symbol: stock.clone(), time_range: *chart_time_range.read() }
                                                }
                                            }
                                        }
                                    } else {
                                        // Standard Expanded View
                                        div { class: "bg-gray-50 mx-12 my-4 p-6 rounded-lg border border-gray-200 space-y-6",
                                            // Row 1: Overview and Dividends
                                            div { class: "flex space-x-6",
                                                // Overview Section
                                                div { class: "flex-1",
                                                    h4 { class: "flex justify-between items-center font-bold text-gray-700 mb-3 bg-[#e6f7ff] p-2 rounded",
                                                        "Overview"
                                                        a {
                                                            class: "text-sm font-normal text-blue-600 hover:underline cursor-pointer mr-2",
                                                            onclick: {
                                                                let symbol_clone = stock.symbol.clone();
                                                                move |_| {
                                                                    *chart_expanded_symbol.write() = Some(symbol_clone.clone());
                                                                    *chart_time_range.write() = ChartTimeRange::OneDay;
                                                                }
                                                            },
                                                            "1D"
                                                        }
                                                        a {
                                                            class: "text-sm font-normal text-blue-600 hover:underline cursor-pointer",
                                                            onclick: {
                                                                let symbol_clone = stock.symbol.clone();
                                                                move |_| {
                                                                    *chart_expanded_symbol.write() = Some(symbol_clone.clone());
                                                                    *chart_time_range.write() = ChartTimeRange::FiveDay;
                                                                }
                                                            },
                                                            "5D"
                                                        }
                                                        a {
                                                            class: "text-sm font-normal text-blue-600 hover:underline cursor-pointer ml-2",
                                                            onclick: {
                                                                let symbol_clone = stock.symbol.clone();
                                                                move |_| {
                                                                    *chart_expanded_symbol.write() = Some(symbol_clone.clone());
                                                                    *chart_time_range.write() = ChartTimeRange::OneMonth;
                                                                }
                                                            },
                                                            "1M"
                                                        }
                                                        a {
                                                            class: "text-sm font-normal text-blue-600 hover:underline cursor-pointer ml-2",
                                                            onclick: {
                                                                let symbol_clone = stock.symbol.clone();
                                                                move |_| {
                                                                    *chart_expanded_symbol.write() = Some(symbol_clone.clone());
                                                                    *chart_time_range.write() = ChartTimeRange::ThreeMonth;
                                                                }
                                                            },
                                                            "3M"
                                                        }
                                                    }
                                                    div { class: "grid grid-cols-3 gap-y-3 gap-x-4",
                                                        DetailCell { label: "Open".to_string(), value: format_opt(stock.open.map(|v| format!("{:.2}", v))) }
                                                        DetailCell { label: "Prev Close".to_string(), value: format_opt(stock.previous_close.map(|v| format!("{:.2}", v))) }
                                                        DetailCell { label: "Day Range".to_string(), value: format_range(stock.day_low, stock.day_high) }
                                                        DetailCell { label: "52 Week Range".to_string(), value: format_range(stock.year_low, stock.year_high) }
                                                        DetailCell { label: "Beta".to_string(), value: format_opt(stock.beta.map(|v| format!("{:.2}", v))) }
                                                    }

                                                }
                                                // Dividends Section
                                                div { class: "flex-1",
                                                    h4 { class: "font-bold text-gray-700 mb-3 bg-[#e6f7ff] p-2 rounded", "Dividends" }
                                                    div { class: "grid grid-cols-3 gap-y-3 gap-x-4",
                                                        DetailCell { label: "Dividend / Share".to_string(), value: format_opt(stock.dividend_per_share.map(|v| format!("${:.2}", v))) }
                                                        DetailCell { label: "Dividend Yield".to_string(), value: format_opt(stock.dividend_yield.map(|v| format!("{:.2}%", v))) }
                                                        DetailCell { label: "Payout Ratio".to_string(), value: format_opt(stock.payout_ratio.map(|v| format!("{:.2}%", v))) }
                                                    }
                                                }
                                            }

                                            // Row 2: Profitability and Income Statement
                                            div { class: "flex space-x-6",
                                                // Profitability Section
                                                div { class: "flex-1",
                                                    h4 { class: "font-bold text-gray-700 mb-3 bg-[#e6f7ff] p-2 rounded", "Profitability" }
                                                    div { class: "grid grid-cols-3 gap-y-3 gap-x-4",
                                                        DetailCell { label: "Net Margin TTM".to_string(), value: format_opt(stock.net_margin.map(|v| format!("{:.2}%", v))) }
                                                        DetailCell { label: "ROA TTM".to_string(), value: format_opt(stock.return_on_assets.map(|v| format!("{:.2}%", v))) }
                                                        DetailCell { label: "ROE TTM".to_string(), value: format_opt(stock.return_on_equity.map(|v| format!("{:.2}%", v))) }
                                                    }
                                                }
                                                // Income Statement Section
                                                div { class: "flex-1",
                                                    h4 { class: "font-bold text-gray-700 mb-3 bg-[#e6f7ff] p-2 rounded", "Income Statement" }
                                                    div { class: "grid grid-cols-3 gap-y-3 gap-x-4",
                                                        DetailCell { label: "Revenue TTM".to_string(), value: format_opt(stock.revenue_ttm.map(|v| format!("${:.2}B", v / 1_000_000_000.0))) }
                                                        DetailCell { label: "Revenue Growth".to_string(), value: format_opt(stock.revenue_growth_ttm.map(|v| format!("{:.2}%", v))) }
                                                        DetailCell { label: "Gross Profit".to_string(), value: format_opt(stock.gross_profit_ttm.map(|v| format!("${:.2}B", v / 1_000_000_000.0))) }
                                                        DetailCell { label: "Operating Income".to_string(), value: format_opt(stock.operating_income_ttm.map(|v| format!("${:.2}B", v / 1_000_000_000.0))) }
                                                        DetailCell { label: "Net Income".to_string(), value: format_opt(stock.net_income_ttm.map(|v| format!("${:.2}B", v / 1_000_000_000.0))) }
                                                    }
                                                }
                                            }

                                            // Row 3: Balance Sheet
                                            div { class: "flex space-x-6",
                                                // Balance Sheet Section
                                                div { class: "flex-1",
                                                    h4 { class: "font-bold text-gray-700 mb-3 bg-[#e6f7ff] p-2 rounded", "Balance Sheet" }
                                                    div { class: "grid grid-cols-3 gap-y-3 gap-x-4",
                                                        DetailCell { label: "Cash on Hand FQ".to_string(), value: format_opt(stock.cash_on_hand_fq.map(|v| format!("${:.2}B", v / 1_000_000_000.0))) }
                                                        DetailCell { label: "Total Debt FQ".to_string(), value: format_opt(stock.total_debt_fq.map(|v| format!("${:.2}B", v / 1_000_000_000.0))) }
                                                        DetailCell { label: "Total Equity FQ".to_string(), value: format_opt(stock.total_equity_fq.map(|v| format!("${:.2}B", v / 1_000_000_000.0))) }
                                                        DetailCell { label: "Debt/Equity FQ".to_string(), value: format_opt(stock.debt_to_equity_fq.map(|v| format!("{:.2}", v))) }
                                                    }
                                                }
                                                // Empty div to take up the other half of the space
                                                div { class: "flex-1" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                })
            }
        }
    }
}

/// A small component to display a labeled data point in the detail view.
#[component]
fn DetailCell(label: String, value: String) -> Element {
    rsx! {
        div { class: "flex flex-col",
            span { class: "text-xs text-gray-500 uppercase tracking-wider", "{label}" }
            span { class: "text-sm font-medium text-gray-900", "{value}" }
        }
    }
}
