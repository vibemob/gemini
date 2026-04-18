use crate::data_models::chart_data::{StockChartData, StockChartPoint, StockQuoteData};
use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};
use rand::Rng;
use std::collections::HashMap;

pub const MARKET_OPEN_H: u32 = 9;
pub const MARKET_OPEN_M: u32 = 30;
pub const MARKET_CLOSE_H: u32 = 16;
pub const MARKET_CLOSE_M: u32 = 0;

// --- Data Fetching and Processing (using mock data) ---
pub async fn fetch_intraday_data_for_chart(quote: &StockQuoteData) -> Option<StockChartData> {
    // In a real application, replace this with an actual API call.
    // This example uses mock data.
    let raw_data_mock = get_mock_alpha_vantage_data(quote.open);

    let mut points: Vec<StockChartPoint> = Vec::new();
    let today_date_naive = Utc::now().date_naive();

    for (datetime_str, values) in raw_data_mock {
        let datetime_naive = if let Ok(dt) = NaiveDate::parse_from_str(
            &datetime_str.split(' ').next().unwrap_or_default(),
            "%Y-%m-%d",
        )
        .and_then(|date| {
            NaiveTime::parse_from_str(
                &datetime_str.split(' ').nth(1).unwrap_or_default(),
                "%H:%M:%S",
            )
            .map(|time| date.and_time(time))
        }) {
            dt
        } else {
            continue;
        };

        if datetime_naive.date() != today_date_naive {
            continue; // Skip data not for today
        }

        let time = datetime_naive.time();
        // Filter for market hours (inclusive of close)
        if let (Some(start), Some(end)) = (NaiveTime::from_hms_opt(MARKET_OPEN_H, MARKET_OPEN_M, 0), NaiveTime::from_hms_opt(MARKET_CLOSE_H, MARKET_CLOSE_M, 0)) {
            if time >= start && time <= end {
                if let (Some(open), Some(close), Some(high), Some(low)) = (values.get("1. open"), values.get("4. close"), values.get("2. high"), values.get("3. low")) {
                    if let (Ok(open), Ok(close), Ok(high), Ok(low)) = (open.parse(), close.parse(), high.parse(), low.parse()) {
                        points.push(StockChartPoint {
                            timestamp: Utc.from_utc_datetime(&datetime_naive).timestamp_millis(), // Convert to UTC timestamp
                            open, close, high, low
                        });
                    }
                }
            }
        }
    }

    // Sort by timestamp to ensure correct plotting order
    points.sort_by_key(|p| p.timestamp);

    // Filter points to be within the precise window, including the close
    let start_of_trading = Utc
        .from_utc_datetime(&today_date_naive.and_time(NaiveTime::from_hms_opt(MARKET_OPEN_H, MARKET_OPEN_M, 0).unwrap()))
        .timestamp_millis();
    let end_of_trading = Utc
        .from_utc_datetime(&today_date_naive.and_time(NaiveTime::from_hms_opt(MARKET_CLOSE_H, MARKET_CLOSE_M, 0).unwrap()))
        .timestamp_millis();

    let filtered_points = points
        .into_iter()
        .filter(|p| p.timestamp >= start_of_trading && p.timestamp <= end_of_trading)
        .collect();

    Some(StockChartData {
        points: filtered_points,
    })
}

pub async fn fetch_5day_data_for_chart(quote: &StockQuoteData) -> Option<StockChartData> {
    let raw_data_mock = get_mock_5day_data(quote.open);

    let mut points: Vec<StockChartPoint> = Vec::new();

    for (datetime_str, values) in raw_data_mock {
        let datetime_naive = if let Ok(dt) = NaiveDate::parse_from_str(
            &datetime_str.split(' ').next().unwrap_or_default(),
            "%Y-%m-%d",
        )
        .and_then(|date| {
            NaiveTime::parse_from_str(
                &datetime_str.split(' ').nth(1).unwrap_or_default(),
                "%H:%M:%S",
            )
            .map(|time| date.and_time(time))
        }) {
            dt
        } else {
            continue;
        };

        let time = datetime_naive.time();
        if let (Some(start), Some(end)) = (NaiveTime::from_hms_opt(MARKET_OPEN_H, MARKET_OPEN_M, 0), NaiveTime::from_hms_opt(MARKET_CLOSE_H, MARKET_CLOSE_M, 0)) {
            if time >= start && time <= end {
                if let (Some(open), Some(close), Some(high), Some(low)) = (values.get("1. open"), values.get("4. close"), values.get("2. high"), values.get("3. low")) {
                    if let (Ok(open), Ok(close), Ok(high), Ok(low)) = (open.parse(), close.parse(), high.parse(), low.parse()) {
                        points.push(StockChartPoint {
                            timestamp: Utc.from_utc_datetime(&datetime_naive).timestamp_millis(),
                            open, close, high, low
                        });
                    }
                }
            }
        }
    }

    points.sort_by_key(|p| p.timestamp);

    Some(StockChartData { points })
}

