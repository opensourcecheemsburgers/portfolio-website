use leptos::{component, view, IntoView};

use crate::components::{ContactForm, SocialsBlock};

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <ContactForm/>
        <div class="divider divider-primary">OR</div>
        <SocialsBlock/>
    }
}
