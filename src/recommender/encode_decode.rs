use std::fmt;
use serde::{Serialize, de::DeserializeOwned};
use std::hash::Hash;

use crate::pixie_result::PixieResult;

use super::Recommender;

impl <T: Eq + Hash + fmt::Debug + Serialize + DeserializeOwned> Recommender<T> {
    pub fn to_yaml(&self) -> PixieResult<String> {
        let serialized = serde_yaml::to_string(self)?;
        Ok(serialized)
    }

    pub fn from_yaml(content: &str) -> PixieResult<Recommender<T>> {
        let deserialized = serde_yaml::from_str(content)?;
        Ok(deserialized)
    }
}
