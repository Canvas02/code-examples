// From https://sotrh.github.io/learn-wgpu/beginner/tutorial9-models/#accessing-files-in-the-res-folder
// Depends on: anyhow, fs_extra, globsw

use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;

fn main() -> Result<()> {
    // This tells cargo to rerun this script if something in /res/ changes.
    println!("cargo:rerun-if-changed=res/*");

    // + Checks if we use a unified target directory and ajusts accordingly
    let out_dir = {
        if let Ok(target) = env::var("CARGO_TARGET_DIR") {
            PathBuf::from(format!("{}/{}", target, env::var("PROFILE").unwrap()))
        } else {
            PathBuf::from(format!(
                "{}/target/{}",
                env::var("CARGO_MANIFEST_DIR").unwrap(),
                env::var("PROFILE").unwrap()
            ))
        }
    };
    // let out_dir = env::var("OUT_DIR").unwrap();

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut paths_to_copy = Vec::new();
    paths_to_copy.push("res/");
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}
