extern crate cbindgen;

use std::env;
use cbindgen::Config;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir.clone())
      //.with_config(Config::from_root_or_default(Path::new(&crate_dir)))
      .with_config(Config::from_file(Path::new(&crate_dir).join("cbindgen.toml").to_str().unwrap()).unwrap())
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("src/backgammonlogic.h");
}
