pub type Error = Box<dyn std::error::Error>;
pub use serde::{Serialize,Deserialize};
pub use bincode::{serialize_into,deserialize_from,serialize,deserialize};
pub use math_lib::vec3::*;
pub fn random() -> usize { rand::random() }
