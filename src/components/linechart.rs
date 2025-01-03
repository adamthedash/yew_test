use yew::{html, Component, Html, Properties};

use crate::plot::bindings::create_chart_js;

#[derive(Default, PartialEq, Clone)]
pub struct LineChartData {
    pub key: String,
    pub title: Option<String>,
    pub y_axis_title: Option<String>,
    pub x_axis_title: Option<String>,
    pub x_data: Vec<String>,
    pub y_data: Vec<f32>,
}

#[derive(Properties, PartialEq)]
pub struct LineChartsListProps {
    pub chart_data: Vec<LineChartData>,
}

pub struct LineChartsList;

impl Component for LineChartsList {
    type Message = ();

    type Properties = LineChartsListProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        // Create canvases which will host the charts
        ctx.props()
            .chart_data
            .iter()
            .map(|chart_data| {
                html! {
                    <div class="chart-container">
                    <canvas id={format!("chart-{}", chart_data.key)} class="chart"></canvas>
                    </div>
                }
            })
            .collect()
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        // Render the charts
        ctx.props().chart_data.iter().for_each(|plot_data| {
            create_chart_js(format!("chart-{}", plot_data.key).as_str(), plot_data);
        });
    }
}
