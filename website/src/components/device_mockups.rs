use leptos::{IntoView, component, view};

#[component]
pub fn DeviceMockups(
    id_prefix: &'static str,
    desktop_img_src: &'static str,
    mobile_img_src: &'static str,
    tablet_img_src: &'static str,
) -> impl IntoView {
    view! {
        <div class="w-full carousel">
            <div
                id=format!("{id_prefix}_desktop")
                class="relative justify-center items-center carousel-item container-inline-size"
            >
                <div class="w-[96rem] container-inline-size lg:w-[96rem]">
                    <div class="device device-spacegray device-macbook-pro">
                        <div class="device-frame">
                            <img
                                loading="lazy"
                                class="device-screen bg-base-100 crisp-edges"
                                src=desktop_img_src
                            />
                        </div>
                        <div class="device-stripe"></div>
                        // <div class="device-header"></div>
                        <div class="device-sensors"></div>
                        <div class="device-btns"></div>
                        <div class="device-power"></div>
                    </div>
                </div>
            </div>
            <div
                id=format!("{id_prefix}_mobile")
                class="relative justify-center w-full items-center carousel-item container-inline-size"
            >
                <div class="w-[9.75rem] container-inline-size md:w-[13rem] lg:w-[19.5rem] 2xl:w-[26rem]">
                    <div class="device device-black device-google-pixel-6-pro">
                        <div class="device-frame">
                            <img
                                loading="lazy"
                                class="device-screen bg-base-100 crisp-edges"
                                src=mobile_img_src
                            />
                        </div>
                        <div class="device-stripe"></div>
                        <div class="device-header"></div>
                        // <div class="device-sensors"></div>
                        <div class="device-btns"></div>
                        <div class="device-power"></div>
                    </div>
                </div>
            </div>
            <div
                id=format!("{id_prefix}_tablet")
                class="relative justify-center items-center carousel-item container-inline-size w-full"
            >
                <div class="w-[15rem] container-inline-size md:w-[20rem] lg:w-[30rem] 2xl:w-[40rem]">
                    <div class="device device-spacegray device-ipad-pro-2017">
                        <div class="device-frame">
                            <img
                                loading="lazy"
                                class="device-screen bg-base-100 crisp-edges"
                                src=tablet_img_src
                            />
                        </div>
                        <div class="device-stripe"></div>
                        <div class="device-header"></div>
                        <div class="device-sensors"></div>
                        <div class="device-btns"></div>
                        <div class="device-power"></div>
                        <div class="device-home"></div>
                    </div>
                </div>
            </div>
        </div>
        <div class="flex gap-2 justify-center py-2 w-full">
            <a
                href=format!("#{id_prefix}_desktop")
                class="rounded-xl btn btn-sm btn-info scroll-smooth lg:btn-md"
            >
                {"PC"}
            </a>
            <a
                href=format!("#{id_prefix}_mobile")
                class="rounded-xl btn btn-sm btn-info scroll-smooth lg:btn-md"
            >
                {"Mobile"}
            </a>
            <a
                href=format!("#{id_prefix}_tablet")
                class="rounded-xl btn btn-sm btn-info scroll-smooth lg:btn-md"
            >
                {"Tablet"}
            </a>
        </div>
    }
}
