use leptos::{component, view, IntoView};
use crate::components::ui::{Button, ButtonVariant};

#[component]
pub fn Unauthorised() -> impl IntoView {
    view! {
        <div class="h-svh">
            <div class="m-auto flex h-full w-full flex-col items-center justify-center gap-2">
                <h1 class="text-[7rem] font-bold leading-tight">401</h1>
                <span class="font-medium">{"unauthorised_title"}</span>
                <p class="text-center text-muted-foreground">{"unauthorised_desc"}</p>
                <div class="mt-6 flex gap-4">
                    <Button variant=ButtonVariant::Outline>{"go_back"}</Button>
                    <Button>{"back_to_home"}</Button>
                </div>
            </div>
        </div>
    }
}
