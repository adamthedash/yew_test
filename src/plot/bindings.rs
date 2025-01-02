use wasm_bindgen::prelude::*;

use super::line::LineChartData;

// Bind to the JavaScript function for creating the chart
#[wasm_bindgen]
extern "C" {
    fn create_chart(
        parent_element_id: JsValue,
        labels: JsValue,
        data: JsValue,
        title: JsValue,
        y_label: JsValue,
    );
}

pub fn create_chart_js(parent_element_id: &str, data: &LineChartData) {
    let labels = serde_wasm_bindgen::to_value(&data.x_data).unwrap();
    let y_data = serde_wasm_bindgen::to_value(&data.y_data).unwrap();
    let title = data.title.clone().unwrap_or("".to_string());
    let y_label = data.y_axis_title.clone().unwrap_or("".to_string());

    create_chart(
        parent_element_id.into(),
        labels,
        y_data,
        title.into(),
        y_label.into(),
    );
}
