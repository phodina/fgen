extern crate cheddar;

fn main() {
	
	println!("Generating header for generator ...");
    cheddar::Cheddar::new().expect("could not read manifest")
        .module("generator").expect("malformed module path")
        .run_build("include/generator.h");
}
