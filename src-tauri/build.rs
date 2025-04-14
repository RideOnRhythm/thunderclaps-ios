fn main() {
    println!(r"cargo:rustc-link-search=native=C:\Users\rideo\RustroverProjects\thunderclaps\");
    println!("cargo:rustc-link-lib=static=tensorflowlite_c");

    println!("cargo:rerun-if-changed=../tflite-dist-2.18.0/tflite-dist/include/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("../tflite-dist-2.18.0/tflite-dist/include/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");

    tauri_build::build();
}
