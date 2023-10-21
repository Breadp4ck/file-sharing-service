use leptos::*;

#[component]
pub fn ListPage() -> impl IntoView {
    view! {
        <div class="flex flex-col my-16">
            <h1 class="mx-auto mb-8 fg font-bold text-4xl">"File is protected"</h1>
            <input type="password" placeholder="Write secret code" class="
                mx-auto text-center w-96 h-16 fg bg text-grad text-2xl
                outline-none border-4 border-grad rounded-xl shadow-xl"
            />
        </div>
    }
}
