use std::fs::write;
use std::{env::var, path::Path, sync::Arc};
use swc::{config::JsMinifyOptions, try_with_handler, BoolOrDataConfig, Compiler};
use swc_common::source_map::SourceMap;
use swc_common::GLOBALS;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/init.js");

    let cm = Arc::<SourceMap>::default();

    let compiler = Compiler::new(cm.clone());

    let output = GLOBALS
        .set(&Default::default(), || {
            try_with_handler(cm.clone(), Default::default(), |handler| {
                let fm = cm
                    .load_file(Path::new("./src/init.js"))
                    .expect("Failed to laod file");

                compiler.minify(
                    fm,
                    handler,
                    &JsMinifyOptions {
                        compress: BoolOrDataConfig::from_bool(true),
                        mangle: BoolOrDataConfig::from_bool(true),
                        keep_fnames: false,
                        ..Default::default()
                    },
                )
            })
        })
        .unwrap();

    // let out_dir = var("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join("init.js");
    // let mut file = File::create(dest_path).expect("Failed to create file");
    // file.write_all(output.code.as_bytes())
    //     .expect("Failed to write to file");

    let out_dir = var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("init.js");

    write(dest_path, output.code).unwrap();
}
