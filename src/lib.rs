#[macro_use]
extern crate tera;
extern crate libc;

use std::ffi::{CStr};

use tera::Tera;
use tera::Context;

use std::path::PathBuf;
use std::error::Error;
use std::fmt;

pub use generator::*;

mod generator {
	
	use super::*;

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
	pub extern fn generate_file(ptr: *mut Generator, src_path : *const libc::c_char, dst_path : *const libc::c_char) {

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

		generator.generate_file(src_path_str, dst_path_str);
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
	template_path: PathBuf
}

impl Generator {

	fn new (project_path : &str, template_path : &str) -> Self {

	Generator { project_path: PathBuf::from(project_path),
				template_path: PathBuf::from(template_path) 
				}
	}

	fn get_template_path (&self) -> &str {

		self.template_path.to_str().unwrap()
	}

	fn get_project_path (&self) -> &str {

		self.project_path.to_str().unwrap()
	}

	fn generate_file(&self, src_path : &str, dst_path : &str) -> Result<(), GenError>{

		let src_path = PathBuf::from(src_path);
		let dst_path = PathBuf::from(dst_path);

		if !src_path.exists() {

			return Err(GenError::new("Source file doesn't exist"));
			}

		if !src_path.is_file() {

			return Err(GenError::new("Source is not a file"));
			}


		println!("Source file {}", src_path.display());
		println!("Destination file {}", dst_path.display());

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
}
