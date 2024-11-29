#![allow(unused)]

use leptos::{
    component, create_signal, html::Div, view, Attribute, DynAttrs, IntoView, MaybeProp, NodeRef,
    Signal, SignalGet, SignalSet, SignalWith, TextProp,
};
use leptos_fluent::web_sys::SubmitEvent;
use radix_leptos_label::Label;
use tailwind_fuse::tw_merge;

use crate::components::{
    icon::Icon,
    icons,
    ui::{Button, ButtonSize, ButtonVariant, Card},
};

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="mx-auto flex w-full flex-col justify-center space-y-2 sm:w-[480px] lg:p-8">
            <div class="mb-4 flex items-center justify-center">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="mr-2 size-6"
                >
                    <path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3"></path>
                </svg>
                <h1 class="text-xl font-medium">Shadcn Admin</h1>
            </div>
            <Card class="p-6">
                <div class="flex flex-col space-y-2 text-left">
                    <h1 class="text-2xl font-semibold tracking-tight">Login</h1>
                    <p class="text-sm text-muted-foreground">
                        Enter your email and password below <br/> to log into your account
                    </p>
                </div>
                <UserAuthForm/>
                <p class="mt-4 px-8 text-center text-sm text-muted-foreground">
                    By clicking login, you agree to our {" "}
                    <a href="/terms" class="underline underline-offset-4 hover:text-primary">
                        Terms of Service
                    </a> {" "} and {" "}
                    <a href="/privacy" class="underline underline-offset-4 hover:text-primary">
                        Privacy Policy
                    </a> .
                </p>
            </Card>
        </div>
    }
}

#[component]
pub fn UserAuthForm(
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[prop(optional)] node_ref: NodeRef<Div>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let (eye, set_eye) = create_signal(false);
    let (loading, set_loading) = create_signal(false);

    let eye_button = Signal::derive(move || {
        eye.with(|e| {
            if *e {
                icons::lucide_eye
            } else {
                icons::lucide_eye_off
            }
        })
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        // TODO: send data to api, handle response, redirect...
    };
    view! {
        <div
            _ref=node_ref
            class=tw_merge!("grid gap-6", class.with(| c | c.get().to_string()))
            {..attrs}
        >
            <form on:submit=on_submit method="post">
                <div class="grid gap-2">
                    <div class="space-y-1">
                        <Label attr:class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                            Email
                        </Label>
                        <input
                            class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                            type="email"
                            name="email"
                        />
                    </div>
                    <div class="space-y-1">
                        <div class="flex items-center justify-between">
                            <Label attr:class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                                Password
                            </Label>
                            <a
                                class="text-sm font-medium text-muted-foreground hover:opacity-75"
                                href="/forgot-password"
                            >
                                Forgot password?
                            </a>
                        </div>
                        <div class="relative rounded-md">
                            <input
                                class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                                type=move || eye.with(|e| if *e { "text" } else { "password" })
                                name="password"
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
                    </div>
                    <Button class="mt-2" loading=loading>
                        Login
                    </Button>
                    <div class="relative my-2">
                        <div class="absolute inset-0 flex items-center">
                            <span class="w-full border-t"></span>
                        </div>
                        <div class="relative flex justify-center text-xs uppercase">
                            <span class="bg-background px-2 text-muted-foreground">
                                Or continue with
                            </span>
                        </div>
                    </div>

                    <div class="flex items-center gap-2">
                        <Button
                            variant=ButtonVariant::Outline
                            class="w-full"
                            loading=loading
                            attr:type="button"
                            prepend=view! {
                                <Icon
                                    icon=icons::google
                                    width="24"
                                    height="24"
                                    class="size-4 mr-2"
                                />
                            }
                        >

                            Google
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            class="w-full"
                            loading=loading
                            attr:type="button"
                            prepend=view! {
                                <Icon icon=icons::apple width="24" height="24" class="size-4 mr-2"/>
                            }
                        >

                            Apple
                        </Button>
                    </div>
                </div>

            </form>
        </div>
    }
}
