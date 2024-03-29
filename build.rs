use std::fs::write;
use std::{env::var, path::Path, sync::Arc};
use swc::{config::JsMinifyOptions, try_with_handler, BoolOrDataConfig, Compiler};
use swc_common::source_map::SourceMap;
use swc_common::GLOBALS;

fn main() {
    #[cfg(all(target_os = "windows", not(debug_assertions)))]
    windows_res();

    compile_js();
}

#[cfg(all(target_os = "windows", not(debug_assertions)))]
fn windows_res() {
    use winres::WindowsResource;

    println!("cargo:rerun-if-changed=assets/volt.ico");

    WindowsResource::new()
        .set_icon("./assets/volt.ico")
        .set("InternalName", "Volt")
        .compile()
        .unwrap();
}

fn compile_js() {
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

    let out_dir = var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("init.js");

    write(dest_path, output.code).unwrap();
}
