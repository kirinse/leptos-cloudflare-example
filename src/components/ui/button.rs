use html::AnyElement;
use leptos::*;
use tailwind_fuse::*;

use crate::components::{icon::Icon, icons};

#[allow(unused_variables)]
#[component]
pub fn Button(
    #[prop(into, optional)] variant: MaybeSignal<ButtonVariant>,
    #[prop(into, optional)] size: MaybeSignal<ButtonSize>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(into, optional)] loading: MaybeSignal<bool>,
    #[prop(into, optional)] prepend: MaybeProp<View>,

    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let variant = variant.get();
        let size = size.get();
        let button_class = ButtonClass { variant, size };
        button_class.with_class(class.get())
    });
    // if as_child.get().is_some_and(|a| a) {
    //     return view! {
    //         <Slot {..attrs} node_ref=node_ref attr:class=class>
    //             {children()}
    //         </Slot>
    //     };
    // }
    return view! {
        <button {..attrs} class=class disabled=move || loading.get()>
            {move || {
                if loading.get() {
                    view! { <Icon icon=icons::spinner class="mr-2 size-4 animate-spin"/> }
                } else {
                    prepend.with(|pre| pre.into_view()).into_view()
                }
            }}

            {children()}
        </button>
    }
    .into_view();
}

#[derive(TwClass)]
#[tw(
    class = r#"inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors 
               focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"#
)]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

#[derive(TwVariant)]
pub enum ButtonVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground shadow hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90")]
    Destructive,
    #[tw(
        class = "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground"
    )]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[derive(TwVariant)]
pub enum ButtonSize {
    #[tw(default, class = "h-9 px-4 py-2")]
    Default,
    #[tw(class = "h-8 rounded-md px-3 text-xs")]
    Sm,
    #[tw(class = "h-10 rounded-md px-8")]
    Lg,
    #[tw(class = "size-9")]
    Icon,
}
