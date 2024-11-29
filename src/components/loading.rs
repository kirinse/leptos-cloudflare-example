use leptos::{component, view, IntoView};

use crate::components::{icon::Icon, icons};

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center text-sm text-muted-foreground">
            <div class="flex items-center justify-center">
                <Icon icon=icons::spinner class="mr-2 size-4 animate-spin" />
                <span>Loading...</span>
            </div>
        </div>
    }
}
