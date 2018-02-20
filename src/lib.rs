extern crate libc;
#[macro_use]
extern crate tera;

use std::ffi::CStr;

use tera::Tera;
use tera::Context;

use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;
use std::io::prelude::*;
use std::fs::File;

pub use generator::*;

mod generator {

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

        generator.0.generate_file(&context.0, src_path, dst_path);
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

pub struct Generator {
    project_path: PathBuf,
    template_path: PathBuf,
    tera: Tera,
}

impl Generator {
    pub fn new(project_path: &Path, template_path: &Path) -> Self {
        if !project_path.exists() {
            println!("Project path doesn't exist!");
        }

        if !template_path.exists() {
            println!("Template path doesn't exist");
        }

        let template_pattern = template_path.join("**/*");
        let template_pattern_str = template_pattern.to_str().unwrap();

        let tera = compile_templates!(template_pattern_str);

        Generator {
            project_path: project_path.to_path_buf(),
            template_path: template_path.to_path_buf(),
            tera: tera,
        }
    }

    pub fn get_template_path(&self) -> &Path {
        &self.template_path
    }

    pub fn get_project_path(&self) -> &Path {
        &self.project_path
    }

    pub fn generate_file(
        &self,
        context: &Context,
        src_path: &Path,
        dst_path: &Path,
    ) -> Result<(), Box<Error>> {
        let src_path_str = src_path.to_str().unwrap();
        let result = self.tera.render(src_path_str, &context)?;

        let dst_path_str = self.project_path.join(dst_path);

        let mut file = File::create(dst_path_str)?;

        file.write_all(result.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    extern crate tempdir;

    use super::*;

    #[test]
    fn generator_initialization() {
        let dir = tempdir::TempDir::new("initialization").unwrap();
        let project_path = dir.path();
        let template_path = Path::new("samples");

        let generator = Generator::new(project_path, template_path);

        assert_eq!(project_path, generator.get_project_path());
        assert_eq!(template_path, generator.get_template_path());
    }

    #[test]
    fn source_doesnt_exist() {
        let dir = tempdir::TempDir::new("doesnt_exist").unwrap();
        let project_path = dir.path();
        let template_path = Path::new("samples");
        let src_path = Path::new("src/file.a");
        let dst_path = Path::new("dst/file.b");

        let context = Context::new();

        let generator = Generator::new(project_path, template_path);

        assert!(
            generator
                .generate_file(&context, src_path, dst_path)
                .is_err()
        );
    }

    #[test]
    fn render_ok() {
        let dir = tempdir::TempDir::new("render_ok").unwrap();
        let project_path = dir.path();
        let template_path = Path::new("samples");

        let src_path = Path::new("render_ok.txt");
        let dst_path = Path::new("samples_ok.txt");

        let generator = Generator::new(project_path, template_path);

        let mut context = Context::new();
        context.add("msg", &"Hello World!");

        generator
            .generate_file(&context, src_path, dst_path)
            .unwrap();

        let mut file = File::open(dir.path().join("samples_ok.txt")).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        assert_eq!(content, "Hello World!\n");
    }

    #[test]
    fn source_is_directory() {}

    #[test]
    fn source_is_empty() {}

    #[test]
    fn destination_is_empty() {}

    #[test]
    fn source_destination_is_empty() {}
}
