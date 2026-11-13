use std::path::PathBuf;

use crate::helper::path_to_str;

pub struct Appdata {
    pub default: Option<PathBuf>,
    pub registered: Vec<PathBuf>,
}

impl Appdata {
    pub fn default_app(&self) -> std::borrow::Cow<'_, str> {
        if let Some(def_path) = &self.default {
            path_to_str(def_path)
        } else {
            std::borrow::Cow::Borrowed("None")
        }
    }
}
