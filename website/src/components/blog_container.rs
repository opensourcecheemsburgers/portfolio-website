use highlight_pulldown::PulldownHighlighter;
use leptos::{component, expect_context, view, IntoView, Signal, SignalGet};
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};
use syntect_assets::assets::HighlightingAssets;

use crate::DarkThemeCtx;

#[component]
pub fn BlogContainer(markdown: String) -> impl IntoView {
    let highlighted_markdown_html = Signal::derive(move || {
        let parser = pulldown_cmark::Parser::new_ext(
            &markdown,
            pulldown_cmark::Options::all(),
        );

        let mut theme_set = ThemeSet::new();
        let assets = HighlightingAssets::from_binary();
        let dracula = assets.get_theme("Dracula");
        theme_set.themes.insert("Dracula".to_string(), dracula.clone());

        let highlighter = PulldownHighlighter {
            syntaxset: SyntaxSet::load_defaults_nonewlines(),
            themeset: theme_set,
            theme: "Dracula".to_string(),
        };
        let events = highlighter.highlight(parser);

        let mut contents = String::new();
        if let Ok(events) = events {
            pulldown_cmark::html::push_html(&mut contents, events.into_iter());
        }
        contents
    });

    let theme = Signal::derive(move || {
        if expect_context::<DarkThemeCtx>().0.get() {
            "rustytube"
        } else {
            "corporate"
        }
    });

    view! {
        <article
            data-theme=theme
            inner_html=highlighted_markdown_html
            class="max-w-full bg-transparent xl:max-w-5xl prose-slate prose prose-sm prose-a:no-underline prose-code:select-all [&>pre]:select-all [&>pre]:mockup-code [&>pre]:bg-netural [&>pre]:text-neutral-content sm:prose-base md:prose-lg lg:prose-xl xl:prose-2xl"
        >

            {}
        </article>
    }
}
