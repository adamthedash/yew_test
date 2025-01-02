use std::collections::HashMap;

use crate::plot::line::LineChartData;

use super::generic::XMLItem;

use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::Client;

/// Fetches the weather forecast data from MET Eireann
pub async fn get_weather(base_url: &str, latitude: f64, longitude: f64) -> Result<String, String> {
    let url = format!(
        "{}/locationforecast?lat={:.6};long={:.6}",
        base_url, latitude, longitude
    );

    let client = Client::new();
    client
        .get(&url)
        .send()
        .await
        .map_err(|err| format!("Failed to fetch data: {}", err))?
        .text()
        .await
        .map_err(|err| format!("Failed to read response: {}", err))
}

/// Represents a single measurement for a given time point
#[derive(Debug)]
pub struct FlatItem {
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub name: String,
    pub attributes: HashMap<String, String>,
}

/// Turns semi-raw XML strucure into long-format data
pub fn flatten_response(root: &XMLItem) -> Vec<FlatItem> {
    // weatherdata
    //      product
    //          time to=X, from=X
    //              location
    //                  measurement k=v

    root.children
        .iter()
        .find(|x| x.name == "product")
        .expect("Failed to get 'product' xml element")
        .children
        .iter()
        .flat_map(|time| {
            time.children
                .first()
                .expect("Timestep is empty!")
                .children
                .iter()
                .map(|measurement| {
                    let to = time
                        .attributes
                        .get("to")
                        .unwrap_or_else(|| panic!("Tag has no 'to' attribute: {:?}", measurement))
                        .parse::<DateTime<Utc>>()
                        .expect("Failed to parse timestamp")
                        .naive_utc();
                    let from = time
                        .attributes
                        .get("from")
                        .unwrap_or_else(|| panic!("Tag has no 'from' attribute: {:?}", measurement))
                        .parse::<DateTime<Utc>>()
                        .expect("Failed to parse timestamp")
                        .naive_utc();

                    FlatItem {
                        to,
                        from,
                        name: measurement.name.clone(),
                        attributes: measurement.attributes.clone(),
                    }
                })
        })
        .collect()
}

/// Parses out data into separate measurements for plotting
pub fn prepare_plot_data(items: &[FlatItem]) -> HashMap<String, LineChartData> {
    // Sort by date
    let mut items = items.iter().collect::<Vec<_>>();
    items.sort_by_key(|item| item.from);

    let mut measurement_groups = HashMap::<_, LineChartData>::new();
    items.into_iter().for_each(|item| {
        // Each measurement has it's own format, so grab the value out specifically for each one
        // Some measurements (like precipitation) provide an interval aswell
        let value = match item.name.as_str() {
            "temperature"
            | "precipitation"
            | "globalRadiation"
            | "humidity"
            | "pressure"
            | "dewpointTemperature" => item.attributes.get("value").unwrap().parse().unwrap(),
            // Direction 0-360 degrees
            "windDirection" => item.attributes.get("deg").unwrap().parse().unwrap(),
            "windSpeed" | "windGust" => item.attributes.get("mps").unwrap().parse().unwrap(),
            // High/med/low clouds are % of each type I think?
            "cloudiness" | "lowClouds" | "mediumClouds" | "highClouds" => {
                item.attributes.get("percent").unwrap().parse().unwrap()
            }
            // Symbol is just an index corresponding to some icon Eg. a cloud
            "symbol" => return,
            _ => todo!("Not implemented: {:?}", item),
        };

        measurement_groups
            .entry(item.name.clone())
            .and_modify(|data| {
                data.x_data.push(item.from.to_string());
                data.y_data.push(value);
            })
            .or_insert_with(|| {
                let y_axis_title = match item.name.as_str() {
                    "temperature" | "dewpointTemperature" => "Celcius",
                    "precipitation" => "Millimetres",
                    "windDirection" => "Degrees",
                    "windSpeed" | "windGust" => "Miles per Hour",
                    "globalRadiation" => "Watts per m^2",
                    "humidity" | "cloudiness" | "lowClouds" | "mediumClouds" | "highClouds" => {
                        "Percent"
                    }
                    "pressure" => "hPa",
                    _ => todo!("Not implemented: {:?}", item),
                };

                LineChartData {
                    title: Some(item.name.clone()),
                    y_axis_title: Some(y_axis_title.to_string()),
                    ..Default::default()
                }
            });
    });

    measurement_groups
}
