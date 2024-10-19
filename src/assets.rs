#[cfg(not(target_family = "wasm"))]
use std::env::current_exe;
use std::path::PathBuf;

pub const ASSET_DIR: &str = "assets";

#[cfg(not(target_family = "wasm"))]
pub fn asset_path() -> PathBuf {
    match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(cargo_dir) => {
            let mut cargo_path = PathBuf::new();
            cargo_path.push(cargo_dir);
            cargo_path.push(ASSET_DIR);
            return cargo_path;
        }
        _ => {}
    };

    match current_exe() {
        Ok(path) => {
            let mut asset_path = path.clone();
            asset_path.pop();
            asset_path.push(ASSET_DIR);
            if asset_path.is_dir() {
                return asset_path;
            }
        }
        _ => {}
    }
    panic!("Asset path doesn't exist")
}

#[cfg(target_family = "wasm")]
pub fn asset_path() -> PathBuf {
    PathBuf::from("assets")
}
