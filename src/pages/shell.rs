use leptos::{component, view, DynAttrs, IntoView};
use leptos_router::Outlet;
use radix_leptos_separator::{Orientation, Separator};

use crate::{
    components::{
        // scroll_area::ScrollArea,
        ui::{SidebarInset, SidebarTrigger},
        AppSidebar,
    },
    providers::SidebarProvider,
};

#[component]
pub fn Shell() -> impl IntoView {
    view! {
        <SidebarProvider>
            <AppSidebar/>
            <SidebarInset class="w-full overflow-hidden">
                <div class="sticky top-0 z-10">
                    <header class="flex h-14 w-full shrink-0 items-center justify-between border-b bg-background/80 px-2 backdrop-blur-sm sm:h-16 sm:px-4">
                        <div class="flex items-center gap-2">
                            <SidebarTrigger class="-ml-0.5 sm:-ml-1"/>
                            <Separator
                                orientation=Orientation::Vertical
                                attr:class="shrink-0 bg-border w-px mr-2 hidden h-4 sm:block"
                            />
                            <div className="hidden sm:flex"></div>
                        </div>
                        // <Search/>
                        // <Link to="https://github.com/TinsFox/shadcnui-boilerplate" target="_blank">
                        // <Button variant="ghost" size="icon">
                        // <Icons.gitHub className="size-5" />
                        // </Button>
                        // </Link>
                        // <Link to="https://shadcnui-boilerplate.pages.dev" target="_blank">
                        // <Button variant="ghost" size="icon">
                        // <CircleHelp className="size-5" />
                        // </Button>
                        // </Link>
                        // <ThemeSwitcher />
                        // <ThemeCustomizer />
                        <div class="ml-auto flex flex-1 items-center space-x-2 px-2 sm:px-4 md:max-w-96 lg:max-w-lg"></div>
                    </header>
                </div>
                // <ScrollArea class="flex h-[calc(100vh-5rem)] flex-col gap-4 p-2 pt-0 sm:h-[calc(100vh-5rem)] sm:p-4">
                <div class="p-2 sm:py-4">
                    <Outlet/>
                </div>
            // </ScrollArea>
            </SidebarInset>
        </SidebarProvider>
    }
}

#[component]
pub fn Full() -> impl IntoView {
    view! {
        <div class="container grid h-svh flex-col items-center justify-center bg-sidebar lg:max-w-none lg:px-0">
            <Outlet/>
        </div>
    }
}
