use dioxus::prelude::*;
use google_dioxus_ads::prelude::*;

fn main() {
    dioxus::desktop::launch_cfg(app, |c| {
        c.with_custom_head(GoogleAds::headers())
    });
}

fn app(cx: Scope) -> Element {

    let slot = AdSlot::new(
        "5302",
        "Desktop/tests",
        "mobile_leaderboard_fyt",
        vec![
            AdSize::fluid(),
            AdSize::sized(320, 50),
        ],
    );

    #[cfg(debug_assertions)]{
        let context = dioxus::desktop::use_window(&cx);
        context.devtool();
    }

    cx.render(rsx! {
        GoogleAd {
            id: "div-gpt-ad-1655721712387-0",
            style: "min-width: 320px; min-height: 50px;",
            slot: slot.clone(),
        }

        GoogleAd {
            id: "div-gpt-ad-1655721712387-1",
            style: "min-width: 320px; min-height: 50px;",
            slot: slot,
        }
    })
}
