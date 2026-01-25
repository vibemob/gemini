use crate::components::StockTable;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "p-4",
            id: "trading",
            StockTable {}
        }

    }
}
