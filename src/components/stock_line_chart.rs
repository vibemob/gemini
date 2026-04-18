use crate::data_models::chart_data::{StockChartData, StockQuoteData};
use crate::services::market_data::{fetch_intraday_data_for_chart, fetch_5day_data_for_chart, fetch_1month_data_for_chart, fetch_3month_data_for_chart, MARKET_CLOSE_H, MARKET_CLOSE_M, MARKET_OPEN_H, MARKET_OPEN_M};
use chrono::{NaiveTime, TimeZone, Utc};
use dioxus::prelude::*;

// Plotters imports
use plotters::prelude::*;
use image::{ImageEncoder, ExtendedColorType};
use base64::{engine::general_purpose::STANDARD, Engine as _};

#[derive(Clone, Copy, PartialEq)]
pub enum ChartTimeRange {
    OneDay,
    FiveDay,
    OneMonth,
    ThreeMonth,
}

// Props for our chart component
#[derive(Props, PartialEq, Clone)]
pub struct StockLineChartProps {
    pub symbol: StockQuoteData,
    pub time_range: ChartTimeRange,
}

/// Stores the necessary coordinate transformation context from plotters
/// to map pixel coordinates back to data coordinates.
#[derive(PartialEq, Clone)]
struct ChartDrawingContext {
    x_range: (chrono::DateTime<Utc>, chrono::DateTime<Utc>),
    y_range: (f64, f64),
    // Plotters uses absolute coordinates for its drawing area.
    // We need the chart's actual pixel boundaries within the canvas
    // to correctly map mouse events.
    plot_left: i32,
    plot_top: i32,
    plot_width: i32,
    plot_height: i32,
}

#[component]
pub fn StockLineChart(props: StockLineChartProps) -> Element {
    let mut chart_data = use_signal(|| None::<StockChartData>);
    let mut hovered_point_idx = use_signal(|| None::<usize>);

    // Fetch data - re-run when symbol changes
    // We read props.symbol inside the effect to establish the dependency
    use_effect(move || {
        let symbol = props.symbol.clone();
        let time_range = props.time_range;
        spawn(async move {
            let fetched_data = match time_range {
                ChartTimeRange::OneDay => fetch_intraday_data_for_chart(&symbol).await,
                ChartTimeRange::FiveDay => fetch_5day_data_for_chart(&symbol).await,
                ChartTimeRange::OneMonth => fetch_1month_data_for_chart(&symbol).await,
                ChartTimeRange::ThreeMonth => fetch_3month_data_for_chart(&symbol).await,
            };
            chart_data.set(fetched_data);
        });
    });

    // Generate chart image and context
    let chart_state = use_memo(move || {
        let time_range = props.time_range;
        chart_data.read().as_ref().and_then(|data| {
            let hovered = *hovered_point_idx.read();
            draw_chart(data, hovered, 800, 400, time_range).ok()
        })
    });

    let on_mouse_move = move |event: Event<MouseData>| {
        if let Some((_, chart_ctx)) = chart_state.read().as_ref() {
            let coords = event.element_coordinates();
            let x_in_canvas = coords.x;
            let y_in_canvas = coords.y;

            // Check if mouse is within the plot area
            if x_in_canvas >= chart_ctx.plot_left as f64
                && x_in_canvas <= (chart_ctx.plot_left + chart_ctx.plot_width) as f64
                && y_in_canvas >= chart_ctx.plot_top as f64
                && y_in_canvas <= (chart_ctx.plot_top + chart_ctx.plot_height) as f64
            {
                // Convert pixel to data coordinates within the plot area
                let x_in_plot_area = x_in_canvas - chart_ctx.plot_left as f64;

                let data_x_millis = chart_ctx.x_range.0.timestamp_millis() as f64
                    + (x_in_plot_area / chart_ctx.plot_width as f64)
                        * (chart_ctx.x_range.1.timestamp_millis()
                            - chart_ctx.x_range.0.timestamp_millis())
                            as f64;

                // Find the closest point by timestamp
                let mut closest_idx = None;
                let mut min_dist = f64::MAX;

                if let Some(data) = chart_data.read().as_ref() {
                    for (i, point) in data.points.iter().enumerate() {
                        let dist = (point.timestamp as f64 - data_x_millis).abs();
                        if dist < min_dist {
                            min_dist = dist;
                            closest_idx = Some(i);
                        }
                    }
                }

                if closest_idx != *hovered_point_idx.read() {
                    hovered_point_idx.set(closest_idx);
                }
            } else {
                // Mouse is outside plot area, clear hover
                if hovered_point_idx.read().is_some() {
                    hovered_point_idx.set(None);
                }
            }
        }
    };

    let on_mouse_leave = move |_: Event<MouseData>| {
        hovered_point_idx.set(None);
    };

    rsx! {
        div {
            class: "stock-line-chart-wrapper",
            if let Some((img_base64, _)) = chart_state.read().as_ref() {
                img {
                    src: "data:image/png;base64,{img_base64}",
                    width: "800",
                    height: "400",
                    style: "border: 1px solid #ddd; background-color: white; display: block;",
                    onmousemove: on_mouse_move,
                    onmouseleave: on_mouse_leave,
                }
            } else {
                div { "Loading chart data..." }
            }
        }
    }
}

