use leptos::{
    component, create_memo, create_node_ref, create_signal, ev::scroll, use_context, view,
    Attribute, Children, IntoView, MaybeProp, MaybeSignal, Provider, SignalGet, SignalSet,
    SignalWith, TextProp,
};
use leptos_use::{
    use_event_listener, use_scroll_with_options, ScrollBehavior, UseScrollOptions, UseScrollReturn,
};
use tailwind_fuse::tw_merge;

#[derive(Debug, Default, Clone)]
#[allow(unused)]
pub struct LayoutContext {
    pub offset: MaybeSignal<f64>,
    pub fixed: MaybeSignal<bool>,
}

#[component]
pub fn Layout(
    #[prop(into, optional)] fixed: MaybeSignal<bool>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let (offset, set_offset) = create_signal(0.0);
    let div_ref = create_node_ref();
    let UseScrollReturn { y, .. } = use_scroll_with_options(
        div_ref,
        UseScrollOptions::default().behavior(ScrollBehavior::Smooth),
    );
    let _ = use_event_listener(div_ref, scroll, move |_| set_offset.set(y.get()));

    let class = create_memo(move |_| {
        tw_merge!(
            "h-full overflow-auto",
            class.with(|c| c.get().to_string()),
            fixed.with(|f| if *f { "flex flex-col" } else { "" })
        )
    });

    view! {
        <Provider value=LayoutContext {
            offset: offset.into(),
            fixed,
        }>
            <div {..attributes} _ref=div_ref class=class>
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn LayoutHeader(
    #[prop(into, optional)] sticky: MaybeSignal<bool>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let context = use_context::<LayoutContext>().expect("there should be a global sidebar state");
    let class = create_memo(move |_| {
        tw_merge!(
            "z-10 flex h-[var(--header-height)] items-center gap-4 bg-background p-4 md:px-8",
            class.with(|c| c.get().to_string()),
            if context.offset.get() > 10.0 && sticky.get() {
                "shadow"
            } else {
                "shadow-none"
            },
            if context.fixed.get() { "flex-none" } else { "" },
            if sticky.get() { "sticky top-0" } else { "" }
        )
    });

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn LayoutBody(
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let context = use_context::<LayoutContext>().expect("there should be a global sidebar state");
    let class = create_memo(move |_| {
        tw_merge!(
            "px-4 py-6 md:overflow-hidden md:px-8",
            class.with(|c| c.get().to_string()),
            if context.fixed.get() { "flex-1" } else { "" },
        )
    });

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}
