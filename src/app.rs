use crate::components::linechart::{LineChartData, LineChartsList};
use wasm_bindgen::JsCast;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window};
use yew::prelude::*;

use crate::{
    helpers::GeolocationPosition,
    xml::{
        generic::parse_xml,
        locationforecast::{flatten_response, get_weather, prepare_plot_data},
    },
};

/// Attemps to fetch the user's geolocation position using the browser API
fn get_geo(position_handle: &UseStateHandle<Option<(f64, f64)>>) {
    let geolocation_handle = window()
        .expect("Failed to get window")
        .navigator()
        .geolocation()
        .expect("Failed to get Geolocation handle");

    let position = position_handle.clone();
    // This closure is compiled into JS (I think?). It's called when we successfully
    // retrieve the user location from the Geolocation browser API
    let geoloc_callback =
        Closure::<dyn Fn(GeolocationPosition)>::new(move |pos: GeolocationPosition| {
            let coords = pos.coords();
            let lat = coords.latitude();
            let lon = coords.longitude();

            console::log_1(&format!("Got location: {:?} {:?}", lat, lon).into());
            if lat != 0. || lon != 0. {
                position.set(Some((lat, lon)));
            }
        });

    use_effect_with_deps(
        move |_| {
            console::log_1(&"hello".into());
            geolocation_handle
                .get_current_position(geoloc_callback.as_ref().unchecked_ref())
                .expect("Geolocation function failed.");
            // We need to leak the closure here so it lives long enough. This should only be
            // called once since we have dependencies == ()
            geoloc_callback.forget();
        },
        (),
    );
}

// This block reaches out to the MET Eireann API to get weather information at the user's
// location. It then post-processes the data into plot-ready data
fn fetch_plot_data(
    base_url: &str,
    position: &UseStateHandle<Option<(f64, f64)>>,
    plot_data: &UseStateHandle<Vec<LineChartData>>,
) {
    let base_url = base_url.to_owned();
    let plot_data = plot_data.clone();
    let position2 = position.clone();
    use_effect_with_deps(
        move |_| {
            if let Some((lat, lon)) = *position2 {
                spawn_local(async move {
                    console::log_1(&format!("Fetching weather at: {:?} {:?}", lat, lon).into());
                    let chart_data = get_weather(base_url.as_str(), lat, lon)
                        .await
                        .map(|xml| parse_xml(&xml))
                        .map(|xml| flatten_response(&xml))
                        .map(|items| prepare_plot_data(&items))
                        .expect("Failed to get weather data");

                    plot_data.clone().set(chart_data)
                });
            }
            || ()
        },
        position.clone(),
    );
}

#[function_component(App)]
pub fn app() -> Html {
    let base_url =
        "https://cors-anywhere.herokuapp.com/http://openaccess.pf.api.met.ie/metno-wdb2ts";

    // todo: for some reason it doesn't like when I put the blocks inside functions :/
    // Get geolocation data
    let position = use_state(|| None);
    {
        let geolocation_handle = window()
            .expect("Failed to get window")
            .navigator()
            .geolocation()
            .expect("Failed to get Geolocation handle");

        let position = position.clone();
        // This closure is compiled into JS (I think?). It's called when we successfully
        // retrieve the user location from the Geolocation browser API
        let geoloc_callback =
            Closure::<dyn Fn(GeolocationPosition)>::new(move |pos: GeolocationPosition| {
                let coords = pos.coords();
                let lat = coords.latitude();
                let lon = coords.longitude();

                console::log_1(&format!("Got location: {:?} {:?}", lat, lon).into());
                if lat != 0. || lon != 0. {
                    position.set(Some((lat, lon)));
                }
            });

        use_effect_with_deps(
            move |_| {
                console::log_1(&"hello".into());
                geolocation_handle
                    .get_current_position(geoloc_callback.as_ref().unchecked_ref())
                    .expect("Geolocation function failed.");
                // We need to leak the closure here so it lives long enough. This should only be
                // called once since we have dependencies == ()
                geoloc_callback.forget();
            },
            (),
        );
    }

    // Fetch weather data & prepare for plotting
    let plot_data = use_state(Vec::new);
    {
        let base_url = base_url.to_owned();
        let plot_data = plot_data.clone();
        let position2 = position.clone();
        use_effect_with_deps(
            move |_| {
                if let Some((lat, lon)) = *position2 {
                    spawn_local(async move {
                        console::log_1(&format!("Fetching weather at: {:?} {:?}", lat, lon).into());
                        let chart_data = get_weather(base_url.as_str(), lat, lon)
                            .await
                            .map(|xml| parse_xml(&xml))
                            .map(|xml| flatten_response(&xml))
                            .map(|items| prepare_plot_data(&items))
                            .expect("Failed to get weather data");

                        plot_data.clone().set(chart_data)
                    });
                }
                || ()
            },
            position.clone(),
        );
    }

    let location_text = if let Some((lat, lon)) = *position.clone() {
        format!("Location: lat={}, lon={}", lat, lon)
    } else {
        "Enable geolocation permissions!".to_string()
    };
    html! {
        <>
            <div>{ location_text }</div>
            <div>
                <LineChartsList chart_data={(*plot_data).clone()} />
            </div>
        </>
    }
}
