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
        <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%; height: 100%;">
            <PostCard post=post/>
        </Box>
    }
}
