use leptos::{
    component, create_memo,
    html::{Div, H3, P},
    view, Attribute, Children, IntoView, MaybeProp, NodeRef, TextProp,
};
use tailwind_fuse::tw_merge;

#[component]
pub fn Skeleton(
    #[prop(optional, into)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {

    view! {
        <div {..attrs} class={
            tw_merge! {
                "animate-pulse rounded-md bg-primary/10", class.with(| c | c
                .get().to_string())
            }}></div>
    }
}