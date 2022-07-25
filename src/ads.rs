use crate::adslot::*;
use dioxus::prelude::*;

const HEADERS_HTML: &str = include_str!("../build/headers.html");

#[derive(PartialEq, Props)]
pub struct GoogleAds<'a> {
    pub id: &'a str,
    pub style: &'a str,
    pub slot: AdSlot,
}

impl GoogleAds<'_> {

    pub fn headers() -> String {
        #[cfg(debug_assertions)]
        return HEADERS_HTML.replace("let debug = false;", "let debug = true;");

        #[cfg(not(debug_assertions))]
        HEADERS_HTML.to_owned()
    }
}

#[allow(non_snake_case)]
pub fn GoogleAd<'a>(cx: Scope<'a, GoogleAds<'a>>) -> Element {

    let ad_script = cx.props.slot.push_script(cx.props.id);

    cx.render(rsx! {
        div {
            id: "dioxus-ads-parent-{cx.props.id}",
            div {
                id: "{cx.props.id}",
                style: "{cx.props.style}",
            }
            input {
                id: "ads_link_target-{cx.props.id}",
                style: "display: none;",
                oninput: move |data|
                {
                    open::that(&data.value).unwrap_or(());
                }
            }
            script {
                "{ad_script}"
            }
        }
    })
}

