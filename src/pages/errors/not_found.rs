use std::ops::Deref;

use leptos::*;
use leptos_fluent::move_tr;
use leptos_router::use_navigate;
use leptos_use::use_window;

use crate::components::ui::{Button, ButtonVariant};

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    let navigate = use_navigate();
    let window = use_window().deref().to_owned();
    view! {
        <div class="h-svh">
            <div class="m-auto flex h-full w-full flex-col items-center justify-center gap-2">
                <h1 class="text-[7rem] font-bold leading-tight">404</h1>
                <span class="font-medium">{move_tr!("not_found_title")}</span>
                <p class="text-center text-muted-foreground">{move_tr!("not_found_description")}</p>
                <div class="mt-6 flex gap-4">
                    <Button
                        variant=ButtonVariant::Outline
                        on:click=move |_| {
                            if let Some(ref win) = window {
                                win.history().expect("window.history should exist").back();
                            }
                        }
                    >

                        {move_tr!("go_back")}
                    </Button>
                    <Button on:click=move |_| navigate(
                        "/",
                        Default::default(),
                    )>{move_tr!("back_to_home")}</Button>
                </div>
            </div>
        </div>
    }
}
