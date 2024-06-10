use leptos::{IntoView, component, view};

use crate::components::{
    Hero, PersonalWebsiteCard, RustyTubeCard, UbiquityCard,
};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Hero/>
        <RustyTubeCard/>
        <UbiquityCard/>
        <PersonalWebsiteCard/>
    }
}
