#![allow(unused)]

use leptos::{
    component, create_signal, html::Div, view, Attribute, DynAttrs, IntoView, MaybeProp, NodeRef,
    SignalSet, TextProp,
};
use leptos_fluent::web_sys::SubmitEvent;
use leptos_router::A;
use radix_leptos_label::Label;
use tailwind_fuse::tw_merge;

use crate::components::{
    icon::Icon,
    icons,
    ui::{Button, ButtonVariant, Card},
    PasswordInput,
};

#[component]
pub fn Signup() -> impl IntoView {
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
                    class="mr-2 h-6 w-6"
                >
                    <path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3"></path>
                </svg>
                <h1 class="text-xl font-medium">Shadcn Admin</h1>
            </div>
            <Card class="p-6">
                <div class="mb-2 flex flex-col space-y-2 text-left">
                    <h1 class="text-lg font-semibold tracking-tight">Create an account</h1>
                    <p class="text-sm text-muted-foreground">
                        Enter your email and password to create an account. <br/>
                        Already have an account? {" "}
                        <A
                            href="/auth/login"
                            class="underline underline-offset-4 hover:text-primary"
                        >
                            Sign In
                        </A>
                    </p>
                </div>
                <SignUpForm/>
                <p class="mt-4 px-8 text-center text-sm text-muted-foreground">
                    By creating an account, you agree to our {" "}
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
fn SignUpForm(
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[prop(optional)] node_ref: NodeRef<Div>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let (loading, set_loading) = create_signal(false);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        // TODO: send data to api, handle response, redirect...
    };

    view! {
        <div
            node_ref=node_ref
            class=tw_merge!("grid gap-6", class.with(| c | c.get().to_string()))
            {..attrs}
        >
            <form on:submit=on_submit>
                <div class="grid gap-2">
                    <div class="space-y-1">
                        <Label
                            attr:for="signup-email"
                            attr:class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                        >
                            Email
                        </Label>
                        <input
                            id="signup-email"
                            class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                            type="email"
                            name="email"
                        />
                    </div>
                    <PasswordInput field_name="password" label="Password" placeholder="********"/>
                    <PasswordInput
                        field_name="confirmPassword"
                        label="Confirm Password"
                        placeholder="********"
                    />
                    <Button class="mt-2" loading=loading>
                        Create Account
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
