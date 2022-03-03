use gl_generator::{Api, DebugStructGenerator, Fallbacks, Profile, Registry, StructGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    let registry = Registry::new(
        Api::Gl,
        (4, 5),
        Profile::Core,
        Fallbacks::All,
        ["GL_NV_command_list"],
    );

    match env::var("CARGO_FEATURE_DEBUG") {
        Ok(_) => registry
            .write_bindings(DebugStructGenerator, &mut file_gl)
            .unwrap(),

        Err(_) => registry
            .write_bindings(StructGenerator, &mut file_gl)
            .unwrap(),
    }
    println!("cargo:rerun-if-changed=build.rs");
}
