use std::path::Path;

pub fn path_to_str(path: &Path) -> std::borrow::Cow<'_, str> {
    path.file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_else(|| path.to_string_lossy())
}
