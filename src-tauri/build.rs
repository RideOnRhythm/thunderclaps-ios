use std::env;
use std::path::PathBuf;

fn main() {
    let cwd: PathBuf = env::current_dir().expect("Failed to get current directory");
    panic!("Current working directory: {}", cwd.display());
    
    println!("cargo:rustc-link-search=framework=/..");
    println!("cargo:rustc-link-lib=framework=TensorFlowLiteC");

    println!("cargo:rerun-if-changed=/../TensorFlowLiteC.framework/Headers/c_api.h");

    let bindings = bindgen::Builder::default()
        .header("/Users/runner/work/thunderclaps-ios/thunderclaps-ios/src-tauri/TensorFlowLiteC.framework/Headers/c_api.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");

    tauri_build::build();
}
