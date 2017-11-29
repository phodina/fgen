#[macro_use]
extern crate tera;
extern crate libc;

use std::ffi::{CStr};

use tera::Tera;
use tera::Context;

use std::path::PathBuf;
use std::error::Error;
use std::fmt;
use std::io::prelude::*;
use std::fs::File;

pub use generator::*;

mod generator {
	
	use super::*;

	#[no_mangle]
	pub extern fn context_new () -> *mut Context {

    	Box::into_raw(Box::new(Context::new()))
	}

	#[no_mangle]
	pub extern fn context_add (ptr: *mut Context, key: *const libc::c_char, value: *const libc::c_char) {
		
		let key = unsafe {
            assert!(!key.is_null());
            CStr::from_ptr(key)
        	};

		let key_str = key.to_str().unwrap().to_string();

		let value = unsafe {
            assert!(!value.is_null());
            CStr::from_ptr(value)
        	};

		let value_str = value.to_str().unwrap().to_string();

		let context = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
			};

		context.add(&key_str, &value_str);
	}
	
	#[no_mangle]
	pub extern fn context_free (ptr: *mut Context) {
	
		if ptr.is_null() { return }
	    		unsafe { Box::from_raw(ptr); }
			}

	#[no_mangle]
	pub extern fn generator_new(project_path : *const libc::c_char, template_path : *const libc::c_char) -> *mut Generator {
    
    	let project_path = unsafe {
            assert!(!project_path.is_null());
            CStr::from_ptr(project_path)
        	};

		let project_path_str = project_path.to_str().unwrap();

		let template_path = unsafe {
            assert!(!template_path.is_null());
            CStr::from_ptr(template_path)
        	};

		let template_path_str = template_path.to_str().unwrap();

    	Box::into_raw(Box::new(Generator::new(project_path_str, template_path_str)))
	}

	#[no_mangle]
	pub extern fn generate_file(ptr: *mut Generator, context: *mut Context, src_path : *const libc::c_char, dst_path : *const libc::c_char) {

		let src_path = unsafe {
            assert!(!src_path.is_null());
            CStr::from_ptr(src_path)
        	};

		let src_path_str = src_path.to_str().unwrap();

		let dst_path = unsafe {
            assert!(!dst_path.is_null());
            CStr::from_ptr(dst_path)
        	};

		let dst_path_str = dst_path.to_str().unwrap();

		let generator = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
			};

		let context = unsafe {
            assert!(!context.is_null());
            &*context
			};
		
		generator.generate_file(context, src_path_str, dst_path_str);
	}

	#[no_mangle]
	pub extern fn generator_free(ptr: *mut Generator) {
    	
    	if ptr.is_null() { return }
    		unsafe { Box::from_raw(ptr); }
		}
}

#[derive(Debug)]
struct GenError {
    details: String
}

impl GenError {
    fn new(msg: &str) -> GenError {
        GenError{details: msg.to_string()}
    }
}

impl fmt::Display for GenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for GenError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct Generator {
	project_path:  PathBuf,
	template_path: PathBuf,
	tera: Tera
}

impl Generator {

	fn new (project_path : &str, template_path : &str) -> Self {

    	let tera = compile_templates!("templates/**/*");

		Generator { project_path: PathBuf::from(project_path),
					template_path: PathBuf::from(template_path),
					tera : tera 
					}
	}

	fn get_template_path (&self) -> &str {

		self.template_path.to_str().unwrap()
	}

	fn get_project_path (&self) -> &str {

		self.project_path.to_str().unwrap()
	}

	fn generate_file(&self, context : &Context, src_path : &str, dst_path : &str) -> Result<(), GenError>{

		let mut src_patha = self.template_path.clone();
		let mut dst_patha = self.project_path.clone();
		
		src_patha.push(src_path);
		dst_patha.push(dst_path);

		println!("Source file {}", src_patha.display());
		println!("Destination file {}", dst_patha.display());

		if !src_patha.exists() {

			return Err(GenError::new("Source file doesn't exist"));
			}

		if !src_patha.is_file() {

			return Err(GenError::new("Source is not a file"));
			}

		let result = self.tera.render(src_patha.to_str().unwrap(), &context).unwrap();

		let mut file = match File::create(dst_patha.to_str().unwrap()) {
	        Err(why) => panic!("couldn't create"),
	        Ok(file) => file,
	    	};

	    match file.write_all(result.as_bytes()) {
	        Err(why) => {
	            panic!("couldn't write to")
	        },
	        Ok(_) => println!("successfully wrote to"),
    	}

		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use super::*;

    #[test]
    fn generator_initialization() {

    	let project_path = "./project";
    	let template_path = "./template";

    	let generator = Generator::new(project_path, template_path);

    	assert_eq!(project_path, generator.get_project_path());
    	assert_eq!(template_path, generator.get_template_path());
    }

    #[test]
    fn source_doesnt_exist() {

		let project_path = "./project";
    	let template_path = "./template";
    	let src_path = "src/file.a";
    	let dst_path = "dst/file.b";

		let context = Context::new();

    	let generator = Generator::new(project_path, template_path);

    	assert!(generator.generate_file(&context, src_path, dst_path).is_err());
    }

    #[test]
    fn source_is_directory() {

    }

	#[test]
    fn source_is_empty() {

    }

    #[test]
    fn destination_is_empty() {

    }

	#[test]
    fn source_destination_is_empty() {

    }
}
