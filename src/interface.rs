use std::ffi::CStr;

use libc;
use std::path::Path;

use std::result::Result::{Err, Ok};
use tera::Context;

use generator::Generator;

pub mod generator {

    use super::*;

    #[repr(C)]
    pub struct CGenerator(Generator);
    #[repr(C)]
    pub struct CContext(Context);

    #[no_mangle]
    pub extern "C" fn context_new() -> *mut CContext {
        Box::into_raw(Box::new(CContext(Context::new())))
    }

    #[no_mangle]
    pub extern "C" fn context_add(
        ptr: *mut CContext,
        key: *const libc::c_char,
        value: *const libc::c_char,
    ) {
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

        context.0.add(&key_str, &value_str);
    }

    #[no_mangle]
    pub extern "C" fn context_free(ptr: *mut CContext) {
        if ptr.is_null() {
            return;
        }
        unsafe {
            Box::from_raw(ptr);
        }
    }

    #[no_mangle]
    pub extern "C" fn generator_new(
        project_path: *const libc::c_char,
        template_path: *const libc::c_char,
    ) -> *mut CGenerator {
        let project_path = unsafe {
            assert!(!project_path.is_null());
            CStr::from_ptr(project_path)
        };

        let project_str = project_path.to_str().unwrap();
        let project_path = Path::new(project_str);

        let template_path = unsafe {
            assert!(!template_path.is_null());
            CStr::from_ptr(template_path)
        };

        let template_str = template_path.to_str().unwrap();
        let template_path = Path::new(template_str);

        Box::into_raw(Box::new(CGenerator(Generator::new(
            project_path,
            template_path,
        ))))
    }

    #[no_mangle]
    pub extern "C" fn generate_file(
        ptr: *mut CGenerator,
        context: *mut CContext,
        src_path: *const libc::c_char,
        dst_path: *const libc::c_char,
    ) {
        let src_path = unsafe {
            assert!(!src_path.is_null());
            CStr::from_ptr(src_path)
        };

        let src_path_str = src_path.to_str().unwrap();
        let src_path = Path::new(src_path_str);

        let dst_path = unsafe {
            assert!(!dst_path.is_null());
            CStr::from_ptr(dst_path)
        };

        let dst_path_str = dst_path.to_str().unwrap();
        let dst_path = Path::new(dst_path_str);

        let generator = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };

        let context = unsafe {
            assert!(!context.is_null());
            &*context
        };

        match generator.0.generate_file(&context.0, src_path, dst_path) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

    #[no_mangle]
    pub extern "C" fn generator_free(ptr: *mut CGenerator) {
        if ptr.is_null() {
            return;
        }
        unsafe {
            Box::from_raw(ptr);
        }
    }

}
