use fluent_templates::{once_cell::sync::Lazy, static_loader, StaticLoader};
use leptos::*;
use leptos_fluent::{leptos_fluent, tr};
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{Router, Routes, Route};

use crate::pages::{home::Home, errors::NotFound, auth::*, RootLayout};

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
pub fn App() -> impl IntoView {
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
    let formatter = |text: String| format!("{text} — {}", tr!("site-name"));

    provide_meta_context();

    view! {
        <Link rel="icon" href="/pkg/favicon.ico"/>
        <Stylesheet id="stylesheet" href="/pkg/index.css"/>
        <Title formatter/>

        <Router fallback=|| view! {<NotFound/>}>
            <Routes>
                <Route path="/" view=RootLayout>
                    <Route path="" view=Home/>
                </Route>
                <Route path="auth" view=RootLayout>
                    <Route path="/login" view=Login/>
                    <Route path="/signup" view=Signup/>
                </Route>
                // <Route path="/*" view=NotFound/>
            </Routes>
        </Router>
    }
}
