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
        self.default
            .as_ref()
            .map_or(std::borrow::Cow::Borrowed("None"), |def_path| {
                path_to_str(def_path)
            })
    }
}

pub struct GetArgs {
    pub extensions: Vec<String>,
    pub json_needed: bool,
}
