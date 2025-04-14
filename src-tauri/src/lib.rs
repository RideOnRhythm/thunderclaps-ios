// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod tflite {
    #![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

    include!("../bindings.rs");
}

use std::ffi::{c_void, CString};
use base64::Engine;
use tauri::Manager;
use tflite::*;
use base64::prelude::BASE64_STANDARD;
use image::GenericImageView;
use ndarray::{Array3, Array4, Axis};

#[tauri::command]
fn classify(image_string: String) -> String {
    let first_comma_index = image_string.find(",").unwrap();
    let base64_data = image_string.chars()
        .skip(first_comma_index + 1)
        .collect::<String>();

    let image_bytes = BASE64_STANDARD.decode(base64_data).unwrap();
    let image = image::load_from_memory(&image_bytes).unwrap();

    let image = image.resize_exact(299, 299, image::imageops::FilterType::Lanczos3);
    let image = image.to_rgb8();

    let mut array = Array3::<u8>::zeros((299, 299, 3));
    for (x, y, pixel) in image.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        array[(y as usize, x as usize, 0)] = r;
        array[(y as usize, x as usize, 1)] = g;
        array[(y as usize, x as usize, 2)] = b;
    }

    let batched: Array4<u8> = array.insert_axis(Axis(0));
    let mut input_data: Array4<f32> = batched.mapv(|x| x as f32);
    input_data /= 255.0;

    let float_output: &[f32] = unsafe {
        let model_path = CString::new("../nsfw.299x299.tflite").unwrap();
        let model = TfLiteModelCreateFromFile(model_path.as_ptr());
        assert!(!model.is_null());

        let options = TfLiteInterpreterOptionsCreate();
        let interpreter = TfLiteInterpreterCreate(model, options);
        assert!(!interpreter.is_null());

        let status = TfLiteInterpreterAllocateTensors(interpreter);
        assert_eq!(status, TfLiteStatus_kTfLiteOk);

        let input_tensor = TfLiteInterpreterGetInputTensor(interpreter, 0);
        let status = TfLiteTensorCopyFromBuffer(
            input_tensor,
            input_data.as_ptr() as *const c_void,
            input_data.len() * size_of::<f32>()
        );
        assert_eq!(status, TfLiteStatus_kTfLiteOk);

        let status = TfLiteInterpreterInvoke(interpreter);
        assert_eq!(status, TfLiteStatus_kTfLiteOk);

        let output_tensor = TfLiteInterpreterGetOutputTensor(interpreter, 0);
        let output_size = TfLiteTensorByteSize(output_tensor);

        let mut output_buffer = vec![0u8; output_size];
        TfLiteTensorCopyToBuffer(
            output_tensor,
            output_buffer.as_mut_ptr() as *mut c_void,
            output_size,
        );

        std::slice::from_raw_parts(output_buffer.as_ptr() as *const f32, output_size / 4)
    };

    format!("Drawing: {}\nHentai: {}\nNeutral: {}\nPorn: {}\nSexy: {}", float_output[0], float_output[1], float_output[2], float_output[3], float_output[4])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![classify])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
