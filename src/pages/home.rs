use crate::app::GenerateRandomNumber;
use crate::components::ui::{Avatar, Button, ButtonVariant};
use crate::components::{
    // counter_btn::Button,
    icon::Icon,
    icons,
    ui::{Badge, BadgeVariant, H1},
    Loading,
};
use leptos::*;
use leptos_fluent::{expect_i18n, move_tr, Language, tr};
use leptos_meta::Title;

use leptos_router::A;
// use leptos_theme::use_theme;

use leptos_use::{breakpoints_tailwind, use_breakpoints, use_window_size, UseWindowSizeReturn};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let toggle_default = create_rw_signal(false);
    // 2. retrieve the theme_signal global state
    // let theme_signal = use_theme();
    let UseWindowSizeReturn { width, height } = use_window_size();
    let screen_width = use_breakpoints(breakpoints_tailwind());
    let i18n = expect_i18n();

    let get_random = create_server_action::<GenerateRandomNumber>();
    let on_click = move |_| get_random.dispatch(GenerateRandomNumber {});

    create_effect(move |_| {
        logging::log!(
            "home.effect: use_window_size: {}x{}",
            width.get(),
            height.get()
        );
        logging::log!(
            "home.effect: screen_width.is_gt Md: {}",
            screen_width.is_gt(leptos_use::BreakpointsTailwind::Md)
        );
        logging::log!(
            "home.effect: screen_width.is_lt Sm: {}",
            screen_width.is_lt(leptos_use::BreakpointsTailwind::Sm)
        );
    });

    view! {
        <Title text=move || tr!("home")/>

        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <H1 class="mb-3">"Welcome to Leptos"</H1>
                <Button variant=ButtonVariant::Outline on:click=on_click>"Get me a random number"</Button>
                <p>{move || get_random.value()}</p>
                <div class="flex items-center space-x-2 my-4">
                    // <Button/>
                    // <Button increment=5/>
                    <Button
                        variant=Signal::derive(move || {
                            if toggle_default.get() {
                                ButtonVariant::Default
                            } else {
                                ButtonVariant::Secondary
                            }
                        })

                        on:click=move |_| {
                            toggle_default.set(!toggle_default.get());
                        }
                    >

                        Click to change variant
                    </Button>
                </div>
                <div class="flex items-center space-x-2 my-4">
                    <Icon icon=icons::apple width="24" height="24"/>
                    <Icon icon=icons::google width="24" height="24"/>
                    <Loading/>
                    <Badge variant=BadgeVariant::Destructive>Default</Badge>
                </div>
                <div class="flex items-center space-x-2 my-4">
                    <p>{move_tr!("select-a-language")}</p>
                    <fieldset>

                        {move || {
                            i18n.languages
                                .iter()
                                .map(|lang| render_language(lang))
                                .collect::<Vec<_>>()
                        }}

                    </fieldset>

                    <ul>
                        <li>
                            <p>
                                {move_tr!(
                                    "html-tag-lang-is", { "lang" => i18n.language.get().id
                                    .to_string() }
                                )}

                            </p>
                            <p>{move_tr!("add-es-en-url-param")}</p>
                        </li>
                        <li>
                            <p>
                                {move_tr!(
                                    "html-tag-dir-is", { "dir" => i18n.language.get().dir
                                    .to_string() }
                                )}

                            </p>
                        </li>
                    </ul>
                </div>
                // <p class="">Current theme: {move || theme_signal.get().to_string()}</p>
                <div class="flex items-center space-x-2 my-4"></div>
                <div class="flex items-center space-x-2 my-4">
                    <Avatar
                        class="size-12 rounded-full"
                        src="https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?&w=128&h=128&dpr=2&q=80"
                        alt="CN"
                    />
                    <Avatar class="size-12 rounded-full" alt="组之"/>
                    <Avatar class="size-12 rounded-xl" alt="组之"/>
                    <A href="/auth/signup">注册</A>
                    <A href="/no_exists">NoExists</A>
                </div>
            </div>
        </ErrorBoundary>
    }
}

fn render_language(lang: &'static Language) -> impl IntoView {
    // Passed as atrribute, `Language` is converted to their code,
    // so `<input id=lang` becomes `<input id=lang.id.to_string()`
    view! {
        <div>
            <input
                id=lang
                name="language"
                value=lang
                checked=lang.is_active()
                on:click=move |_| lang.activate()
                type="radio"
            />
            <label for=lang>{lang.name}</label>
        </div>
    }
}
