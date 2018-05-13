use tera::Tera;
use tera::Context;

use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs;

use errors::*;

#[derive(Debug)]
pub struct Generator {
    project_path: PathBuf,
    template_path: PathBuf,
    tera: Tera,
}

impl Generator {
    pub fn new(project_path: &Path, template_path: &Path) -> Result<Generator> {
        if !project_path.exists() {
            return Err(ErrorKind::ProjectDirErr(
                project_path
                    .to_str()
                    .unwrap_or("Project path error")
                    .to_owned(),
            ).into());
        }

        if !template_path.exists() {
            return Err(ErrorKind::TemplateDirErr(
                template_path
                    .to_str()
                    .unwrap_or("Template path error")
                    .to_owned(),
            ).into());
        }

        let template_pattern = template_path.join("**/*");
        let template_pattern_str = template_pattern.to_str().unwrap();

        let tera = compile_templates!(template_pattern_str);

        Ok(Generator {
            project_path: project_path.to_path_buf(),
            template_path: template_path.to_path_buf(),
            tera: tera,
        })
    }

    pub fn get_template_path(&self) -> &Path {
        &self.template_path
    }

    pub fn get_project_path(&self) -> &Path {
        &self.project_path
    }

    pub fn generate_file(&self, context: &Context, src_path: &Path, dst_path: &Path) -> Result<()> {
        let src_path_str = src_path.to_str().ok_or("Invalid src path")?;
        let dst_path_str = dst_path.to_str().ok_or("Invalid dst path")?;

        if src_path_str.is_empty() {
            error!("Invalid src path {} ", src_path_str);
            return Err(ErrorKind::SrcPathErr(src_path_str.to_owned()).into());
        }

        if dst_path_str.is_empty() {
            error!("Generating {} ...", dst_path_str);
            return Err(ErrorKind::DstPathErr(dst_path_str.to_owned()).into());
        }

        let mut dst_exists = self.project_path.clone();
        dst_exists.push(dst_path);

        if !dst_exists.parent().unwrap().exists() {
            info!(
                "Create dirs {} ",
                dst_exists.parent().unwrap().to_str().unwrap()
            );
            fs::create_dir(dst_exists.parent().unwrap()).chain_err(|| "Failed to create dirs")?;
        }

        let result = self.tera
            .render(src_path_str, &context)
            .chain_err(|| format!("Failed to render file {}", src_path_str))?;

        let dst_path_string = match dst_path_str.is_empty() {
            true => self.project_path.join(src_path),
            false => self.project_path.join(dst_path),
        };

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(dst_path_string)
            .chain_err(|| "Failed to open file")?;

        file.write_all(result.as_bytes())?;
        info!("Template {} rendered into {} ", src_path_str, dst_path_str);

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    extern crate tempdir;

    use super::*;
    use std::fs::File;

    #[test]
    fn initialization_ok() {
        let dir = tempdir::TempDir::new("initialization").unwrap();
        let project_path = dir.path();
        let template_path = Path::new("samples");

        let generator = Generator::new(project_path, template_path).unwrap();

        assert_eq!(project_path, generator.get_project_path());
        assert_eq!(template_path, generator.get_template_path());
    }

    #[test]
    fn missing_project_dir() {
        let project_path = Path::new("none");
        let template_path = Path::new("samples");

        let generator_err = Generator::new(project_path, template_path).err().unwrap();

        match generator_err.kind() {
            &ErrorKind::ProjectDirErr(_) => assert!(true),
            &_ => assert!(false, "Expected ProjectDirErr"),
        }
    }

    #[test]
    fn missing_template_dir() {
        let dir = tempdir::TempDir::new("initialization").unwrap();
        let project_path = dir.path();
        let template_path = Path::new("none");

        let generator_err = Generator::new(project_path, template_path).err().unwrap();

        match generator_err.kind() {
            &ErrorKind::TemplateDirErr(_) => assert!(true),
            &_ => assert!(false, "Expected TemplateDirErr"),
        }
    }

    #[test]
    fn missing_src_path() {
        let dir = tempdir::TempDir::new("doesnt_exist").unwrap();
        let project_path = dir.path();
        let template_path = Path::new("samples");
        let src_path = Path::new("");
        let dst_path = Path::new("dst/file.b");

        let context = Context::new();

        let generator = Generator::new(project_path, template_path).unwrap();

        let err = generator
            .generate_file(&context, src_path, dst_path)
            .err()
            .unwrap();

        match err.kind() {
            &ErrorKind::SrcPathErr(_) => (),
            &_ => assert!(false, "Expected SrcPathErr"),
        }
    }

    #[test]
    fn missing_dst_path() {
        let dir = tempdir::TempDir::new("doesnt_exist").unwrap();
        let project_path = dir.path();
        let template_path = Path::new("samples");
        let src_path = Path::new("src/file.a");
        let dst_path = Path::new("");

        let context = Context::new();

        let generator = Generator::new(project_path, template_path).unwrap();

        let err = generator
            .generate_file(&context, src_path, dst_path)
            .err()
            .unwrap();

        match err.kind() {
            &ErrorKind::DstPathErr(_) => (),
            &_ => assert!(false),
        }
    }

    #[test]
    fn render_ok() {
        let dir = tempdir::TempDir::new("render_ok").unwrap();
        let project_path = dir.path();
        let template_path = Path::new("samples");

        let src_path = Path::new("render_ok.txt");
        let dst_path = Path::new("samples_ok.txt");

        let generator = Generator::new(project_path, template_path).unwrap();

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
    fn render_nested_ok() {
        let dir = tempdir::TempDir::new("render_ok").unwrap();
        let project_path = dir.path();
        let template_path = Path::new("samples");

        let src_path = Path::new("render_ok.txt");
        let dst_path = Path::new("nested/samples_ok.txt");

        let generator = Generator::new(project_path, template_path).unwrap();

        let mut context = Context::new();
        context.add("msg", &"Hello World!");

        generator
            .generate_file(&context, src_path, dst_path)
            .unwrap();

        let mut file = File::open(dir.path().join("nested/samples_ok.txt")).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        assert_eq!(content, "Hello World!\n");
    }
}
