use leptos::{component, view, Await, IntoView, Params, Signal, SignalGet};
use leptos_router::{use_params, Params};

use crate::components::BlogContainer;

#[derive(Params, PartialEq, Eq, Clone)]
pub struct BlogParams {
    id: Option<String>,
}

#[component]
pub fn BlogPage() -> impl IntoView {
    let blog_params = use_params::<BlogParams>();
    let blog_id = Signal::derive(move || {
        blog_params
            .get()
            .map(|blog_params| blog_params.id)
            .ok()
            .flatten()
            .unwrap_or_default()
    });

    view! {
        <Await future=move || fetch_blog_markdown(blog_id.get()) let:markdown>
            <BlogContainer markdown=markdown.clone().unwrap_or_default()/>
        </Await>
    }
}

#[allow(clippy::future_not_send)]
async fn fetch_blog_markdown(id: String) -> Option<String> {
    let blog_url = format!("/blogs/{id}.md");
    let request = gloo::net::http::Request::get(&blog_url);
    let response = request.send().await.ok()?;
    let markdown = response.text().await.ok()?;
    Some(markdown)
}
