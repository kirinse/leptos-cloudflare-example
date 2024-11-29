use leptos::{
    component, create_memo, create_signal, view, DynAttrs, IntoView, MaybeProp, Signal, SignalGet,
    SignalSet, SignalWith, TextProp, View,
};
use radix_leptos_label::Label;

use crate::components::{
    icon::Icon,
    icons,
    ui::{Button, ButtonSize, ButtonVariant},
};

#[component]
pub fn PasswordInput(
    #[prop(optional, into)] label: MaybeProp<TextProp>,
    #[prop(into)] field_name: String,
    #[prop(optional, into)] placeholder: MaybeProp<TextProp>,
    #[prop(optional, into)] extra: MaybeProp<View>,
) -> impl IntoView {
    let (eye, set_eye) = create_signal(false);
    let eye_button = Signal::derive(move || {
        eye.with(|e| {
            if *e {
                icons::lucide_eye
            } else {
                icons::lucide_eye_off
            }
        })
    });
    let label_el = view! {
        <Label
            // TODO: attr:class:text-destructive if validity error
            attr:class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            attr:for=field_name.clone()
        >
            {label.get()}
        </Label>
    };

    let label_row = create_memo(move |_| {
        if let Some(ex) = extra.get() {
            view! { <div class="flex items-center justify-between">{label_el.clone()} {ex}</div> }
                .into_view()
        } else {
            label_el.clone().into_view()
        }
    });

    view! {
        <div class="space-y-1">
            {move || { label_row.get() }} <div class="relative rounded-md">
                <input
                    class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                    type=move || eye.with(|e| if *e { "text" } else { "password" })
                    name=field_name.clone()
                    id=field_name
                    placeholder=placeholder
                />
                <Button
                    variant=ButtonVariant::Ghost
                    size=ButtonSize::Icon
                    attr:type="button"
                    class="hover:bg-accent hover:text-accent-foreground absolute right-1 top-1/2 size-6 -translate-y-1/2 rounded-md text-muted-foreground"
                    on:click=move |_| set_eye.set(!eye.get())
                >
                    <Icon icon=eye_button width="18" height="18"/>
                </Button>
            </div>
        // TODO: error message: <p id=":r1:-form-item-message" class="text-[0.8rem] font-medium text-destructive">Please enter your password</p>
        </div>
    }
}
