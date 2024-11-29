use leptos::{component, view, IntoView};
use leptos_router::Outlet;

#[component]
pub fn RootLayout() -> impl IntoView {
    view! {
        <div class="container grid h-svh flex-col items-center justify-center bg-sidebar lg:max-w-none lg:px-0">
            <Outlet/>
        </div>
    }
}
