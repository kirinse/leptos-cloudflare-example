use leptos::{component, view, IntoView};
use crate::components::ui::{Button, ButtonVariant};

#[component]
pub fn General() -> impl IntoView {
    view! {
        <div class="h-svh w-full">
            <div class="m-auto flex h-full w-full flex-col items-center justify-center gap-2">
                <h1 class="text-[7rem] font-bold leading-tight">500</h1>
                <span class="font-medium">{"500_title"}</span>
                <p class="text-center text-muted-foreground">{"500_desc"}</p>
                <div class="mt-6 flex gap-4">
                    <Button variant=ButtonVariant::Outline>{"go_back"}</Button>
                    <Button>{"back_to_home"}</Button>
                </div>
            </div>
        </div>
    }
}
