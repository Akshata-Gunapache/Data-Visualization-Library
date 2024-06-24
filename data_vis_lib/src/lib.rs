use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ChartData {
    data: Vec<f64>,
    labels: Vec<String>,
}

#[wasm_bindgen]
impl ChartData {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<f64>, labels: Vec<String>) -> ChartData {
        ChartData { data, labels }
    }

    pub fn get_data(&self) -> JsValue {
        JsValue::from_serde(&self.data).unwrap()
    }

    pub fn get_labels(&self) -> JsValue {
        JsValue::from_serde(&self.labels).unwrap()
    }
}

#[wasm_bindgen]
pub fn generate_bar_chart(data: &ChartData) -> String {
    let svg = format!(
        r#"<svg width="400" height="200" xmlns="http://www.w3.org/2000/svg">
           <!-- SVG content here using data and labels -->
           </svg>"#
    );
    svg
}