/// Draws the stock line chart on the given canvas.
/// Returns a `ChartDrawingContext` for coordinate mapping.
fn draw_chart(
    data: &StockChartData,
    hovered_point_idx: Option<usize>,
    width: u32,
    height: u32,
    time_range: ChartTimeRange,
) -> Result<(String, ChartDrawingContext), Box<dyn std::error::Error>> {
    let mut buffer = vec![0u8; (width * height * 3) as usize];

    // Determine chart title based on time range
    let chart_title = match time_range {
        ChartTimeRange::OneDay => "1-Day",
        ChartTimeRange::FiveDay => "5-Day",
        ChartTimeRange::OneMonth => "1-Month",
        ChartTimeRange::ThreeMonth => "3-Month",
    };

    // Draw chart in a separate scope to drop root_area before using PNG
    let (start_datetime_utc, end_datetime_utc, y_min, y_max, plot_left, plot_top, plot_width, plot_height) = {
        let root_area = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        root_area.fill(&WHITE)?;

        let (start_datetime_utc, end_datetime_utc) = match time_range {
            ChartTimeRange::OneDay => {
                let today_date_naive = Utc::now().date_naive();
                let start_time_local = NaiveTime::from_hms_opt(MARKET_OPEN_H, MARKET_OPEN_M, 0).unwrap();
                let end_time_local = NaiveTime::from_hms_opt(MARKET_CLOSE_H, MARKET_CLOSE_M, 0).unwrap();
                let start = Utc.from_utc_datetime(&today_date_naive.and_time(start_time_local));
                let end = Utc.from_utc_datetime(&today_date_naive.and_time(end_time_local));
                (start, end)
            }
            ChartTimeRange::FiveDay => {
                let today_date_naive = Utc::now().date_naive();
                let start_date = today_date_naive.pred_opt().and_then(|d| d.checked_sub_days(chrono::Days::new(4)));
                let start_date = start_date.unwrap_or(today_date_naive);
                let start_time_local = NaiveTime::from_hms_opt(MARKET_OPEN_H, MARKET_OPEN_M, 0).unwrap();
                let end_time_local = NaiveTime::from_hms_opt(MARKET_CLOSE_H, MARKET_CLOSE_M, 0).unwrap();
                let start = Utc.from_utc_datetime(&start_date.and_time(start_time_local));
                let end = Utc.from_utc_datetime(&today_date_naive.and_time(end_time_local));
                (start, end)
            }
            ChartTimeRange::OneMonth => {
                let today_date_naive = Utc::now().date_naive();
                let start_date = today_date_naive.pred_opt().and_then(|d| d.checked_sub_days(chrono::Days::new(29)));
                let start_date = start_date.unwrap_or(today_date_naive);
                let start_time_local = NaiveTime::from_hms_opt(MARKET_OPEN_H, MARKET_OPEN_M, 0).unwrap();
                let end_time_local = NaiveTime::from_hms_opt(MARKET_CLOSE_H, MARKET_CLOSE_M, 0).unwrap();
                let start = Utc.from_utc_datetime(&start_date.and_time(start_time_local));
                let end = Utc.from_utc_datetime(&today_date_naive.and_time(end_time_local));
                (start, end)
            }
            ChartTimeRange::ThreeMonth => {
                let today_date_naive = Utc::now().date_naive();
                let start_date = today_date_naive.pred_opt().and_then(|d| d.checked_sub_days(chrono::Days::new(89)));
                let start_date = start_date.unwrap_or(today_date_naive);
                let start_time_local = NaiveTime::from_hms_opt(MARKET_OPEN_H, MARKET_OPEN_M, 0).unwrap();
                let end_time_local = NaiveTime::from_hms_opt(MARKET_CLOSE_H, MARKET_CLOSE_M, 0).unwrap();
                let start = Utc.from_utc_datetime(&start_date.and_time(start_time_local));
                let end = Utc.from_utc_datetime(&today_date_naive.and_time(end_time_local));
                (start, end)
            }
        };

        // Find min/max close price for Y-axis scaling
        let min_close = data
            .points
            .iter()
            .map(|p| p.close)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);
        let max_close = data
            .points
            .iter()
            .map(|p| p.close)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(100.0);

        // Add some padding to Y-axis
        let y_padding = (max_close - min_close).max(1.0) * 0.1; // Ensure padding even for flat lines
        let y_min = min_close - y_padding;
        let y_max = max_close + y_padding;

        let margin = 15;
        let x_label_area_size = 30;
        let y_label_area_size = 60;

        let mut chart = ChartBuilder::on(&root_area)
            .margin(margin)
            .caption(chart_title, ("sans-serif", 20))
            .x_label_area_size(x_label_area_size)
            .y_label_area_size(y_label_area_size)
            .build_cartesian_2d(
                start_datetime_utc..end_datetime_utc, // X-axis: Time from 9:30 AM to 4:00 PM
                y_min..y_max,                         // Y-axis: Closing Price range
            )?;

        let x_axis_desc = match time_range {
            ChartTimeRange::OneDay => "Time (9:30 AM - 4:00 PM)",
            ChartTimeRange::FiveDay | ChartTimeRange::OneMonth | ChartTimeRange::ThreeMonth => "Date",
        };

        chart
            .configure_mesh()
            .disable_x_mesh()
            .x_labels(5)
            .y_labels(5)
            .x_label_formatter(&|dt| {
                match time_range {
                    ChartTimeRange::OneDay => dt.format("%H:%M").to_string(),
                    ChartTimeRange::FiveDay | ChartTimeRange::OneMonth | ChartTimeRange::ThreeMonth => dt.format("%m/%d").to_string(),
                }
            })
            .axis_desc_style(TextStyle::from(("sans-serif", 15.0).into_font()))
            .x_desc(x_axis_desc)
            .y_desc("Closing Price")
            .draw()?;

        // Store plot area dimensions for mouse event mapping
        // These values are based on plotters' internal calculation for the actual drawing area
        let plot_left = margin + y_label_area_size;
        let plot_top = margin;
        let plot_width = width as i32 - (margin * 2 + y_label_area_size);
        let plot_height = height as i32 - (margin * 2 + x_label_area_size);

        // Draw the line series based on close price
        chart
            .draw_series(LineSeries::new(
                data.points
                    .iter()
                    .map(|p| (Utc.timestamp_millis_opt(p.timestamp).unwrap(), p.close)),
                &BLUE.mix(0.8),
            ))?
            .label("Close Price")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE.mix(0.8)));

        // Draw normal dots for each point, excluding the hovered one
        chart.draw_series(
            data.points
                .iter()
                .enumerate()
                .filter(|(idx, _)| Some(*idx) != hovered_point_idx)
                .map(|(_, p)| {
                    Circle::new(
                        (Utc.timestamp_millis_opt(p.timestamp).unwrap(), p.close),
                        3,
                        ShapeStyle::from(&BLUE).filled(),
                    )
                }),
        )?;

        // Draw the hovered dot if applicable
        if let Some(idx) = hovered_point_idx {
            if let Some(p) = data.points.get(idx) {
                let point_coord = (Utc.timestamp_millis_opt(p.timestamp).unwrap(), p.close);
                chart.draw_series(PointSeries::of_element(
                    vec![point_coord],
                    5,                                        // Larger size for hover
                    ShapeStyle::from(&RED.mix(0.8)).filled(), // Distinct color for hover
                    &|coord, size, style| Circle::new(coord, size, style),
                ))?;

                // --- Draw Tooltip ---
                // Get the pixel coordinates of the hovered point
                let (x_px, y_px) = chart.backend_coord(&point_coord);
                {
                    let tooltip_lines = vec![
                        format!("Open: ${:.2}", p.open),
                        format!("Close: ${:.2}", p.close),
                        format!("High: ${:.2}", p.high),
                        format!("Low: ${:.2}", p.low),
                    ];

                    let line_height = 15;
                    let char_width = 7;
                    let max_len = tooltip_lines.iter().map(|l| l.len()).max().unwrap_or(0);
                    let box_width = (max_len * char_width + 20) as i32;
                    let box_height = (tooltip_lines.len() * line_height + 10) as i32;

                    let mut box_x = x_px + 15;
                    let mut box_y = y_px - box_height / 2;

                    // Boundary checks
                    if box_x + box_width > width as i32 { box_x = x_px - box_width - 15; }
                    if box_y < 0 { box_y = 5; }
                    if box_y + box_height > height as i32 { box_y = height as i32 - box_height - 5; }

                    // Draw tooltip background
                    root_area.draw(&Rectangle::new(
                        [(box_x, box_y), (box_x + box_width, box_y + box_height)],
                        ShapeStyle::from(&WHITE.mix(0.9)).filled(),
                    ))?;
                    root_area.draw(&Rectangle::new(
                        [(box_x, box_y), (box_x + box_width, box_y + box_height)],
                        ShapeStyle::from(&BLACK).stroke_width(1),
                    ))?;

                    // Draw text
                    for (i, line) in tooltip_lines.iter().enumerate() {
                        root_area.draw(&Text::new(
                            line.to_string(),
                            (box_x + 10, box_y + 5 + i as i32 * line_height as i32),
                            TextStyle::from(("sans-serif", 12.0).into_font()),
                        ))?;
                    }
                }
            }
        }

        root_area.present()?;

        (start_datetime_utc, end_datetime_utc, y_min, y_max, plot_left, plot_top, plot_width, plot_height)
    };


    let mut png_data = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
    encoder.write_image(&buffer, width, height, ExtendedColorType::Rgb8).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let base64_string = STANDARD.encode(&png_data);

    Ok((base64_string, ChartDrawingContext {
        x_range: (start_datetime_utc, end_datetime_utc),
        y_range: (y_min, y_max),
        plot_left,
        plot_top,
        plot_width,
        plot_height,
    }))
}
