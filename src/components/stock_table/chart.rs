use dioxus::prelude::*;
use plotters::prelude::*;
use super::types::StockQuoteData;

#[derive(Clone, Debug, PartialEq)]
struct Candle {
    time: i32,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ChartRange {
    OneDay,
    FiveDay,
    ThreeMonth,
    SixMonth,
}

#[component]
pub fn PriceChart(stock: StockQuoteData) -> Element {
    let symbol = stock.symbol.clone();
    // Use open price if available, else price, else 100.0 as fallback
    let start_price = stock.open.or(stock.price).unwrap_or(100.0);

    let mut selected_range = use_signal(|| ChartRange::OneDay);

    let candles = use_memo(move || {
        let mut data = Vec::new();
        let mut current_price = start_price;

        // Simple pseudo-random generator seeded by symbol to keep chart consistent for the stock
        let mut seed = symbol.bytes().fold(0u64, |acc, x| acc.wrapping_add(x as u64));

        // 9:30 AM = 570 min, 4:00 PM = 960 min
        for t in (570..=960).step_by(15) {
            // Random delta between -1.50 and +1.50
            // LCG: x = (a * x + c) % m
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let rand_float = (seed as f64) / (u64::MAX as f64); // 0.0 to 1.0
            let delta = (rand_float * 3.0) - 1.50;

            let open = current_price;
            let close = current_price + delta;
            let high = open.max(close) + (rand_float * 0.5);
            let low = open.min(close) - (rand_float * 0.5);
            let volume = (rand_float * 10000.0) as u64 + 1000;

            data.push(Candle {
                time: t,
                open,
                high,
                low,
                close,
                volume,
            });

            current_price += delta;
        }
        data
    });

    let mut active_candle = use_signal(|| None::<Candle>);
    let mut mouse_x = use_signal(|| 0.0);

    let width = 800;
    let height = 300;
    let margin = 10;
    let x_label_area = 30;
    let y_label_area = 50;

    let chart_svg = use_memo(move || {
        let data = candles.read();
        if data.is_empty() {
            return String::new();
        }

        let mut buffer = String::new();
        {
            let root = SVGBackend::with_string(&mut buffer, (width as u32, height as u32)).into_drawing_area();
            root.fill(&WHITE).unwrap();

            let min_price = data.iter().map(|c| c.close).fold(f64::INFINITY, f64::min);
            let max_price = data.iter().map(|c| c.close).fold(f64::NEG_INFINITY, f64::max);

            let mut chart = ChartBuilder::on(&root)
                .margin(margin)
                .x_label_area_size(x_label_area)
                .y_label_area_size(y_label_area)
                .build_cartesian_2d(0f64..(data.len() - 1) as f64, (min_price * 0.99)..(max_price * 1.01))
                .unwrap();

            chart.configure_mesh()
                .x_labels(10)
                .y_labels(5)
                .disable_x_mesh()
                .disable_y_mesh()
                .x_label_formatter(&|idx| {
                    if let Some(c) = data.get(*idx as usize) {
                        let h = c.time / 60;
                        let m = c.time % 60;
                        let am_pm = if h >= 12 { "PM" } else { "AM" };
                        let h_12 = if h > 12 { h - 12 } else if h == 0 { 12 } else { h };
                        format!("{:02}:{:02} {}", h_12, m, am_pm)
                    } else {
                        String::new()
                    }
                })
                .draw()
                .unwrap();

            chart.draw_series(
                AreaSeries::new(
                    data.iter().enumerate().map(|(i, c)| (i as f64, c.close)),
                    min_price * 0.99,
                    RGBColor(0, 122, 255).mix(0.1).filled(),
                )
            ).unwrap();

            chart.draw_series(LineSeries::new(
                data.iter().enumerate().map(|(i, c)| (i as f64, c.close)),
                RGBColor(0, 122, 255).stroke_width(2),
            )).unwrap();
        }
        buffer
    });

    rsx! {
        div {
            class: "mt-6 p-4 rounded shadow mr-2.5",
            div {
                class: "mb-2 text-center",
                style: "width: {width}px;",
                h3 { class: "text-lg font-semibold text-gray-700", "Price Movement" }
            }
            div {
                class: "relative select-none",
                style: "width: {width}px; height: {height}px;",
                onmousemove: move |evt| {
                    let coords = evt.element_coordinates();
                    let x = coords.x;

                    // Calculate plot area boundaries based on plotters configuration
                    let plot_x_start = (margin + y_label_area) as f64;
                    let plot_width = (width - (margin * 2) - y_label_area) as f64;

                    let data = candles.read();
                    let data_len = data.len();

                    if data_len > 0 && x >= plot_x_start && x <= (plot_x_start + plot_width) {
                        let relative_x = x - plot_x_start;
                        // Map relative X to index
                        let idx = (relative_x / plot_width * (data_len - 1) as f64).round() as usize;

                        if let Some(c) = data.get(idx) {
                            active_candle.set(Some(c.clone()));
                            // Snap mouse_x to the actual data point X for the vertical line
                            let snapped_x = plot_x_start + (idx as f64 / (data_len - 1) as f64) * plot_width;
                            mouse_x.set(snapped_x);
                        }
                    } else {
                        active_candle.set(None);
                    }
                },
                onmouseleave: move |_| {
                    active_candle.set(None);
                },

                div {
                    dangerous_inner_html: "{chart_svg}"
                }

                if let Some(c) = active_candle.read().as_ref() {
                    {
                        let x = *mouse_x.read();
                        let tooltip_left = if x > (width as f64 / 2.0) { x - 210.0 } else { x + 10.0 };

                        let h = c.time / 60;
                        let m = c.time % 60;
                        let am_pm = if h >= 12 { "PM" } else { "AM" };
                        let h_12 = if h > 12 { h - 12 } else if h == 0 { 12 } else { h };

                        let data = candles.read();
                        let min_price = data.iter().map(|c| c.close).fold(f64::INFINITY, f64::min);
                        let max_price = data.iter().map(|c| c.close).fold(f64::NEG_INFINITY, f64::max);
                        let min_scale = min_price * 0.99;
                        let max_scale = max_price * 1.01;
                        let scale_range = max_scale - min_scale;

                        let plot_height = (height - (margin * 2) - x_label_area) as f64;
                        let plot_top = margin as f64;
                        let y_pos = plot_top + ((max_scale - c.close) / scale_range) * plot_height;

                        rsx! {
                            // Vertical Line
                            div {
                                class: "absolute border-l border-gray-400 border-dashed pointer-events-none",
                                style: "left: {x}px; top: {margin}px; height: {height - margin - x_label_area}px;"
                            }
                            // Dot
                            div {
                                class: "absolute w-3 h-3 bg-blue-600 rounded-full border-2 border-white shadow-sm transform -translate-x-1/2 -translate-y-1/2 pointer-events-none",
                                style: "left: {x}px; top: {y_pos}px;"
                            }
                            // Tooltip
                            div {
                                class: "absolute top-4 bg-white/95 backdrop-blur border border-gray-200 p-3 rounded shadow-lg text-xs pointer-events-none z-10",
                                style: "left: {tooltip_left}px; width: 200px;",
                                div { class: "font-bold mb-2 border-b pb-1", "Today, {h_12}:{m:02} {am_pm}" }
                                div { class: "grid grid-cols-2 gap-1",
                                    span { class: "text-gray-500", "Open" }
                                    span { class: "text-right font-mono", "{c.open:.2}" }
                                    span { class: "text-gray-500", "High" }
                                    span { class: "text-right font-mono", "{c.high:.2}" }
                                    span { class: "text-gray-500", "Low" }
                                    span { class: "text-right font-mono", "{c.low:.2}" }
                                    span { class: "text-gray-500", "Close" }
                                    span { class: "text-right font-mono", "{c.close:.2}" }
                                    span { class: "text-gray-500", "Volume" }
                                    span { class: "text-right font-mono", "{c.volume}" }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "flex justify-start mt-4",
                style: "width: {width}px; padding-left: {margin + y_label_area}px;",
                div {
                    class: "flex space-x-12 text-sm font-medium",
                    for range in [ChartRange::OneDay, ChartRange::FiveDay, ChartRange::ThreeMonth, ChartRange::SixMonth] {
                        {
                            let is_selected = *selected_range.read() == range;
                            let label = match range {
                                ChartRange::OneDay => "1D",
                                ChartRange::FiveDay => "5D",
                                ChartRange::ThreeMonth => "3M",
                                ChartRange::SixMonth => "6M",
                            };
                            if is_selected {
                                rsx! { span { class: "text-gray-900", "{label}" } }
                            } else {
                                rsx! {
                                    a {
                                        class: "text-blue-600 cursor-pointer hover:underline",
                                        onclick: move |_| selected_range.set(range),
                                        "{label}"
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