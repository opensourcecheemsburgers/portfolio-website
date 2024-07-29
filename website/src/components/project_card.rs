use leptos::{
    component, expect_context, view, Children, IntoView, Signal, SignalGet,
};
use phosphor_leptos::{AppWindow, Binary, GithubLogo, IconWeight, Star};

use super::device_mockups::DeviceMockups;
use crate::{DarkThemeCtx, BODY_TEXT, HEADING_TEXT};

#[component]
fn ProjectCardContainer(children: Children) -> impl IntoView {
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
            class="bg-base-200 rounded-3xl lg:p-12 p-8 space-y-10 text-base-content data-[theme=winter]:shadow-2xl data-[theme=rustytube]:shadow-none w-full"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn RustyTubeCard() -> impl IntoView {
    view! {
        <ProjectCardContainer>
            <div class="flex flex-row gap-4 items-center">
                <h2 class=HEADING_TEXT>{"RustyTube"}</h2>
                <a
                    target="_blank"
                    href="https://github.com/opensourcecheemsburgers/RustyTube"
                    class="badge badge-lg border-[#FFCF40] gap-2"
                >
                    <Star weight=IconWeight::Duotone class="w-5 h-5"/>
                    {"500+"}
                </a>
            </div>
            <p class=BODY_TEXT>{"A rusty Youtube client for web, desktop and mobile."}</p>
            <div class="flex flex-col gap-2 sm:flex-row">
                <RustBadge/>
                <LeptosBadge/>
                <TailwindBadge/>
                <DaisyUiBadge/>
            </div>
            <DeviceMockups
                id_prefix="rt"
                desktop_img_src="img/rt_desktop.webp"
                mobile_img_src="img/rt_mobile.webp"
                tablet_img_src="img/rt_tablet.webp"
            />
            <div class="flex flex-col gap-4">
                <a href="https://rustytube.rs" class="btn btn-lg btn-block btn-primary">
                    <AppWindow weight=IconWeight::Duotone class="w-8 h-8 primary-content"/>
                    {"Web App"}
                </a>
                <a
                    target="_blank"
                    href="https://github.com/opensourcecheemsburgers/RustyTube/releases"
                    class="btn btn-lg btn-block btn-secondary"
                >
                    <Binary weight=IconWeight::Duotone class="w-8 h-8 primary-content"/>
                    {"Binaries"}
                </a>
                <a
                    target="_blank"
                    href="https://github.com/opensourcecheemsburgers/RustyTube"
                    class="btn btn-lg btn-block btn-neutral"
                >
                    <GithubLogo weight=IconWeight::Duotone class="w-8 h-8 primary-content"/>
                    {"Source"}
                </a>

            </div>

        </ProjectCardContainer>
    }
}

#[component]
pub fn UbiquityCard() -> impl IntoView {
    view! {
        <ProjectCardContainer>
            <div class="flex flex-row gap-4 items-center">
                <h1 class=HEADING_TEXT>{"Ubiquity"}</h1>
                <a
                    target="_blank"
                    href="https://github.com/opensourcecheemsburgers/ubiquity"
                    class="badge badge-lg border-[#FFCF40] gap-2"
                >
                    <Star weight=IconWeight::Duotone class="w-5 h-5"/>
                    {"150+"}
                </a>
            </div>
            <p class=BODY_TEXT>{"A rusty markdown editor for web, desktop and mobile."}</p>
            <div class="flex flex-col gap-2 sm:flex-row">
                <RustBadge/>
                <YewBadge/>
                <TailwindBadge/>
                <DaisyUiBadge/>
            </div>
            <DeviceMockups
                id_prefix="ubi"
                desktop_img_src="img/ubi_desktop.webp"
                mobile_img_src="img/ubi_mobile.webp"
                tablet_img_src="img/ubi_tablet.webp"
            />
            <div class="flex flex-col gap-4">
                <a
                    target="_blank"
                    href="https://ubiquity.rs"
                    class="btn btn-lg btn-block btn-primary"
                >
                    <AppWindow weight=IconWeight::Duotone class="w-8 h-8 primary-content"/>
                    {"Web App"}
                </a>
                <a
                    target="_blank"
                    href="https://github.com/opensourcecheemsburgers/ubiquity/releases"
                    class="btn btn-lg btn-block btn-secondary"
                >
                    <Binary weight=IconWeight::Duotone class="w-8 h-8 primary-content"/>
                    {"Binaries"}
                </a>
                <a
                    target="_blank"
                    href="https://github.com/opensourcecheemsburgers/ubiquity"
                    class="btn btn-lg btn-block btn-neutral"
                >
                    <GithubLogo weight=IconWeight::Duotone class="w-8 h-8 primary-content"/>
                    {"Source"}
                </a>

            </div>

        </ProjectCardContainer>
    }
}

#[component]
pub fn PersonalWebsiteCard() -> impl IntoView {
    view! {
        <ProjectCardContainer>
            <div class="flex flex-row gap-4 items-center">
                <h1 class=HEADING_TEXT>{"Personal Website"}</h1>
            </div>
            <p class=BODY_TEXT>{"This website. Responsive and Rusty."}</p>
            <div class="flex flex-col gap-2 sm:flex-row">
                <RustBadge/>
                <LeptosBadge/>
                <TailwindBadge/>
                <DaisyUiBadge/>
            </div>
            <DeviceMockups
                id_prefix="website"
                desktop_img_src="img/website_desktop.webp"
                mobile_img_src="img/website_mobile.webp"
                tablet_img_src="img/website_tablet.webp"
            />
            <div class="flex flex-col gap-4 lg:flex-row">
                <a
                    target="_blank"
                    href="https://github.com/opensourcecheemsburgers/portfolio-website"
                    class="btn btn-lg btn-block btn-neutral"
                >
                    <GithubLogo weight=IconWeight::Duotone class="w-8 h-8 primary-content"/>
                    {"Source"}
                </a>

            </div>

        </ProjectCardContainer>
    }
}

#[component]
fn RustBadge() -> impl IntoView {
    view! {
        <a href="https://rust-lang.org" class="badge badge-lg border-[#F74B00] gap-2">
            <svg
                class="w-5 h-5"
                viewBox="0 0 4417 3259"
                version="1.1"
                xmlns="http://www.w3.org/2000/svg"
                xmlns:xlink="http://www.w3.org/1999/xlink"
                xml:space="preserve"
                xmlns:serif="http://www.serif.com/"
                style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:1.41421;"
            >
                <g transform="matrix(4.16667,0,0,4.16667,0,0)">
                    <path
                        d="M525.403,293.05C393.77,293.05 274.175,308.875 185.633,334.665L185.633,554.963C274.175,580.753 393.77,596.577 525.403,596.577C676.06,596.577 810.938,575.848 901.537,543.175L901.537,346.457C810.938,313.781 676.06,293.05 525.403,293.05Z"
                        style="fill:rgb(143,30,28);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M907.423,492.442C903.566,481.779 902.794,468.288 906.062,455.28C911.912,431.991 928.483,419.082 943.075,426.447C946.693,428.274 949.849,431.178 952.462,434.865C952.701,434.864 952.94,434.865 953.177,434.881C953.177,434.881 997.729,487.987 956.49,550.884C955.595,554.453 879.956,642.602 862.447,645.408C850.987,647.244 877.338,555.41 907.423,492.442Z"
                        style="fill:rgb(143,30,28);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M176.479,482.021C181.779,472.391 183.637,459.233 180.696,445.596C175.43,421.18 156.786,404.486 139.054,408.311C134.656,409.259 130.729,411.383 127.388,414.409C127.106,414.351 126.824,414.296 126.543,414.256C126.543,414.256 70.251,456.208 114.486,528.18C115.291,531.921 198.337,637.018 218.797,643.943C232.188,648.475 207.55,551.418 176.479,482.021Z"
                        style="fill:rgb(143,30,28);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M97.467,488.066L97.474,488.081C97.659,488.226 97.831,488.357 97.467,488.066Z"
                        style="fill:rgb(227,58,37);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M993.119,412.903C992.239,409.839 991.363,406.777 990.457,403.741L1021.14,359.29C1024.27,354.768 1024.91,348.892 1022.87,343.735C1020.83,338.605 1016.38,334.925 1011.11,334.025L959.224,325.22C957.216,321.118 955.108,317.078 952.994,313.07L974.791,263.167C977.034,258.08 976.56,252.172 973.588,247.559C970.627,242.923 965.598,240.215 960.239,240.426L907.583,242.339C904.856,238.789 902.087,235.271 899.261,231.818L911.362,178.328C912.587,172.895 911.04,167.21 907.259,163.264C903.497,159.332 898.03,157.705 892.833,158.981L841.544,171.589C838.223,168.654 834.845,165.756 831.43,162.916L833.278,108.002C833.476,102.443 830.885,97.161 826.434,94.077C821.988,90.973 816.341,90.504 811.478,92.811L763.631,115.558C759.777,113.348 755.903,111.158 751.987,109.041L743.532,54.926C742.675,49.444 739.147,44.788 734.206,42.661C729.283,40.523 723.638,41.213 719.315,44.469L676.656,76.476C672.456,75.08 668.237,73.743 663.964,72.465L645.578,21.148C643.708,15.919 639.397,12.077 634.14,10.997C628.901,9.926 623.51,11.74 619.877,15.799L583.97,55.971C579.628,55.471 575.285,55.015 570.927,54.639L543.204,7.926C540.394,3.194 535.434,0.314 530.088,0.314C524.754,0.314 519.784,3.194 516.998,7.926L489.265,54.639C484.907,55.015 480.543,55.471 476.209,55.971L440.299,15.799C436.663,11.74 431.252,9.926 426.031,10.997C420.776,12.089 416.458,15.919 414.598,21.148L396.196,72.465C391.936,73.743 387.715,75.092 383.505,76.476L340.861,44.469C336.525,41.203 330.881,40.514 325.945,42.661C321.026,44.788 317.484,49.444 316.632,54.926L308.171,109.041C304.257,111.158 300.382,113.335 296.518,115.558L248.676,92.811C243.818,90.496 238.147,90.973 233.722,94.077C229.277,97.161 226.68,102.443 226.882,108.002L228.717,162.916C225.312,165.756 221.943,168.654 218.605,171.589L167.326,158.981C162.115,157.716 156.656,159.332 152.885,163.264C149.09,167.21 147.553,172.895 148.772,178.328L160.851,231.818C158.049,235.285 155.276,238.789 152.558,242.339L99.903,240.426C94.588,240.269 89.516,242.923 86.547,247.559C83.572,252.172 83.122,258.08 85.336,263.167L107.15,313.07C105.031,317.078 102.926,321.118 100.901,325.22L49.018,334.025C43.747,334.913 39.304,338.591 37.254,343.735C35.217,348.892 35.878,354.768 38.989,359.29L69.679,403.741C69.442,404.525 69.224,405.317 68.989,406.105L52.126,424.017L97.467,488.066C97.467,488.066 532.619,688.798 936.264,491.462C982.372,483.189 993.119,412.903 993.119,412.903Z"
                        style="fill:rgb(228,58,37);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M608.303,376.759C608.303,376.759 656.46,324.03 704.618,376.759C704.618,376.759 742.458,447.071 704.618,482.222C704.618,482.222 642.701,531.439 608.303,482.222C608.303,482.222 567.024,443.55 608.303,376.759Z"
                        style="fill:rgb(3,4,4);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M664.057,396.32C664.057,416.853 651.954,433.499 637.027,433.499C622.103,433.499 610,416.853 610,396.32C610,375.788 622.103,359.14 637.027,359.14C651.954,359.14 664.057,375.788 664.057,396.32Z"
                        style="fill:white;fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M393.365,362.361C393.365,362.361 475.973,325.785 498.519,407.423C498.519,407.423 522.137,502.577 430.682,507.948C430.682,507.948 314.06,485.486 393.365,362.361Z"
                        style="fill:rgb(3,4,4);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M434.855,397.668C434.855,418.841 422.375,436.014 406.978,436.014C391.587,436.014 379.104,418.841 379.104,397.668C379.104,376.49 391.587,359.322 406.978,359.322C422.375,359.322 434.855,376.49 434.855,397.668Z"
                        style="fill:white;fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M111.602,499.216C122.569,486.753 149.213,471.659 147.172,452.934C143.519,419.407 115.716,394.935 85.073,398.275C77.473,399.103 70.415,401.567 64.149,405.311C63.687,405.204 63.224,405.1 62.761,405.017C62.761,405.017 -40.87,455.89 18.197,557.674C18.754,562.811 136.045,713.342 168.985,724.805C190.544,732.307 149.074,596.165 111.602,499.216Z"
                        style="fill:rgb(228,58,37);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M953.549,494.673C940.856,483.973 907.387,474.255 906.629,455.435C905.273,421.737 929.141,393.414 959.941,392.175C967.579,391.867 974.925,393.258 981.676,396.032C982.118,395.858 982.56,395.686 983.005,395.535C983.005,395.535 1093.03,430.486 1049.7,539.901C1049.91,545.064 956.232,711.317 925.355,727.536C905.146,738.151 930.861,596.105 953.549,494.673Z"
                        style="fill:rgb(228,58,37);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M191.142,495.558C191.142,495.558 189.759,632.854 324.308,663.49L352.362,607.127C352.362,607.127 254.867,616.558 247.367,495.558L191.142,495.558Z"
                        style="fill:rgb(228,58,37);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M876.362,495.558C876.362,495.558 877.744,632.854 743.195,663.49L715.141,607.127C715.141,607.127 812.636,616.558 820.136,495.558L876.362,495.558Z"
                        style="fill:rgb(228,58,37);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M779.167,635.591C758.917,586.649 693.572,567.218 633.216,592.191C580.09,614.172 548.579,663.223 555.592,708.036C597.538,707.384 642.532,704.665 686.328,698.318C686.328,698.318 660.491,740.081 622.471,776.529C648.037,783.128 677.854,781.297 706.547,769.425C766.904,744.452 799.417,684.532 779.167,635.591Z"
                        style="fill:rgb(228,58,37);fill-rule:nonzero;"
                    ></path>
                    <path
                        d="M404.746,695.984C404.746,695.984 459.949,703.279 535.416,705.14C542.026,657.629 506.036,607.348 448.615,587.897C385.177,566.409 319.626,590.689 302.201,642.129C284.776,693.569 322.077,752.689 385.515,774.178C413.636,783.704 442.168,784.227 466.744,777.385C429.833,740.88 404.746,695.984 404.746,695.984Z"
                        style="fill:rgb(228,58,37);fill-rule:nonzero;"
                    ></path>
                </g>
            </svg>

            Rust
        </a>
    }
}

#[component]
fn YewBadge() -> impl IntoView {
    view! {
        <a href="https://yew.rs" class="badge badge-lg border-[#B3E1CE] gap-2">
            <svg class="w-5 h-5" viewBox="0 0 75 82" xmlns="http://www.w3.org/2000/svg">
                <circle cx="38" cy="40" r="25" fill="#B3E1CE"></circle>
                <path
                    d="M38.2373 41.0339L14 14"
                    stroke="#444444"
                    stroke-width="6"
                    stroke-linecap="round"
                ></path>
                <path
                    d="M38.2373 41.0339L62.4746 14"
                    stroke="#444444"
                    stroke-width="6"
                    stroke-linecap="round"
                ></path>
                <path
                    d="M38.2373 41.0339L38.2373 69"
                    stroke="#444444"
                    stroke-width="6"
                    stroke-linecap="round"
                ></path>
                <circle
                    cx="38"
                    cy="41"
                    r="7"
                    fill="#009A5B"
                    stroke="#444444"
                    stroke-width="4"
                ></circle>
            </svg>
            {"Yew"}
        </a>
    }
}

#[component]
fn LeptosBadge() -> impl IntoView {
    view! {
        <a href="https://leptos.dev" class="badge badge-lg border-[#4338ca] gap-2">
            <svg
                class="w-5 h-5"
                version="1.1"
                id="Layer_1"
                xmlns="http://www.w3.org/2000/svg"
                xmlns:xlink="http://www.w3.org/1999/xlink"
                viewBox="0 0 115.9988 115.9988"
                style="enable-background:new 0 0 115.9988 115.9988;"
                xml:space="preserve"
            >
                <g>
                    <g>
                        <g>
                            <path
                                style="fill:#180D38;"
                                d="M29.1281,108.2941c9.5736-4.5548,17.1531-12.6456,21.0335-22.5787
                                c-12.0865-3.4232-20.9707-14.548-20.9707-27.7159c0-15.8849,12.9236-28.8085,28.8085-28.8085
                                c1.4832,0,2.9404,0.113,4.3639,0.3303c1.9125-4.8287,4.5771-9.2786,7.8607-13.1979c-2.7243-2.8871-4.4077-6.7665-4.4077-11.0399
                                c0-1.6191,0.2457-3.1808,0.6921-4.6562C63.7305,0.2186,60.8908,0,57.9995,0C25.9672,0,0,25.9672,0,57.9994
                                C0,79.5165,11.7263,98.2828,29.1281,108.2941z"
                            ></path>
                            <path
                                style="fill:#EF3939;"
                                d="M81.9297,15.0082c3.6788,0,6.886-2.0536,8.5379-5.0742
                                c-5.3185-3.5997-11.2684-6.3339-17.646-8.0151c-0.3903,1.0504-0.6168,2.1798-0.6168,3.3644
                                C72.2049,10.6458,76.5673,15.0082,81.9297,15.0082z"
                            ></path>
                            <path
                                style="fill:#180D38;"
                                d="M95.5663,13.828c-2.8537,4.5375-7.8918,7.5688-13.6366,7.5688
                                c-1.2614,0-2.4835-0.1604-3.6622-0.4359c-0.9722-0.2272-1.9121-0.5362-2.8083-0.931c-2.8492,3.3173-5.1907,7.0806-6.8945,11.1766
                                c10.6715,4.2233,18.2432,14.6371,18.2432,26.7928c0,15.8849-12.9235,28.8085-28.8085,28.8085
                                c-0.4718,0-0.9406-0.0131-1.4069-0.0357c-3.7532,10.4704-11.0354,19.2744-20.406,24.9696
                                c6.7355,2.7367,14.0948,4.257,21.8129,4.257c32.0322,0,57.9994-25.9672,57.9994-57.9995
                                C115.9988,40.3018,108.0628,24.4664,95.5663,13.828z"
                            ></path>
                            <circle
                                style="fill:#EF3939;"
                                cx="57.9994"
                                cy="57.9994"
                                r="22.4198"
                            ></circle>
                        </g>
                        <path
                            style="fill:#FFFFFF;"
                            d="M78.2676,20.961c1.1786,0.2755,2.4008,0.4359,3.6622,0.4359
                            c5.7448,0,10.7829-3.0313,13.6366-7.5688c-1.6275-1.3855-3.3236-2.6925-5.0987-3.894c-1.6519,3.0206-4.8591,5.0742-8.5379,5.0742
                            c-5.3624,0-9.7249-4.3624-9.7249-9.7249c0-1.1846,0.2264-2.3141,0.6168-3.3644c-2.062-0.5436-4.1682-0.9763-6.3133-1.2917
                            c-0.4464,1.4753-0.6921,3.0371-0.6921,4.6562c0,4.2734,1.6834,8.1528,4.4077,11.0399c-3.2836,3.9193-5.9482,8.3692-7.8607,13.1979
                            c-1.4235-0.2172-2.8807-0.3303-4.3639-0.3303c-15.8849,0-28.8085,12.9235-28.8085,28.8085
                            c0,13.168,8.8842,24.2928,20.9707,27.7159c-3.8804,9.9332-11.4599,18.0239-21.0335,22.5787
                            c2.2621,1.3013,4.6175,2.456,7.0584,3.4478c9.3706-5.6952,16.6528-14.4992,20.406-24.9696
                            c0.4663,0.0226,0.9351,0.0357,1.4069,0.0357c15.8849,0,28.8085-12.9236,28.8085-28.8085c0-12.1557-7.5717-22.5695-18.2432-26.7928
                            c1.7038-4.0959,4.0453-7.8593,6.8945-11.1766C76.3555,20.4248,77.2953,20.7338,78.2676,20.961z M80.4193,57.9994
                            c0,12.3623-10.0576,22.4199-22.4198,22.4199c-12.3623,0-22.4199-10.0576-22.4199-22.4199
                            c0-12.3622,10.0576-22.4199,22.4199-22.4199C70.3617,35.5795,80.4193,45.6371,80.4193,57.9994z"
                        ></path>
                    </g>
                </g>
            </svg>
            {"Leptos"}
        </a>
    }
}

#[component]
fn TailwindBadge() -> impl IntoView {
    view! {
        <a href="https://tailwindcss.com" class="badge badge-lg border-[#38bdf8] gap-2">
            <svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 54 33">
                <g clip-path="url(#prefix__clip0)">
                    <path
                        fill="#38bdf8"
                        fill-rule="evenodd"
                        d="M27 0c-7.2 0-11.7 3.6-13.5 10.8 2.7-3.6 5.85-4.95 9.45-4.05 2.054.513 3.522 2.004 5.147 3.653C30.744 13.09 33.808 16.2 40.5 16.2c7.2 0 11.7-3.6 13.5-10.8-2.7 3.6-5.85 4.95-9.45 4.05-2.054-.513-3.522-2.004-5.147-3.653C36.756 3.11 33.692 0 27 0zM13.5 16.2C6.3 16.2 1.8 19.8 0 27c2.7-3.6 5.85-4.95 9.45-4.05 2.054.514 3.522 2.004 5.147 3.653C17.244 29.29 20.308 32.4 27 32.4c7.2 0 11.7-3.6 13.5-10.8-2.7 3.6-5.85 4.95-9.45 4.05-2.054-.513-3.522-2.004-5.147-3.653C23.256 19.31 20.192 16.2 13.5 16.2z"
                        clip-rule="evenodd"
                    ></path>
                </g>
                <defs>
                    <clipPath id="prefix__clip0">
                        <path fill="#fff" d="M0 0h54v32.4H0z"></path>
                    </clipPath>
                </defs>
            </svg>
            {"Tailwind"}
        </a>
    }
}

#[component]
fn DaisyUiBadge() -> impl IntoView {
    view! {
        <a href="https://daisyui.com" class="badge badge-lg border-[#1AD1A5] gap-2">
            <svg
                class="w-5 h-5"
                viewBox="0 0 1024 1024"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <rect x="256" y="670.72" width="512" height="256" rx="128" fill="#1AD1A5"></rect>
                <circle cx="512" cy="353.28" r="256" fill="white"></circle>
                <circle
                    cx="512"
                    cy="353.28"
                    r="261"
                    stroke="black"
                    stroke-opacity="0.2"
                    stroke-width="10"
                ></circle>
                <circle cx="512" cy="353.28" r="114.688" fill="#FF9903"></circle>
            </svg>
            {"DaisyUI"}
        </a>
    }
}
