use tera::Tera;
use tera::Context;

use std::path::{Path, PathBuf};
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

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
