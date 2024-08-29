use leptos::{component, expect_context, view, IntoView, Signal, SignalGet};
use leptos_router::Outlet;

use crate::{
    components::{Drawer, Navbar},
    DarkThemeCtx,
};

#[component]
pub fn Page() -> impl IntoView {
    let dark_theme_ctx = expect_context::<DarkThemeCtx>().0;
    let theme = Signal::derive(move || {
        if dark_theme_ctx.get() {
            "rustytube"
        } else {
            "winter"
        }
    });

    view! {
        <div
            data-theme=theme
            class="flex overflow-y-auto flex-col justify-between items-center w-full min-h-screen data-[theme=rustytube]:bg-base-300 data-[theme=winter]:bg-base-100"
        >
            <Drawer>
                <div class="flex flex-col gap-8 items-center px-2 w-full sm:px-4 lg:px-8 xl:px-16 2xl:px-32 min-[1800px]:px-64 min-[2160px]:px-[28rem]">
                    <Navbar/>
                    <div class="flex flex-col items-center py-16 px-6 space-y-16 w-full">
                        <Outlet/>
                    </div>
                </div>
            </Drawer>
        </div>
    }
}
