//! Components module - contains UI components for the application.

mod stock_line_chart;
mod stock_table;

pub use stock_line_chart::{StockLineChart, ChartTimeRange};
pub use stock_table::StockTable;
