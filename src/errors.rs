error_chain! {

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Tera(::tera::Error);
    }

    errors {
        /*
        InvalidToolchainName(t: String) {
            description("invalid toolchain name")
            display("invalid toolchain name: '{}'", t)
        }*/
    }
}
