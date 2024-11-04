use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;

#[derive(Serialize)]
pub struct ChartSize {
    pub width: u32,
    pub height: u32,
}

impl ChartSize {
    pub fn fullscreen() -> Option<Self> {
        if let Some(window) = web_sys::window() {
            let width = window.inner_width().ok()?.as_f64()? as u32;
            let height = window.inner_height().ok()?.as_f64()? as u32;
            Some(ChartSize { height, width })
        } else {
            None
        }
    }

    pub fn to_value(&self) -> JsValue {
        to_value(self).unwrap()
    }
}
