extern crate generator;
extern crate tempdir;

use generator::{Context, Generator};
use std::path::Path;

#[test]
fn as_json() {
    let dir = tempdir::TempDir::new("initialization").unwrap();
    let project_path = dir.path();
    let template_path = Path::new("templates");

    let src_file = Path::new("src.txt");
    let dst_file = Path::new("dst.txt");
    let gen = Generator::new(project_path, template_path).unwrap();
    let context = Context::new();

    gen.generate_file(&context, src_file, dst_file).unwrap();
    let generated_path = project_path.cl
}
