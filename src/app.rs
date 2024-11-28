use fluent_templates::{once_cell::sync::Lazy, static_loader, StaticLoader};
use leptos::*;
use leptos_fluent::{leptos_fluent, move_tr};
use leptos_meta::{provide_meta_context, Link, Stylesheet};

#[server]
pub async fn generate_random_number() -> Result<f64, ServerFnError> {
    Ok(js_sys::Math::random())
}

static_loader! {
  static TRANSLATIONS = {
      locales: "./locales",
      fallback_language: "en",
  };
}
pub static COMPOUND: &[&Lazy<StaticLoader>] = &[&TRANSLATIONS, &TRANSLATIONS];

#[component]
pub fn hello_world() -> impl IntoView {
    leptos_fluent! {
        translations: [TRANSLATIONS, TRANSLATIONS] + COMPOUND,
        locales: "./locales",
        languages: "./locales/languages.json",
        // check_translations: "./src/**/*.rs",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lang",
        cookie_attrs: "SameSite=Strict; Secure; path=/; max-age=600",
        initial_language_from_cookie: true,
        initial_language_from_cookie_to_localstorage: true,
        set_language_to_cookie: true,
        url_param: "lang",
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_localstorage: true,
        initial_language_from_url_param_to_cookie: true,
        // set_language_to_url_param: true,
        localstorage_key: "language",
        initial_language_from_localstorage: true,
        initial_language_from_localstorage_to_cookie: true,
        set_language_to_localstorage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_localstorage: true,
        initial_language_from_accept_language_header: true,
    };

    provide_meta_context();

    let get_random = create_server_action::<GenerateRandomNumber>();
    let on_click = move |_| get_random.dispatch(GenerateRandomNumber {});

    view! {
        <Link rel="icon" href="/pkg/favicon.ico"/>
        <Stylesheet id="" href="/pkg/index.css"/>

        <h1 class="text-xl font-medium mb-3">"Hello, World! " {move || get_random.value()}</h1>
        {move_tr!("select-a-language")}
        <button
            class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 w-full"
            on:click=on_click
        >
            "Get me a random number"
        </button>
    }
}
