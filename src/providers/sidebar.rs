use codee::string::JsonSerdeCodec;
use leptos::{
    component, provide_context, use_context, view, ChildrenFn, Effect, IntoView,
    MaybeProp, Signal, SignalGet, SignalGetUntracked, SignalSet, StoredValue, WriteSignal,
};
use leptos_use::{
    breakpoints_tailwind,
    storage::{use_local_storage_with_options, UseStorageOptions},
    use_breakpoints, BreakpointsTailwind,
};

const STORAGE_KEY: &str = "sidebar_opened";
const SIDEBAR_WIDTH: &str = "16rem";
#[allow(unused)]
const SIDEBAR_WIDTH_MOBILE: &str = "18rem";
const SIDEBAR_WIDTH_ICON: &str = "3rem";
#[allow(unused)]
const SIDEBAR_KEYBOARD_SHORTCUT: &str = "b";

/// The `SidebarProvider` component.
///
/// This component provides a sidebar context to its children, allowing them to access and react to sidebar changes.
///
/// ## Properties
/// * `default_open` - A boolean flag to sync with the system sidebar preference.
///                     Defaults to `true`.
/// * `children` - The child components that will consume the sidebar context.
#[component]
#[allow(unused_braces, unused_variables)]
pub fn SidebarProvider(
    #[prop(optional, into, default = true.into())] default_open: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    // Attempt to retrieve the sidebar from local storage
    let (sidebar_storage_state, set_sidebar_storage_state, _) =
        use_local_storage_with_options::<bool, JsonSerdeCodec>(
            STORAGE_KEY,
            UseStorageOptions::default()
                .listen_to_storage_changes(true)
                // .delay_during_hydration(true),
                .initial_value(default_open.get().unwrap_or_default()),
        );

    let is_mobile = use_breakpoints(breakpoints_tailwind()).lt(BreakpointsTailwind::Sm);

    let stored_state = StoredValue::new(sidebar_storage_state.get_untracked());

    // Update local storage whenever the sidebar state changes
    Effect::new(move |_| {
        if is_mobile.get() {
            stored_state.set_value(sidebar_storage_state.get_untracked());
            set_sidebar_storage_state.set(true);
        } else {
            set_sidebar_storage_state.set(stored_state.get_value());
        }
    });

    provide_context((sidebar_storage_state, set_sidebar_storage_state, is_mobile));
    view! {
        <div
            class="group/sidebar-wrapper flex min-h-svh w-full has-[[data-variant=inset]]:bg-sidebar"
            style=format!(
                "--sidebar-width: {SIDEBAR_WIDTH}; --sidebar-width-icon: {SIDEBAR_WIDTH_ICON}",
            )
        >

            {children()}
        </div>
    }
}

/// Provides the global `Sidebar` state
///
/// This function is used to access the current sidebar state from the global context.
/// The state is wrapped as an `RwSignal`.
pub fn use_sidebar() -> (Signal<bool>, WriteSignal<bool>, Signal<bool>) {
    use_context().expect("there should be a sidebar state")
}
