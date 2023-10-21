use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col my-16">
            <h1 class="mx-auto mb-8 fg font-bold text-5xl">"Simple file sharing"</h1>
            <label for="file-upload" class="
                flex mx-auto w-36 h-16 accent justify-center content-center font-semibold cursor-pointer
                rounded-3xl shadow-xl transition ease-in-out hover:scale-110 active:scale-95 duration-300"
            >
                <input id="file-upload" type="file" class="hidden"/>
                <span class="self-center">"Upload file!"</span>
            </label>
        </div>
    }
}
