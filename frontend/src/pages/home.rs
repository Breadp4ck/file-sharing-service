use leptos::wasm_bindgen::JsCast;
use leptos::*;
use web_sys::{Event, FormData, HtmlInputElement};

#[component]
pub fn HomePage() -> impl IntoView {
    let upload_file = create_action(|file: &web_sys::File| {
        let file = file.to_owned();

        let form_data = FormData::new().unwrap(); // result can't be error
        form_data.append_with_blob("file", &file).unwrap(); // result can't be error

        async move {
            log::info!("Uploading file...");

            let response = gloo::net::http::Request::post("http://localhost:8000/api/files")
                .body(form_data)
                .unwrap() // result can't be error
                .send()
                .await;

            match response {
                Ok(_) => {
                    log::info!("File successfully uploaded!");
                }
                Err(err) => {
                    log::warn!("Error uploading file: {}", err);
                }
            }
        }
    });

    let on_file_input_changed = move |ev: Event| {
        let file_input = ev.target().unwrap().unchecked_into::<HtmlInputElement>(); // can't be error
        let files = file_input.files();

        if let Some(files) = files {
            if let Some(file) = files.item(0) {
                upload_file.dispatch(file);
            }
        }
    };

    view! {
        <div class="flex flex-col my-16">
            <h1 class="mx-auto mb-8 fg font-bold text-5xl">"Simple file sharing"</h1>
            <label for="file-upload" class="
                flex mx-auto w-36 h-16 accent justify-center content-center font-semibold cursor-pointer
                rounded-3xl shadow-xl transition ease-in-out hover:scale-110 active:scale-95 duration-300"
            >
                <input id="file-upload" type="file" class="hidden" on:change=on_file_input_changed />
                <span class="self-center">"Upload file!"</span>
            </label>
        </div>
    }
}
