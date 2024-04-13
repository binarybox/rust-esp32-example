use std::env;

// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(val) = env::var("TARGET") {
        match val.as_str() {
            // include the proper forwarding of cfg args and link args to top level cargo from esp-idf compilations
            // note: cpp-example component (stack and PiCon-MIO-specific OD definitions) are included in esp-idf-sys compilation
            // other native C-dependencies for esp32 target (e.g. IO-channel API) are also included as extra components in esp-idf-sys compilation
            // see configuration of extra components in Cargo.toml
            "xtensa-esp32-espidf" => {
                embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
                embuild::build::LinkArgs::output_propagated("ESP_IDF")?;
            }
            // include compilation of native C dependencies under linux (cpp_example stack)
            // TBD: Also include required C-API for simulated IO-channels? Probably not needed, only to be defined on rust level (no C-API)
            _ => {
                println!(
                    "cargo:warning=Tring to build canoopen stack lib now with OUT_DIR={:#?}!",
                    env::var("OUT_DIR")?
                );
                //
                // caopen library (including generated headers for PiCon_MIO which is pulled in via a git submodule)
                cc::Build::new()
                    .file("library/src/hello_world.cpp")
                    .include("library/lib")
                    .define("C99", None)
                    .static_flag(true)
                    .warnings(true)
                    .opt_level(2)
                    .compile("cpp_example");

                //
                // generate rust bindigs for cpp_example library
                //
                // Tell cargo to look for libraries in the specified directories
                println!("cargo:rustc-link-search={}", env::var("OUT_DIR")?);
                // Tell cargo to tell rustc to link the libcanopen.a library.
                println!("cargo:rustc-link-lib=cpp_example");
                // Tell cargo to invalidate the built crate whenever the wrapper changes
                println!("cargo:rerun-if-changed=library/bindings.h");
                // The bindgen::Builder is the main entry point
                // to bindgen, and lets you build up options for
                // the resulting bindings.
                let bindings = bindgen::Builder::default()
                    // The input header we would like to generate
                    // bindings for.
                    .header("library/bindings.h")
                    // Tell cargo to invalidate the built crate whenever any of the
                    // included header files changed.
                    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                    // set clang arguments for include search paths
                    .clang_arg("-Ilibrary/lib")
                    // Finish the builder and generate the bindings.
                    .generate()
                    // Unwrap the Result and panic on failure.
                    .expect("Unable to generate bindings");

                // Write the bindings to the $OUT_DIR/bindings.rs file.
                // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
                bindings
                    .write_to_file("src/bindings.rs")
                    .expect("Couldn't write bindings!");
            }
        }
    }
    Ok(())
}
