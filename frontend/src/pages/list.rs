use leptos::*;

#[component]
pub fn ListPage() -> impl IntoView {
    view! {
        <div class="flex flex-col my-16">
            <h1 class="mx-auto fg font-semibold text-4xl">"Shared files"</h1>
            <ul class="mt-8 space-y-4">
                <ListItem
                    filename="kek.rs".to_string()
                    expired_at=chrono::NaiveDateTime::from_timestamp_millis(1662921288000).unwrap()
                />
            </ul>
        </div>
    }
}

#[component]
fn ListItem(
    #[prop(into)] filename: String,
    #[prop(into)] expired_at: chrono::NaiveDateTime,
) -> impl IntoView {
    view! {
        <li class="flex justify-between py-2 px-4 rounded-3xl border-4 border-grad">
            <div class="flex flex-col">
                <h2 class="fg font-semibold text-lg">{filename}</h2>
                <p class="fg">{format!("Available till {}", expired_at)}</p>
            </div>
            <div class="flex flex-row">
                <a href="http://kek.lol/obtain/adskldas" target="_blank" class="self-center text-grad material-symbols-outlined">
                    open_in_new
                </a>
                <button on:click=move |_|
                    match window().navigator().clipboard() {
                        Some(clipboard) => {
                            let _ = clipboard.write_text("http://localhost:80/obtain/");
                        }
                        None => log::warn!("Can't copy to clipboard: there is no keyboard."),
                    }
                class="text-grad material-symbols-outlined">
                    content_copy
                </button>
                <button class="">"Delete"</button>
            </div>
        </li>
    }
}
