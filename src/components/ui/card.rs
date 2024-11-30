use leptos::{
    component, create_memo,
    html::{Div, H3, P},
    view, Attribute, Children, IntoView, MaybeProp, NodeRef, TextProp,
};
use tailwind_fuse::tw_merge;

#[component]
pub fn Card(
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[allow(unused)]
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "rounded-xl border bg-card text-card-foreground shadow",
            class.with(|c| c.get().to_string())
        )
    });

    view! {
        <div {..attrs} node_ref=node_ref class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[allow(unused)]
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex flex-col space-y-1.5 p-6",
            class.with(|c| c.get().to_string())
        )
    });

    view! {
        <div {..attrs} node_ref=node_ref class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[allow(unused)]
    #[prop(optional)]
    node_ref: NodeRef<H3>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "font-semibold leading-none tracking-tight",
            class.with(|c| c.get().to_string())
        )
    });

    view! {
        <h3 {..attrs} node_ref=node_ref class=class>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardDescription(
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[allow(unused)]
    #[prop(optional)]
    node_ref: NodeRef<P>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "text-sm text-muted-foreground",
            class.with(|c| c.get().to_string())
        )
    });
    view! {
        <p {..attrs} node_ref=node_ref class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn CardContent(
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[allow(unused)]
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("p-6 pt-0", class.with(|c| c.get().to_string())));

    view! {
        <div {..attrs} node_ref=node_ref class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[allow(unused)]
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex items-center p-6 pt-0",
            class.with(|c| c.get().to_string())
        )
    });

    view! {
        <div {..attrs} node_ref=node_ref class=class>
            {children()}
        </div>
    }
}
