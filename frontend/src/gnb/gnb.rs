use leptonic::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn Gnb() -> impl IntoView {
    view! {
        <AppBar>
            <Stack
                orientation=StackOrientation::Horizontal
                spacing=Size::Px(8)
                style="margin-left: 24px;"
            >
                <img src="/favicon.ico" alt="Teekey.dev Logo" style="height: 36px; width: 36px;"/>
                <H3>Teekey.dev</H3>
            </Stack>
        </AppBar>
    }
}
