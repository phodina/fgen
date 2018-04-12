error_chain! {

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Tera(::tera::Error);
    }

    errors {
        ProjectDirErr(d: String) {
            description("Invalid project dir")
            display("Invalid project dir: '{}'", d)
        }

        TemplateDirErr(d: String) {
            description("Invalid template dir")
            display("Invalid template dir: '{}'", d)
        }

        SrcPathErr(d: String) {
            description("Invalid src path")
            display("Invalid src path: '{}'", d)
        }

        DstPathErr(d: String) {
            description("Invalid dst path")
            display("Invalid dst path: '{}'", d)
        }
    }
}
