use leptos::{IntoView, SignalGet, SignalSet, component, expect_context, view};
use phosphor_leptos::{
    GithubLogo, IconWeight, LinkedinLogo, List, Mailbox, MoonStars, Sun,
};

use crate::{components::DRAWER_ID, DarkThemeCtx, CONTACT_PAGE_LINK};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="navbar data-[theme=rustytube]:bg-base-300 data-[theme=winter]:bg-base-100 px-0">
            <NavbarStart/>
            <NavbarCenter/>
            <NavbarEnd/>
        </div>
    }
}

#[component]
fn NavbarStart() -> impl IntoView {
    view! {
        <div class="navbar-start">
            <label class="btn btn-ghost" for=DRAWER_ID>
                <List weight=IconWeight::Duotone class="w-8 h-8 base-content"/>
            </label>
        </div>
    }
}

#[component]
fn NavbarCenter() -> impl IntoView {
    view! {
        <div class="hidden md:flex navbar-center">
            <a href="/" class="text-xl font-semibold normal-case btn btn-ghost">
                Stephen Power
            </a>
        </div>
    }
}

#[component]
fn NavbarEnd() -> impl IntoView {
    view! {
        <div class="navbar-end">
            <div class="flex lg:gap-2">
                <ContactBtn/>
                <GithubBtn/>
                <div class="hidden lg:block">
                    <LinkedInBtn/>
                </div>
                <ThemeControllerBtn/>
            </div>
        </div>
    }
}

#[component]
fn ContactBtn() -> impl IntoView {
    view! {
        <div class="lg:tooltip lg:tooltip-bottom" data-tip="Contact Me">
            <a class="btn btn-ghost" href=CONTACT_PAGE_LINK>
                <Mailbox weight=IconWeight::Duotone class="w-8 h-8 base-content"/>
            </a>
        </div>
    }
}

#[component]
fn GithubBtn() -> impl IntoView {
    view! {
        <div class="lg:tooltip lg:tooltip-bottom" data-tip="My Github">
            <a
                class="btn btn-ghost"
                href="https://github.com/opensourcecheemsburgers"
                target="_blank"
            >
                <GithubLogo weight=IconWeight::Duotone class="w-8 h-8 base-content"/>
            </a>
        </div>
    }
}

#[component]
fn LinkedInBtn() -> impl IntoView {
    view! {
        <div class="lg:tooltip lg:tooltip-bottom" data-tip="My LinkedIn">
            <a
                class="btn btn-ghost"
                href="https://www.linkedin.com/in/stephen-power-284b18204/"
                target="_blank"
            >
                <LinkedinLogo weight=IconWeight::Duotone class="w-8 h-8 base-content"/>
            </a>
        </div>
    }
}

#[component]
fn ThemeControllerBtn() -> impl IntoView {
    let dark_theme_ctx = expect_context::<DarkThemeCtx>().0;

    view! {
        <div class="lg:tooltip lg:tooltip-bottom" data-tip="Theme">
            <button
                on:click=move |_| dark_theme_ctx.set(!dark_theme_ctx.get())
                class="btn btn-ghost"
            >
                {move || {
                    if dark_theme_ctx.get() {
                        view! { <Sun weight=IconWeight::Duotone class="w-8 h-8"/> }
                    } else {
                        view! { <MoonStars weight=IconWeight::Duotone class="w-8 h-8"/> }
                    }
                }}

            </button>
        </div>
    }
}
