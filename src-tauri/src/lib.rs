use std::io::Cursor;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use image::ImageFormat;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let hue = if delta == 0.0 {
        f32::NAN
    } else if max == r {
        (g - b) / delta
    } else if max == g {
        (b - r) / delta + 2.0
    } else if max == b {
        (r - g) / delta + 4.0
    } else {
        f32::NAN
    };

    let brightness = max;
    let saturation = if brightness == 0.0 {
        0.0
    } else {
        delta / brightness
    };

    (hue, saturation, brightness)
}

fn hsv_to_rgb(hue: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let h = if hue.is_nan() {
        f32::NAN
    } else if hue < 0.0 {
        hue.rem_euclid(6.0) + 6.0
    } else {
        hue.rem_euclid(6.0)
    };

    let a = v * (1.0 - s);
    let b = if h.is_nan() {
        f32::NAN
    } else {
        v * (1.0 - (h - h.floor()) * s)
    };
    let g = if h.is_nan() {
        f32::NAN
    } else {
        v * (1.0 - (1.0 - (h - h.floor())) * s)
    };

    if h.is_nan() {
        (v, v, v)
    } else if 0.0 <= h && h < 1.0 {
        (v, g, a)
    } else if 1.0 <= h && h < 2.0 {
        (b, v, a)
    } else if 2.0 <= h && h < 3.0 {
        (a, v, g)
    } else if 3.0 <= h && h < 4.0 {
        (a, b, v)
    } else if 4.0 <= h && h < 5.0 {
        (g, a, v)
    } else if 5.0 <= h && h < 6.0 {
        (v, a, b)
    } else {
        (0.0, 0.0, 0.0)
    }
}

fn vibrance_down(s: f32, vibrance: f32) -> f32 {
    (vibrance * s).tanh() / (1.0_f32.tanh() + 0.5)
}

#[tauri::command]
fn apply_filters(image_string: String) -> String {
    let first_comma_index = image_string.find(",").unwrap();
    let base64_data = image_string.chars()
        .skip(first_comma_index + 1)
        .collect::<String>();

    let image_bytes = BASE64_STANDARD.decode(base64_data).unwrap();
    let img = image::load_from_memory(&image_bytes).unwrap();
    let mut out = img.to_rgb8();

    for pixel in out.pixels_mut() {
        let [r, g, b] = pixel.0;
        let (mut r, mut g, mut b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);

        // vibrance
        let (h, s, v) = rgb_to_hsv(r, g, b);
        let s = vibrance_down(s, -2.0);
        (r, g, b) = hsv_to_rgb(h, s, v);

        // Clamp and write back
        pixel.0 = [
            (r.clamp(0.0, 1.0) * 255.0) as u8,
            (g.clamp(0.0, 1.0) * 255.0) as u8,
            (b.clamp(0.0, 1.0) * 255.0) as u8,
        ];
    }

    let mut buffer = Cursor::new(Vec::new());
    out.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let encoded = BASE64_STANDARD.encode(buffer.get_ref());

    format!("data:{};base64,{}", "image/png", encoded)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![apply_filters])
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
