use leptonic::prelude::*;
use leptos::{component, view, IntoView};

use crate::post::{post::Post, post_card::PostCard};

#[component]
pub fn HomePage() -> impl IntoView {
    let post = Post {
        title: "Hello, World!".to_string(),
        content: "Welcome to Leptos!".to_string(),
        created_at: chrono::Local::now(),
        updated_at: chrono::Local::now(),
    };

    view! {
        <AppBar>
            <img src="/favicon.ico" alt="Teekey.dev Logo" style="height: 2em; width: 2em;"/>
            <H3>Teekey.dev</H3>
        </AppBar>
        <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%; height: 100%;">
            <PostCard post=post/>
        </Box>
    }
}
