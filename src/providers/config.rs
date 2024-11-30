use codee::string::JsonSerdeCodec;
use leptos::{
    component, provide_context, use_context, view, Children, IntoView, Signal, SignalWith,
    WriteSignal,
};
use leptos_use::storage::{use_local_storage_with_options, UseStorageOptions};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    style: String,
    color: String,
    radius: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            style: "new-york".into(),
            color: "zinc".into(),
            radius: 0.5,
        }
    }
}

const STORAGE_KEY: &str = "config";

#[component]
pub fn ConfigProvider(children: Children) -> impl IntoView {
    let (config, set_config, _) = use_local_storage_with_options::<Config, JsonSerdeCodec>(
        STORAGE_KEY,
        UseStorageOptions::default().listen_to_storage_changes(true), // .delay_during_hydration(true),
    );
    provide_context((config, set_config));
    view! {
        <div
            class=move || format!("w-full theme-{}", config.with(|cfg| cfg.color.clone()))
            style=move || format!("--radius: {}rem", config.with(|cfg| cfg.radius))
        >
            {children()}
        </div>
    }
}

/// Provides the global `Config` state
///
/// This function is used to access the current config state from the global context.
/// The state is wrapped as an `RwSignal`.
pub fn use_config() -> (Signal<Config>, WriteSignal<Config>) {
    use_context().expect("there should be a global config state")
}
