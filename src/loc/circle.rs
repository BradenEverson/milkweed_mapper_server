use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Circle {
    longitude: f64,
    latitude: f64,
    radius: f64
}

impl Circle {
    pub fn in_bounds(&self, lon: f64, lat: f64) -> bool {
        todo!();
    }
}
