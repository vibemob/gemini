use serde::{Deserialize, Serialize};

// TODO: uncomment when the API call is uncommented
//use futures::future::join_all;

/// The structure of the data we expect from the FMP API.
/// We use `serde` to automatically deserialize the JSON response into this struct.
#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StockQuoteData {
    // TODO: uncomment serde attributes when API is uncommented
    pub symbol: String,
    pub name: Option<String>,
    pub price: Option<f64>,
    //#[serde(rename = "changePercentage")]
    pub change_pct: Option<f64>,
    pub volume: Option<u64>,
    //#[serde(rename = "marketCap")]
    pub market_cap: Option<f64>,
    //#[serde(rename = "priceToEarningsRatio")]
    pub pe: Option<f64>,
    //#[serde(rename = "ownersEarningsPerShare")]
    pub eps: Option<f64>,
    pub open: Option<f64>,
    //#[serde(rename = "previousClose")]
    pub previous_close: Option<f64>,
    //#[serde(rename = "dayLow")]
    pub day_low: Option<f64>,
    //#[serde(rename = "dayHigh")]
    pub day_high: Option<f64>,
    //#[serde(rename = "yearLow")]
    pub year_low: Option<f64>,
    //#[serde(rename = "yearHigh")]
    pub year_high: Option<f64>,
    pub beta: Option<f64>,
    // TODO: find out API name
    pub dividend_per_share: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub payout_ratio: Option<f64>,
    pub net_margin: Option<f64>,
    pub return_on_assets: Option<f64>,
    pub return_on_equity: Option<f64>,
    pub revenue_ttm: Option<f64>,
    pub revenue_growth_ttm: Option<f64>,
    pub gross_profit_ttm: Option<f64>,
    pub operating_income_ttm: Option<f64>,
    pub net_income_ttm: Option<f64>,
    pub cash_on_hand_fq: Option<f64>,
    pub total_debt_fq: Option<f64>,
    pub total_equity_fq: Option<f64>,
    pub debt_to_equity_fq: Option<f64>,
    pub free_cash_flow: Option<f64>,
}

// A list of stock symbols we want to fetch data for.
// TODO: uncomment when the API call is uncommented
//const SYMBOLS: &[&str] = &["AAPL", "MSFT", "GOOGL"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StockChartPoint {
    /// Timestamp in milliseconds since epoch
    pub timestamp: i64,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StockChartData {
    pub points: Vec<StockChartPoint>,
}
