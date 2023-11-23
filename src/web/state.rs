extern crate noise;
extern crate rocket;

use crate::map::*;
use rocket::serde::Serialize;
use serde::ser::SerializeStruct;

#[derive(Clone)]
pub struct RocketState {
    pub noises: Vec<Noise>,
    pub seed: u32,
    pub width: usize,
    pub height: usize,
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64,
    pub is_seamless: bool,
}

impl RocketState {
    pub fn new(
        noises: Vec<Noise>,
        seed: u32,
        width: usize,
        height: usize,
        x1: f64,
        x2: f64,
        y1: f64,
        y2: f64,
        is_seamless: bool,
    ) -> Self {
        Self {
            noises,
            seed,
            width,
            height,
            x1,
            x2,
            y1,
            y2,
            is_seamless,
        }
    }
}

impl Serialize for RocketState {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("RocketState", 9)?;
        state.serialize_field("seed", &self.seed)?;
        state.serialize_field("width", &self.width)?;
        state.serialize_field("height", &self.height)?;
        state.serialize_field("x1", &self.x1)?;
        state.serialize_field("x2", &self.x2)?;
        state.serialize_field("y1", &self.y1)?;
        state.serialize_field("y2", &self.y2)?;
        state.serialize_field("is_seamless", &self.is_seamless)?;
        state.end()
    }
}
