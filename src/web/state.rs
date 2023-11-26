extern crate noise;
extern crate rocket;

use crate::map::data::*;
use rocket::serde::Serialize;
use serde::ser::SerializeStruct;

#[derive(Clone, Debug)]
pub struct RocketState {
    pub noises: Noises,
    pub seed: u32,
    pub chunk: Chunk,
}

impl RocketState {
    pub fn new(noises: Noises, seed: u32, chunk: Chunk) -> Self {
        Self {
            noises,
            seed,
            chunk,
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
        state.serialize_field("width", &self.chunk.width)?;
        state.serialize_field("height", &self.chunk.height)?;
        state.serialize_field("x1", &self.chunk.x1)?;
        state.serialize_field("x2", &self.chunk.x2)?;
        state.serialize_field("y1", &self.chunk.y1)?;
        state.serialize_field("y2", &self.chunk.y2)?;
        state.serialize_field("is_seamless", &self.chunk.is_seamless)?;
        state.end()
    }
}
