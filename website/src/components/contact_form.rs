use leptos::{html::Textarea, IntoView, NodeRef, RwSignal, Signal, SignalGet, SignalSet, component, expect_context, view};
use phosphor_leptos::{EnvelopeSimple, IconWeight, Lock, MatrixLogo};

use crate::{DarkThemeCtx, BODY_TEXT};

#[component]
pub fn ContactForm() -> impl IntoView {
    let dark_theme_ctx = expect_context::<DarkThemeCtx>().0;
    let theme = Signal::derive(move || {
        if dark_theme_ctx.get() {
            "rustytube"
        } else {
            "winter"
        }
    });

    let subject = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let mailto = Signal::derive(move || {
        format!(
            "mailto:simpilldev@gmail.com?subject={}&body={}",
            subject.get(),
            email.get()
        )
    });

    view! {
        <div
            data-theme=theme
            class="flex flex-col w-full max-w-2xl bg-base-200 rounded-3xl lg:p-12 p-8 gap-8 text-base-content data-[theme=winter]:shadow-2xl data-[theme=rustytube]:shadow-none"
        >
            // <h1 class=HEADING_TEXT>{"Email Form"}</h1>
            <form class="flex flex-col gap-6">
                <ContactSubject subject=subject/>
                <ContactEmail email=email/>
            </form>
            <a target="_blank" href=mailto class="btn btn-primary btn-lg btn-block">
                <EnvelopeSimple weight=IconWeight::Duotone class="w-8 h-8"/>
                {"Create Email"}
            </a>
        </div>
    }
}

#[component]
fn ContactSubject(subject: RwSignal<String>) -> impl IntoView {
    let subject_ref = NodeRef::new();

    view! {
        <label class="w-full form-control">
            <div class="label">
                <span class=BODY_TEXT>{"Subject"}</span>
            // <span class="label-text-alt">Top Right label</span>
            </div>
            <input
                _ref=subject_ref
                on:change=move |_| {
                    if let Some(input) = subject_ref.get() {
                        subject.set(input.value());
                    }
                }

                type="text"
                placeholder=""
                class="w-full input input-bordered"
            />
        // <div class="label">
        // <span class="label-text-alt">Bottom Left label</span>
        // <span class="label-text-alt">Bottom Right label</span>
        // </div>
        </label>
    }
}

#[component]
fn ContactEmail(email: RwSignal<String>) -> impl IntoView {
    let email_ref = NodeRef::<Textarea>::new();

    view! {
        <label class="w-full form-control">
            <div class="label">
                <span class=BODY_TEXT>{"Email"}</span>
            // <span class="label-text-alt">Top Right label</span>
            </div>

            <textarea
                _ref=email_ref
                on:input=move |_| {
                    if let Some(input) = email_ref.get() {
                        email.set(input.value());
                    }
                }

                class="textarea textarea-bordered min-h-48"
                placeholder="Hi Stephen,\n\nI am reaching out to discuss...\n\nKind Regards,\n{Your Name}"
            ></textarea>
        // <div class="label">
        // <span class="label-text-alt">Bottom Left label</span>
        // <span class="label-text-alt">Bottom Right label</span>
        // </div>
        </label>
    }
}

#[component]
pub fn SocialsBlock() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 w-full max-w-2xl">
            <a
                target="_blank"
                href="https://matrix.to/#/@stephenpower:mozilla.org"
                class="btn btn-lg btn-block btn-success"
            >
                <MatrixLogo weight=IconWeight::Duotone class="w-8 h-8"/>
                {"Matrix"}
            </a>
            <div class="divider divider-horizontal"></div>
            <a
                target="_blank"
                href="gpg/stephen_power.gpg"
                download="stephen_power.gpg"
                class="btn btn-lg btn-block btn-neutral"
            >
                <Lock weight=IconWeight::Duotone class="w-8 h-8"/>
                {"Email with PGP Encryption"}
            </a>
        </div>
    }
}
