use leptos::{
    component, create_rw_signal,
    html::{AnyElement, Div, Li, Main, Ul},
    view, Attribute, Callable as _, Callback, Children, DynAttrs as _, DynBindings as _, Effect,
    IntoAttribute, IntoView, MaybeProp, MaybeSignal, NodeRef, Signal, SignalGet, SignalSet,
    SignalWith, TextProp,
};
use leptos_use::use_document;
use radix_leptos_separator::Separator;
use tailwind_fuse::tw_merge;

use crate::components::icons;

use crate::{
    components::{
        icon::Icon,
        icons::{menu, x},
        // layout::{Layout, LayoutHeader},
        ui::{Button, ButtonSize, ButtonVariant},
    },
    providers::use_sidebar,
};

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Position {
    #[default]
    Left,
    Right,
}

impl From<Position> for String {
    fn from(value: Position) -> Self {
        match value {
            Position::Left => "left".into(),
            Position::Right => "right".into(),
        }
    }
}
impl IntoAttribute for Position {
    fn into_attribute(self) -> Attribute {
        let s: String = self.into();
        Attribute::String(s.into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum SidebarCollapsible {
    /// 画面外
    #[default]
    Offcanvas,
    /// 保留图标
    Icon,
    /// 不变?
    None,
}

impl From<SidebarCollapsible> for String {
    fn from(value: SidebarCollapsible) -> Self {
        match value {
            SidebarCollapsible::Offcanvas => "offcanvas".into(),
            SidebarCollapsible::Icon => "icon".into(),
            SidebarCollapsible::None => "none".into(),
        }
    }
}
impl IntoAttribute for SidebarCollapsible {
    fn into_attribute(self) -> Attribute {
        let s: String = self.into();
        Attribute::String(s.into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

impl From<SidebarVariant> for String {
    fn from(value: SidebarVariant) -> Self {
        match value {
            SidebarVariant::Sidebar => "sidebar".into(),
            SidebarVariant::Floating => "floating".into(),
            SidebarVariant::Inset => "inset".into(),
        }
    }
}
impl IntoAttribute for SidebarVariant {
    fn into_attribute(self) -> Attribute {
        let s: String = self.into();
        Attribute::String(s.into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

#[component]
#[allow(unused)]
pub fn Sidebar(
    #[prop(into, optional)] side: MaybeSignal<Position>,
    #[prop(into, optional)] variant: MaybeSignal<SidebarVariant>,
    #[prop(into, optional)] collapsible: MaybeSignal<SidebarCollapsible>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    let (sidebar_state, set_sidebar_state, is_mobile) = use_sidebar();

    let mobile_opened = create_rw_signal(false);

    Effect::new(move |_| {
        let body = use_document().as_ref().unwrap().body().unwrap();
        if mobile_opened.get() {
            let _ = body.class_list().add_1("overflow-hidden");
        } else {
            let _ = body.class_list().remove_1("overflow-hidden");
        }
    });

    let mobile_toggle = Signal::derive(
        move || view! { <Icon width="24" height="24" icon=if mobile_opened.get() { x } else { menu }/> },
    );

    let toggle_icon_class = Signal::derive(move || {
        if !sidebar_state.get() {
            "size-5 rotate-180"
        } else {
            "size-5"
        }
    });

    if collapsible.with(|c| *c == SidebarCollapsible::None) {
        return view! {
            <div {..attrs} class=class node_ref=node_ref>
                {children()}
            </div>
        };
    }

    // TODO: if is_mobile {
    // <Sheet open=...>
    // ...
    // </Sheet>
    // }
    view! {
        <div
            node_ref=node_ref
            class="group peer hidden md:block text-sidebar-foreground"
            attr:data-variant=move || variant.get()
            attr:data-state=move || {
                sidebar_state.with(|s| if *s { "expanded" } else { "collapsed" })
            }

            attr:data-collapsible=move || {
                sidebar_state.with(|s| if *s { String::new() } else { collapsible.get().into() })
            }

            attr:data-side=move || side.get()
        >

            // This is what handles the sidebar gap on desktop
            <div class=tw_merge!(
                "duration-200 relative h-svh w-[--sidebar-width] bg-transparent transition-[width] ease-linear",
                "group-data-[collapsible=offcanvas]:w-0", "group-data-[side=right]:rotate-180",
                (move || { if variant.with(| v | * v == SidebarVariant::Floating || * v ==
                SidebarVariant::Inset) {
                "group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)_+_theme(spacing.4))]"
                } else { "group-data-[collapsible=icon]:w-[--sidebar-width-icon]" } }) ()
            )></div>
            <div
                {..attrs}
                class=tw_merge!(
                    "duration-200 fixed inset-y-0 z-10 hidden h-svh w-[--sidebar-width] transition-[left,right,width] ease-linear md:flex",
                    (move || { if side.with(| s | * s == Position::Left) {
                    "left-0 group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]"
                    } else {
                    "right-0 group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]"
                    } }) (), (move || { if variant.with(| v | * v == SidebarVariant::Floating || * v
                    == SidebarVariant::Inset) {
                    "p-2 group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)_+_theme(spacing.4)_+2px)]"
                    } else {
                    "group-data-[collapsible=icon]:w-[--sidebar-width-icon] group-data-[side=left]:border-r group-data-[side=right]:border-l"
                    } }) (),
                )
            >

                <div
                    data-sidebar="sidebar"
                    class="flex h-full w-full flex-col bg-sidebar group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:border group-data-[variant=floating]:border-sidebar-border group-data-[variant=floating]:shadow"
                >
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SidebarHeader(
    #[prop(optional, into)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[allow(unused)]
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            data-sidebar="header"
            {..attrs}
            class=tw_merge!("flex flex-col gap-2 p-2", class.with(| c | c.get().to_string()))
        >

            {children()}
        </div>
    }
}

#[component]
pub fn SidebarFooter(
    #[prop(optional, into)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[allow(unused)]
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            data-sidebar="footer"
            {..attrs}
            class=tw_merge!("flex flex-col gap-2 p-2", class.with(| c | c.get().to_string()))
        >

            {children()}
        </div>
    }
}

#[component]
pub fn SidebarSeparator(
    #[prop(optional, into)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
    view! {
        <Separator
            node_ref=node_ref
            attr:data-sidebar="separator"
            {..attrs}
            attr:class=tw_merge!(
                "mx-2 w-auto bg-sidebar-border", class.with(| c | c.get().to_string())
            )
        />
    }
}

#[component]
pub fn SidebarContent(
    #[prop(optional, into)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[allow(unused)]
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            data-sidebar="content"
            {..attrs}
            class=tw_merge!(
                "flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=icon]:overflow-hidden",
                class.with(| c | c.get().to_string())
            )
        >

            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroup(
    #[prop(optional, into)] class: MaybeProp<TextProp>,
    #[allow(unused)]
    #[prop(optional)] node_ref: NodeRef<Div>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            data-sidebar="group"
            {..attrs}
            class=tw_merge!(
                "relative flex w-full min-w-0 flex-col p-2", class.with(| c | c.get().to_string())
            )
        >

            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroupContent(
    #[prop(optional, into)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[allow(unused)]
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            data-sidebar="group-conent"
            {..attrs}
            class=tw_merge!("w-full text-sm", class.with(| c | c.get().to_string()))
        >

            {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenu(
    #[prop(optional, into)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[allow(unused)]
    #[prop(optional)] node_ref: NodeRef<Ul>,
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            node_ref=node_ref
            data-sidebar="menu"
            {..attrs}
            class=tw_merge!(class.with(| c | c.get().to_string()))
        >

            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuItem(
    #[prop(optional, into)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[allow(unused)]
    #[prop(optional)] node_ref: NodeRef<Li>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            node_ref=node_ref
            data-sidebar="menu-item"
            {..attrs}
            class=tw_merge!("group/menu-item relative", class.with(| c | c.get().to_string()))
        >

            {children()}
        </li>
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SidebarMenuButtonClass {
    pub variant: SidebarMenuButtonVariant,
    pub size: SidebarMenuButtonSize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SidebarMenuButtonVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SidebarMenuButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

#[allow(unused)]
#[component]
pub fn SidebarMenuButton(
    #[prop(into, optional, default=SidebarMenuButtonVariant::Default.into())] variant: MaybeProp<
        SidebarMenuButtonVariant,
    >,
    #[prop(into, optional, default=SidebarMenuButtonSize::Default.into())] size: MaybeProp<
        SidebarMenuButtonSize,
    >,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] is_active: MaybeProp<bool>,
) -> impl IntoView {
    view! {}
}

#[allow(unused)]
#[component]
pub fn SidebarInset(
    #[prop(optional, into)] class: MaybeProp<TextProp>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] node_ref: NodeRef<Main>,
    children: Children,
) -> impl IntoView {
    view! {
        <main
            {..attrs}
            node_ref=node_ref
            class=tw_merge!(
                "relative flex min-h-svh flex-1 flex-col bg-background",
                "peer-data-[variant=inset]:min-h-[calc(100svh-theme(spacing.4))] md:peer-data-[variant=inset]:m-2 md:peer-data-[state=collapsed]:peer-data-[variant=inset]:ml-2 md:peer-data-[variant=inset]:ml-0 md:peer-data-[variant=inset]:rounded-xl md:peer-data-[variant=inset]:shadow",
                class.with(| c | c.get().to_string())
            )
        >

            {children()}
        </main>
    }
}

#[component]
pub fn SidebarTrigger(
    #[prop(into, optional)] class: MaybeProp<TextProp>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(into, optional)] on_click: MaybeProp<Callback<leptos_fluent::web_sys::MouseEvent>>,
) -> impl IntoView {
    let (state, set_state, ..) = use_sidebar();
    view! {
        <Button
            attr:data-sidebar="trigger"
            node_ref=node_ref
            variant=ButtonVariant::Ghost
            size=ButtonSize::Icon
            class=tw_merge!("size-7", class.with(| c | c.get().to_string()))
            on:click=move |ev| {
                if let Some(on_click) = on_click.get() {
                    on_click.call(ev);
                }
                set_state.set(!state.get());
            }
        >

            {move || {
                state
                    .with(|s| {
                        if *s {
                            view! { <Icon icon=icons::panel_left_close width="24" height="24"/> }
                        } else {
                            view! { <Icon icon=icons::panel_left_open width="24" height="24"/> }
                        }
                    })
            }}

        </Button>
    }
}
