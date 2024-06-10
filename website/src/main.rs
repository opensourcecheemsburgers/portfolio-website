#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::module_name_repetitions)]

mod components;
mod pages;

use components::Page;
use leptos::{component, provide_context, view, window, IntoView, RwSignal};
use leptos_router::{Route, Router, Routes};

use crate::pages::{Contact, Home};

pub static BODY_TEXT: &str =
    "text-pretty font-sans font-regular text-base-content text-lg sm:text-lg \
     md:text-lg lg:text-lg xl:text-xl 2xl:text-xl 3xl:text-xl 4xl:text-xl";
pub static HEADING_TEXT: &str = "text-pretty font-sans font-semibold \
                                 text-base-content text-2xl xl:text-3xl \
                                 2xl:text-4xl 3xl:text-5xl";
pub static TITLE_TEXT: &str =
    "text-pretty font-display font-bold text-primary text-base-content \
     text-[38px] sm:text-[40px] xl:text-5xl 2xl:text-6xl 3xl:text-7xl";

pub static CONTACT_PAGE_LINK: &str = "/contact";

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DarkThemeCtx(RwSignal<bool>);

#[component]
fn App() -> impl IntoView {
    let dark_theme = window()
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
        .map_or(false, |media_query_list| media_query_list.matches());
    let dark_theme_ctx = DarkThemeCtx(RwSignal::new(dark_theme));
    provide_context(dark_theme_ctx);

    view! {
        <Router>
            <Routes>
                <Route path="" view=Page>
                    <Route path="/" view=Home/>
                    <Route path=CONTACT_PAGE_LINK view=Contact/>
                </Route>
            </Routes>
        </Router>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> });
}
