use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}
