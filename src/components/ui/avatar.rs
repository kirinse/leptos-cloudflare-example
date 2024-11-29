use leptos::{component, view, DynAttrs, IntoView, MaybeProp, Signal, SignalGet, TextProp};
use radix_leptos_avatar::{Avatar as AvatarRoot, *};
use tailwind_fuse::tw_merge;

#[component]
pub fn Avatar(
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[prop(into, optional)] src: MaybeProp<String>,
    #[prop(into, optional)] alt: MaybeProp<String>,
) -> impl IntoView {
    let _src = Signal::derive(move || src.get());
    let _alt = Signal::derive(move || alt.get());
    let _fallback =
        Signal::derive(move || _alt.get().map(|a| a.chars().take(2).collect::<String>()));

    view! {
        <AvatarRoot attr:class={
            tw_merge! {
                "relative flex size-10 shrink-0 overflow-hidden rounded-full", class.with(| c | c
                .get().to_string())
            }
        }>
            <AvatarImage
                attr:class="rounded-[inherit] object-cover size-full"
                src=_src
                attr:alt=_alt
            />
            <AvatarFallback
                attr:class="flex size-full items-center justify-center rounded-[inherit] bg-muted"
                delay_ms=600
            >
                {move || _fallback.get()}
            </AvatarFallback>
        </AvatarRoot>
    }
}
