use wasm_bindgen::prelude::*;
use web_sys::{ClipboardEvent, FileReader, MouseEvent};
use leptos::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ClassifyArgs {
    imageString: String,
}

#[component]
pub fn App() -> impl IntoView {
    let image_data = create_signal::<Option<String>>(None);
    let (get_classification, set_classification) = create_signal(String::new());

    let classify = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let image_string = image_data.0.get_untracked().unwrap();

            let args = serde_wasm_bindgen::to_value(&ClassifyArgs { imageString: image_string }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let results = invoke("classify", args).await.as_string().unwrap();
            set_classification.set(results);
        });
    };

    let on_paste = move |event: web_sys::Event| {
        let clipboard_event = event.dyn_ref::<ClipboardEvent>();
        let Some(clipboard_event) = clipboard_event else {
            return;
        };
        let Some(items) = clipboard_event.clipboard_data().and_then(|d| Some(d.items())) else {
            return;
        };

        for i in 0..items.length() {
            let item = items.get(i).unwrap();
            if item.kind() == "file" && item.type_().starts_with("image/") {
                let file = item.get_as_file().unwrap().unwrap();
                let reader = FileReader::new().unwrap();

                let cloned_signal = image_data.clone();
                let onloadend_cb = Closure::wrap(Box::new(move |e: web_sys::ProgressEvent| {
                    let reader = e.target().unwrap().unchecked_into::<FileReader>();
                    if let Ok(result) = reader.result() {
                        if let Some(data_url) = result.as_string() {
                            cloned_signal.1.set(Some(data_url));
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                reader.read_as_data_url(&file).unwrap();
                onloadend_cb.forget();
            }
        }
    };

    view! {
        <main class="container">
            <h1>"Welcome to Tauri + Leptos"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>

        <div class="row">
            <div contenteditable="true" on:paste=on_paste>Paste image here...</div>
        </div>
        <div class="row">
            <button type="button" on:click=classify>Classify</button>
        </div>
        <p>{ move || get_classification.get() }</p>

        </main>
    }
}