pub async fn fetch_1month_data_for_chart(quote: &StockQuoteData) -> Option<StockChartData> {
    let raw_data_mock = get_mock_1month_data(quote.open);

    let mut points: Vec<StockChartPoint> = Vec::new();

    for (datetime_str, values) in raw_data_mock {
        let datetime_naive = if let Ok(dt) = NaiveDate::parse_from_str(
            &datetime_str.split(' ').next().unwrap_or_default(),
            "%Y-%m-%d",
        )
        .and_then(|date| {
            NaiveTime::parse_from_str(
                &datetime_str.split(' ').nth(1).unwrap_or_default(),
                "%H:%M:%S",
            )
            .map(|time| date.and_time(time))
        }) {
            dt
        } else {
            continue;
        };

        let time = datetime_naive.time();
        if let (Some(start), Some(end)) = (NaiveTime::from_hms_opt(MARKET_OPEN_H, MARKET_OPEN_M, 0), NaiveTime::from_hms_opt(MARKET_CLOSE_H, MARKET_CLOSE_M, 0)) {
            if time >= start && time <= end {
                if let (Some(open), Some(close), Some(high), Some(low)) = (values.get("1. open"), values.get("4. close"), values.get("2. high"), values.get("3. low")) {
                    if let (Ok(open), Ok(close), Ok(high), Ok(low)) = (open.parse(), close.parse(), high.parse(), low.parse()) {
                        points.push(StockChartPoint {
                            timestamp: Utc.from_utc_datetime(&datetime_naive).timestamp_millis(),
                            open, close, high, low
                        });
                    }
                }
            }
        }
    }

    points.sort_by_key(|p| p.timestamp);

    Some(StockChartData { points })
}

pub async fn fetch_3month_data_for_chart(quote: &StockQuoteData) -> Option<StockChartData> {
    let raw_data_mock = get_mock_3month_data(quote.open);

    let mut points: Vec<StockChartPoint> = Vec::new();

    for (datetime_str, values) in raw_data_mock {
        let datetime_naive = if let Ok(dt) = NaiveDate::parse_from_str(
            &datetime_str.split(' ').next().unwrap_or_default(),
            "%Y-%m-%d",
        )
        .and_then(|date| {
            NaiveTime::parse_from_str(
                &datetime_str.split(' ').nth(1).unwrap_or_default(),
                "%H:%M:%S",
            )
            .map(|time| date.and_time(time))
        }) {
            dt
        } else {
            continue;
        };

        let time = datetime_naive.time();
        if let (Some(start), Some(end)) = (NaiveTime::from_hms_opt(MARKET_OPEN_H, MARKET_OPEN_M, 0), NaiveTime::from_hms_opt(MARKET_CLOSE_H, MARKET_CLOSE_M, 0)) {
            if time >= start && time <= end {
                if let (Some(open), Some(close), Some(high), Some(low)) = (values.get("1. open"), values.get("4. close"), values.get("2. high"), values.get("3. low")) {
                    if let (Ok(open), Ok(close), Ok(high), Ok(low)) = (open.parse(), close.parse(), high.parse(), low.parse()) {
                        points.push(StockChartPoint {
                            timestamp: Utc.from_utc_datetime(&datetime_naive).timestamp_millis(),
                            open, close, high, low
                        });
                    }
                }
            }
        }
    }

    points.sort_by_key(|p| p.timestamp);

    Some(StockChartData { points })
}

fn get_mock_5day_data(start_price: Option<f64>) -> HashMap<String, HashMap<String, String>> {
    let mut data = HashMap::new();
    let mut rng = rand::thread_rng();
    let mut current_price = start_price.unwrap_or(100.0);
    let today = Utc::now().date_naive();

    for day_offset in 0..5 {
        let date = today.pred_opt().and_then(|d| d.checked_sub_days(chrono::Days::new(day_offset as u64)));
        let date = if let Some(d) = date {
            d
        } else {
            continue;
        };
        let date_str = date.format("%Y-%m-%d").to_string();

        if date.format("%w").to_string() == "0" || date.format("%w").to_string() == "6" {
            continue;
        }

        let open = current_price;
        let delta: f64 = rng.gen_range(-1.0..1.0);
        current_price += delta;
        let close = current_price;
        let high = open.max(close) + rng.gen_range(0.0..0.2);
        let low = open.min(close) - rng.gen_range(0.0..0.2);

        let mut values = HashMap::new();
        values.insert("1. open".to_string(), format!("{:.4}", open));
        values.insert("4. close".to_string(), format!("{:.4}", close));
        values.insert("2. high".to_string(), format!("{:.4}", high.max(0.0)));
        values.insert("3. low".to_string(), format!("{:.4}", low.max(0.0)));

        data.insert(format!("{} 09:30:00", date_str), values);
    }

    data
}

