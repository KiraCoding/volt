use oxc::allocator::Allocator;
use oxc::codegen::{Codegen, CodegenOptions};
use oxc::parser::Parser;
use oxc::span::SourceType;
use oxc::transformer::{TransformOptions, Transformer};
use std::env::var;
use std::fs::{read_to_string, write};
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=src/init.ts");

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
    let allocator = Allocator::default();
    let path = Path::new("./src/init.ts");
    let source_text = read_to_string(path).expect("{name} not found");
    let source_type = SourceType::from_path(path).unwrap();

    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    if !ret.errors.is_empty() {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
        return;
    }

    let mut program = ret.program;

    let transform_options = TransformOptions::default();

    Transformer::new(
        &allocator,
        path,
        source_type,
        &source_text,
        &ret.trivias,
        transform_options,
    )
    .build(&mut program)
    .unwrap();

    let printed = Codegen::<true>::new("", &source_text, CodegenOptions::default())
        .build(&program)
        .source_text;

    let out_dir = var("OUT_DIR").unwrap();
    write(Path::new(&out_dir).join("init.js"), printed).unwrap();
}
