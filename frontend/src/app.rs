use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
        <main class="py-16">
            <div class="mx-auto p-8 w-[48rem] bg fg rounded-3xl shadow-2xl">
            <Routes>
                <Route path="" view=  move || view! { <HomePage/> }/>
                <Route path="/list" view=  move || view! { <ListPage/> }/>
                <Route path="/manage" view=  move || view! { <ManagePage/> }/>
                <Route path="/obtain" view=  move || view! { <ObtainPage/> }/>
            </Routes>
            </div>
        </main>
        </Router>
    }
}