fn get_mock_1month_data(start_price: Option<f64>) -> HashMap<String, HashMap<String, String>> {
    let mut data = HashMap::new();
    let mut rng = rand::thread_rng();
    let mut current_price = start_price.unwrap_or(100.0);
    let today = Utc::now().date_naive();

    // Generate trading days over a 30-day calendar span
    for day_offset in 0..30 {
        let date = today.pred_opt().and_then(|d| d.checked_sub_days(chrono::Days::new(day_offset as u64)));
        let date = if let Some(d) = date {
            d
        } else {
            continue;
        };
        let date_str = date.format("%Y-%m-%d").to_string();

        // Skip weekends
        if date.format("%w").to_string() == "0" || date.format("%w").to_string() == "6" {
            continue;
        }

        let open = current_price;
        let delta: f64 = rng.gen_range(-1.0..1.0);
        current_price += delta;
        let close = current_price;
        let high = open.max(close) + rng.gen_range(0.0..0.2);
        let low = open.min(close) - rng.gen_range(0.0..0.2);

        let mut values = HashMap::new();
        values.insert("1. open".to_string(), format!("{:.4}", open));
        values.insert("4. close".to_string(), format!("{:.4}", close));
        values.insert("2. high".to_string(), format!("{:.4}", high.max(0.0)));
        values.insert("3. low".to_string(), format!("{:.4}", low.max(0.0)));

        data.insert(format!("{} 09:30:00", date_str), values);
    }

    data
}

fn get_mock_3month_data(start_price: Option<f64>) -> HashMap<String, HashMap<String, String>> {
    let mut data = HashMap::new();
    let mut rng = rand::thread_rng();
    let mut current_price = start_price.unwrap_or(100.0);
    let today = Utc::now().date_naive();

    for day_offset in 0..90 {
        let date = today.pred_opt().and_then(|d| d.checked_sub_days(chrono::Days::new(day_offset as u64)));
        let date = if let Some(d) = date {
            d
        } else {
            continue;
        };
        let date_str = date.format("%Y-%m-%d").to_string();

        if date.format("%w").to_string() == "0" || date.format("%w").to_string() == "6" {
            continue;
        }

        let open = current_price;
        let delta: f64 = rng.gen_range(-1.0..1.0);
        current_price += delta;
        let close = current_price;
        let high = open.max(close) + rng.gen_range(0.0..0.2);
        let low = open.min(close) - rng.gen_range(0.0..0.2);

        let mut values = HashMap::new();
        values.insert("1. open".to_string(), format!("{:.4}", open));
        values.insert("4. close".to_string(), format!("{:.4}", close));
        values.insert("2. high".to_string(), format!("{:.4}", high.max(0.0)));
        values.insert("3. low".to_string(), format!("{:.4}", low.max(0.0)));

        data.insert(format!("{} 09:30:00", date_str), values);
    }

    data
}

// Mock data for demonstration purposes, replace with actual API call
fn get_mock_alpha_vantage_data(start_price: Option<f64>) -> HashMap<String, HashMap<String, String>> {
    let mut data = HashMap::new();
    let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();
    let mut rng = rand::thread_rng();
    let mut current_price = start_price.unwrap_or(0.0);
    
    // Generate data for each 5-minute interval
    for h in MARKET_OPEN_H..=MARKET_CLOSE_H {
        for m in (0..60).step_by(5) {
            // Skip times before open
            if h == MARKET_OPEN_H && m < MARKET_OPEN_M {
                continue;
            }
            // Stop at close
            if h == MARKET_CLOSE_H && m > MARKET_CLOSE_M {
                break;
            }

            let mut values = HashMap::new();
            let open = current_price;
            let delta: f64 = rng.gen_range(0.1..1.0);
            if rng.gen() {
                current_price += delta;
            } else {
                current_price -= delta;
            }
            let close = current_price;
            let high = open.max(close) + rng.gen_range(0.0..0.1);
            let low = open.min(close) - rng.gen_range(0.0..0.1);

            values.insert("1. open".to_string(), format!("{:.4}", open));
            values.insert("4. close".to_string(), format!("{:.4}", close));
            values.insert("2. high".to_string(), format!("{:.4}", high.max(0.0)));
            values.insert("3. low".to_string(), format!("{:.4}", low.max(0.0)));

            data.insert(format!("{} {:02}:{:02}:00", today, h, m), values);
        }
    }

    data
}
