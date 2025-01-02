#[derive(Default)]
pub struct LineChartData {
    pub title: Option<String>,
    pub y_axis_title: Option<String>,
    pub x_axis_title: Option<String>,
    pub x_data: Vec<String>,
    pub y_data: Vec<f32>,
}
