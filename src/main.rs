use objc2_app_kit::NSWorkspace;
use std::env;

mod data;
mod helper;
use crate::{data::Appdata, helper::path_to_str};

#[cfg(target_os = "macos")]
mod macos;

fn main() {
    let workspace = NSWorkspace::sharedWorkspace();
    let args: Vec<String> = env::args().skip(1).collect();

    // Use CLI arguments if provided, otherwise inspect common text/config types
    let extensions: Vec<&str> = if args.is_empty() {
        vec![
            ".txt", ".md", ".toml", "json", "yaml", "rs", "sh", "pdf", "png",
        ]
    } else {
        args.iter().map(String::as_str).collect()
    };

    for ext in extensions {
        use crate::macos::inspect_extension;

        println!("{ext} ----------------");
        let x = inspect_extension(&workspace, ext);
        match x {
            Ok(app) => {
                println!("\tDefault : {}", app.default_app());
                println!("\tRegisered apps :");
                app.registered
                    .iter()
                    .for_each(|x| println!("\t\t{}", path_to_str(x)));
            }
            Err(err) => {
                eprintln!("\tError while working with extension {ext} : {}", err);
            }
        }
    }
}
