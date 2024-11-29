use leptos::{component, view, IntoView};
use crate::components::ui::{Button, ButtonVariant};

#[component]
pub fn Maintenance() -> impl IntoView {
    view! {
        <div class="h-svh">
            <div class="m-auto flex h-full w-full flex-col items-center justify-center gap-2">
                <h1 class="text-[7rem] font-bold leading-tight">503</h1>
                <span class="font-medium">{"maintenance_title"}</span>
                <p class="text-center text-muted-foreground">{"maintenance_desc"}</p>
                <div class="mt-6 flex gap-4">
                    <Button variant=ButtonVariant::Outline>{"learn_more"}</Button>
                </div>
            </div>
        </div>
    }
}
