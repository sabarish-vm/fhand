use crate::Extdata;
use anyhow::Context;
use objc2::rc::Retained;
use objc2_app_kit::NSWorkspace;
use objc2_foundation::NSString;
use objc2_uniform_type_identifiers::UTType;
use std::path::PathBuf;

pub fn get_default_for_type(
    workspace: &NSWorkspace,
    ut_type: &UTType,
) -> Result<Option<PathBuf>, anyhow::Error> {
    match workspace.URLForApplicationToOpenContentType(ut_type) {
        Some(default_url) => {
            if let Some(path) = default_url.to_file_path() {
                Ok(Some(path))
            } else {
                anyhow::bail!(format!("Cannot convert {:?} to PathBuf", default_url))
            }
        }
        None => Ok(None),
    }
}

pub fn get_all_registered_apps_for_type(workspace: &NSWorkspace, ut_type: &UTType) -> Vec<PathBuf> {
    let all_urls = workspace.URLsForApplicationsToOpenContentType(ut_type);
    all_urls
        .iter()
        .filter_map(|x| match x.to_file_path() {
            Some(path) => Some(path),
            None => {
                eprintln!("Warning: skipped non-file application URL: {x:?}");
                None
            }
        })
        .collect::<Vec<PathBuf>>()
}

pub fn ut_type_from_ext(ext: &str) -> Result<Retained<UTType>, anyhow::Error> {
    let clean_ext = ext.rsplit_once('.').map_or(ext, |(_, ext)| ext);
    let ns_ext = NSString::from_str(clean_ext);

    let ut_type = UTType::typeWithFilenameExtension(&ns_ext)
        .with_context(|| format!(".{clean_ext}: Unknown file type"))?;
    Ok(ut_type)
}

pub fn inspect_extension(workspace: &NSWorkspace, ext: &str) -> Result<Extdata, anyhow::Error> {
    let ut_type = ut_type_from_ext(ext)?;

    let default_app_url = get_default_for_type(workspace, &ut_type)?;

    let registered_apps = get_all_registered_apps_for_type(workspace, &ut_type);

    Ok(Extdata {
        ext: ext.to_string(),
        registered: registered_apps,
        default: default_app_url,
    })
}
