fn main() {
    println!(r"cargo:rustc-link-search=framework=../");
    println!("cargo:rustc-link-lib=framework=TensorFlowLiteC");

    println!("cargo:rerun-if-changed=../TensorFlowLiteC.framework/Headers/c_api.h");

    let bindings = bindgen::Builder::default()
        .header("../TensorFlowLiteC.framework/Headers/c_api.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");

    tauri_build::build();
}
