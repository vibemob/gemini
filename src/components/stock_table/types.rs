use serde::Deserialize;

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

/// Helper to format optional numbers, returning "N/A" if None.
pub fn format_opt<T: std::fmt::Display>(val: Option<T>) -> String {
    val.map(|v| v.to_string())
        .unwrap_or_else(|| "N/A".to_string())
}

/// Enum to represent the direction of sorting.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    /// Reverses the sort direction.
    pub fn reverse(&self) -> Self {
        match self {
            Self::Asc => Self::Desc,
            Self::Desc => Self::Asc,
        }
    }
}