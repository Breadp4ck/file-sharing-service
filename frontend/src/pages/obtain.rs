use leptos::*;

#[component]
pub fn ObtainPage() -> impl IntoView {
    view! {
        <div class="flex flex-col my-16">
            <h1 class="mx-auto mb-8 fg font-bold text-4xl">"filename.rs"</h1>
            <p class="mx-auto fg font-semibold text-xl">"Available till 14/08/23, 12:23 AM"</p>
            <p class="mx-auto mb-8 fg text-xl">"7 days left"</p>
            <label for="file-upload" class="
                flex flex-col mx-auto w-36 h-16 accent justify-center content-center cursor-pointer
                rounded-3xl shadow-xl transition ease-in-out hover:scale-110 active:scale-95 duration-300"
            >
                <input id="file-upload" type="file" class="hidden"/>
                <p class="self-center text-lg font-semibold">"Download"</p>
                <p class="self-center font-normal">"13.4Mb"</p>
            </label>
        </div>
    }
}
