use leptos::{Children, IntoView, SignalGet, SignalSet, component, expect_context, view};
use phosphor_leptos::{
    GithubLogo, House, IconWeight, LinkedinLogo, Mailbox, MoonStars, Sun,
};

use crate::DarkThemeCtx;

pub static DRAWER_ID: &str = "drawer";

#[component]
pub fn Drawer(children: Children) -> impl IntoView {
    view! {
        <div class="drawer">
            <input id=DRAWER_ID type="checkbox" class="drawer-toggle"/>
            <div class="flex flex-row justify-center items-center drawer-content">{children()}</div>
            <div class="z-50 drawer-side">
                <label for=DRAWER_ID aria-label="close sidebar" class="drawer-overlay"></label>
                <div class="flex flex-col w-64 h-full bg-base-200 lg:96">
                    <DrawerHeader/>
                    <div class="flex flex-col justify-between h-full">
                        <div class="flex flex-col">
                            <HomeBtn/>
                            <ContactBtn/>
                            <GithubBtn/>
                            <LinkedInBtn/>
                        </div>
                        <div class="flex flex-col">
                            <ThemeControllerBtn/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn DrawerHeader() -> impl IntoView {
    view! {
        <label class="text-xl font-medium btn btn-ghost btn-block font-display">
            {"Stephen Power"}
        </label>
    }
}

#[component]
fn HomeBtn() -> impl IntoView {
    view! {
        <a href="/" class="flex flex-nowrap justify-start items-center btn btn-ghost btn-block">
            <House weight=IconWeight::Duotone class="w-8 h-8 swap-off"/>
            {"Home"}
        </a>
    }
}

#[component]
fn ContactBtn() -> impl IntoView {
    view! {
        <a
            href="/contact"
            class="flex flex-nowrap justify-start items-center btn btn-ghost btn-block"
        >
            <Mailbox weight=IconWeight::Duotone class="w-8 h-8 swap-off"/>
            {"Contact"}
        </a>
    }
}

#[component]
fn GithubBtn() -> impl IntoView {
    view! {
        <a
            target="_blank"
            href="https://github.com/opensourcecheemsburgers"
            class="flex flex-nowrap justify-start items-center btn btn-ghost btn-block"
        >
            <GithubLogo weight=IconWeight::Duotone class="w-8 h-8 swap-off"/>
            {"Github"}
        </a>
    }
}

#[component]
fn LinkedInBtn() -> impl IntoView {
    view! {
        <a
            target="_blank"
            href="https://www.linkedin.com/in/stephen-power-284b18204/"
            class="flex flex-nowrap justify-start items-center btn btn-ghost btn-block"
        >
            <LinkedinLogo weight=IconWeight::Duotone class="w-8 h-8 swap-off"/>
            {"LinkedIn"}
        </a>
    }
}

#[component]
fn ThemeControllerBtn() -> impl IntoView {
    let dark_theme_ctx = expect_context::<DarkThemeCtx>().0;

    view! {
        <button
            on:click=move |_| dark_theme_ctx.set(!dark_theme_ctx.get())
            class="flex flex-nowrap justify-start items-center btn btn-ghost btn-block"
        >
            {move || {
                if dark_theme_ctx.get() {
                    view! {
                        <Sun weight=IconWeight::Duotone class="w-8 h-8"/>
                        {"Light Mode"}
                    }
                } else {
                    view! {
                        <MoonStars weight=IconWeight::Duotone class="w-8 h-8"/>
                        {"Dark Mode"}
                    }
                }
            }}

        </button>
    }
}
