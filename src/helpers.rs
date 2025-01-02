use wasm_bindgen::prelude::*;

// https://github.com/rustwasm/wasm-bindgen/issues/2491#issuecomment-799676299
#[wasm_bindgen]
extern "C" {
    pub type GeolocationCoordinates;

    #[wasm_bindgen(method, getter)]
    pub fn latitude(this: &GeolocationCoordinates) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn longitude(this: &GeolocationCoordinates) -> f64;

    pub type GeolocationPosition;

    #[wasm_bindgen(method, getter)]
    pub fn coords(this: &GeolocationPosition) -> GeolocationCoordinates;
}

