use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::helper::path_to_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Extdata {
    pub ext: String,
    pub default: Option<PathBuf>,
    pub registered: Vec<PathBuf>,
}

impl Extdata {
    pub fn default_app(&self) -> std::borrow::Cow<'_, str> {
        if let Some(def_path) = &self.default {
            path_to_str(def_path)
        } else {
            std::borrow::Cow::Borrowed("None")
        }
    }
}

pub struct GetArgs {
    pub extensions: Vec<String>,
    pub json_needed: bool,
}
