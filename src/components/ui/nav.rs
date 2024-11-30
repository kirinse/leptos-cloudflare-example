use leptos::{component, view, IntoView};

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <div class="group border-b bg-background py-2 transition-[max-height,padding] duration-500 data-[collapsed=true]:py-2 md:border-none">
            <nav class="grid gap-1 group-[[data-collapsed=true]]:justify-center group-[[data-collapsed=true]]:px-2"></nav>
        </div>
    }
}
