use leptos::*;

#[component]
pub fn ManagePage() -> impl IntoView {
    let (protection, set_protection) = create_signal(false);
    let swap_protection_mode = move |_| set_protection.update(|v| *v = !*v);

    view! {
        <div class="flex flex-row">
            <div class="flex flex-row">
                {move || if protection() {
                    view! {
                        <button on:click=swap_protection_mode class="text-grad material-symbols-outlined">
                            lock
                        </button>
                        <input type="password" placeholder="Set password" class="
                            ml-2 self-center bg-transparant text-grad px-2 border-grad border-2 rounded-3xl outline-none"
                        />
                    }
                } else {
                    view! {
                        <button on:click=swap_protection_mode class="text-grad material-symbols-outlined">
                            lock_open_right
                        </button>
                        <p class="ml-2 self-center text-grad">"Unprotected"</p>
                    }
                }}
            </div>
            <button class="ml-auto mr-0 w-8 h-8 bg-red-500 accent rounded-full">
                <div class="font-light text-2xl rotate-45">"+"</div>
            </button>
        </div>
        <div class="flex flex-col my-16">
            <h1 class="mx-auto mb-8 fg font-bold text-4xl">"filename.rs"</h1>
            <p class="mx-auto fg font-semibold text-xl">"Available till 14/08/23, 12:23 AM"</p>
            <p class="mx-auto mb-8 fg text-xl">"7 days left"</p>

            <h1 class="mx-auto mb-4 fg font-semibold text-3xl">"Share"</h1>
            <div class="mx-auto flex flex-row px-4 py-2 text-xl border-4 border-grad rounded-3xl">
                <input type="url" onClick="this.select();" readonly="readonly"
                    value="http://localhost:80/obtain/" class="
                        flex align-middle mr-2 bg-transparent outline-none
                    "
                />
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
            </div>

            <label for="file-upload" class="
                flex flex-col mx-auto mt-12 w-36 h-16 accent justify-center content-center cursor-pointer
                rounded-3xl shadow-xl transition ease-in-out hover:scale-110 active:scale-95 duration-300"
            >
                <input id="file-upload" type="file" class="hidden"/>
                <p class="self-center text-lg font-semibold">"Download"</p>
                <p class="self-center font-normal">"13.4Mb"</p>
            </label>
        </div>
    }
}
