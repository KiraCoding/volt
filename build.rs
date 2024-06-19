use oxc::allocator::Allocator;
use oxc::codegen::WhitespaceRemover;
use oxc::parser::Parser;
use oxc::span::SourceType;
use oxc::transformer::{TransformOptions, Transformer};
use std::env::var;
use std::fs::{read_to_string, write};
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=src/init.ts");

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

    let _ = Transformer::new(
        &allocator,
        path,
        source_type,
        &source_text,
        ret.trivias.clone(),
        transform_options,
    )
    .build(&mut program);

    let printed = WhitespaceRemover::new().build(&program).source_text;

    let out_dir = var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("init.js");
    write(&out_path, printed).unwrap();

    println!("cargo::rustc-env=SCRIPT={}", &out_path.display());
}
