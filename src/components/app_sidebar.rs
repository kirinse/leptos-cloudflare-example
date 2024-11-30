use leptos::{component, view, Attribute, DynBindings, IntoView};
use leptos_fluent::move_tr;
use leptos_router::A;

use crate::components::ui::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarHeader, SidebarMenu,
    SidebarMenuItem, SidebarVariant,
};

#[component]
pub fn AppSidebar(#[prop(attrs)] attrs: Vec<(&'static str, Attribute)>) -> impl IntoView {
    view! {
        <Sidebar {..attrs} variant=SidebarVariant::Inset collapsible=SidebarCollapsible::Icon>
            <SidebarHeader>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <A href="/auth/login">{move_tr!("login")}</A>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarHeader>
            <SidebarContent>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <A href="/stories">故事</A>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarContent>
            <SidebarFooter>""</SidebarFooter>
        </Sidebar>
    }
}
