use anyhow::Context;
use objc2_app_kit::NSWorkspace;

use crate::{
    data::{self, Extdata},
    helper::path_to_str,
    macos::inspect_extension,
};

pub fn output_get(ret: Result<data::GetArgs, anyhow::Error>, workspace: &NSWorkspace) {
    match ret {
        Ok(gargs) => {
            let results: Vec<(&String, Result<Extdata, anyhow::Error>)> = gargs
                .extensions
                .iter()
                .map(|ext| (ext, inspect_extension(workspace, ext.as_str())))
                .collect();

            if gargs.json_needed {
                let x = json_output_all(&results);
                if let Err(er) = x {
                    eprintln!("json error : {er}")
                }
            } else {
                for (ext, res) in results {
                    normal_out(ext.as_str(), &res);
                }
            }
        }
        Err(err) => {
            eprintln!(
                "\t Parsing parsing error : {err}. Look at examples or have a look at the usage",
            );
        }
    }
}

fn json_output_all(
    results: &[(&String, Result<Extdata, anyhow::Error>)],
) -> Result<(), anyhow::Error> {
    use std::collections::BTreeMap;

    let mut out = BTreeMap::new();
    for (ext, res) in results {
        match res {
            Ok(data) => {
                out.insert(*ext, serde_json::json!(data));
            }
            Err(e) => {
                out.insert(*ext, serde_json::json!({ "error": e.to_string() }));
            }
        }
    }
    let pretty = serde_json::to_string_pretty(&out).context("Failed to serialize")?;
    println!("{pretty}");
    Ok(())
}

fn normal_out(ext: &str, x: &Result<Extdata, anyhow::Error>) {
    println!("{ext}----------------");
    match x {
        Ok(app) => {
            println!("\tDefault: {}", app.default_app());
            println!("\tRegisered apps:");
            app.registered
                .iter()
                .for_each(|x| println!("\t\t{}", path_to_str(x)));
        }
        Err(err) => {
            eprintln!("\tError while working with extension {ext} : {}", err);
        }
    }
}
