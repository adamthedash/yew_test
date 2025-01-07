use gloo::console::console;
use gloo::utils::window;
use js_sys::Reflect::{construct, get};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::js_sys;
use web_sys::js_sys::Reflect::set;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::jsgets;
use crate::jsobj;

/// Props for the Google Map Component
#[derive(Properties, PartialEq)]
pub struct GoogleMapProps {
    /// Callback to pass selected coordinates to the parent component
    pub on_location_select: Callback<(f64, f64)>,
}

#[function_component(GoogleMap)]
pub fn google_map(props: &GoogleMapProps) -> Html {
    // Keep track of the map container div
    let map_ref = use_node_ref();

    // Initialize the map
    {
        let map_ref = map_ref.clone();
        let on_location_select = props.on_location_select.clone();

        use_effect_with((), move |_| {
            let map_container = map_ref
                .cast::<HtmlElement>()
                .expect("Map container div not found");

            // Get Google Maps API
            let window = window();

            // Map Options
            let map_options = jsobj! {
                "center": {
                    "lat": 53.362688,
                    "lng": (-6.3111168),
                },
                "zoom": 8,
                "mapId": "DEMO_MAP_ID",
            };

            // Create the map
            // new window.google.maps.Map(map_container, map_options)
            let map = construct(
                &jsgets!(window, "google", "maps", "Map").into(),
                &js_sys::Array::of2(&map_container.into(), &map_options),
            )
            .unwrap();

            // Marker Options
            let marker_options = jsobj! {
                "position": {
                    "lat": 53.362688,
                    "lng": (-6.3111168)
                },
                "map": &map,
                "draggable": true,
            };

            // Create the marker
            // new window.google.maps.Marker(marker_options)
            let marker = construct(
                &jsgets!(window, "google", "maps", "marker", "AdvancedMarkerElement").into(),
                &js_sys::Array::of1(&marker_options),
            )
            .unwrap();

            // Add click Listener
            let marker2 = marker.clone();
            let on_click_closure = Closure::wrap(Box::new(move |event: JsValue| {
                console::log_1(&"Got map clicked event".into());
                console::log_1(&event);

                let position = jsgets!(event, "latLng");
                let lat = jsgets!(position, "lat")
                    .dyn_into::<js_sys::Function>()
                    .unwrap()
                    .call0(&position)
                    .unwrap()
                    .as_f64()
                    .unwrap();
                let lng = jsgets!(position, "lng")
                    .dyn_into::<js_sys::Function>()
                    .unwrap()
                    .call0(&position)
                    .unwrap()
                    .as_f64()
                    .unwrap();

                // Pass coordinates to parent component
                on_location_select.emit((lat + 1., lng));
            }) as Box<dyn Fn(_)>);

            // marker.addListener(dragend, drag_end_closure)
            jsgets!(map, "addListener")
                .dyn_into::<js_sys::Function>()
                .unwrap()
                .call2(
                    &map,
                    &"click".into(),
                    on_click_closure.as_ref().unchecked_ref(),
                )
                .unwrap();

            on_click_closure.forget(); // Keep closure alive
            console::log_1(&"Map initalised".into());

            || ()
        });
    }

    html! {
        <div ref={map_ref} id="map" style="height: 400px; width: 100%;"></div>
    }
}
