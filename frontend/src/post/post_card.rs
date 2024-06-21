use chrono::TimeZone;
use leptonic::prelude::*;
use leptos::{component, view, IntoView};

use super::post::Post;

#[component]
pub fn PostCard(post: Post) -> impl IntoView {
    let mut preview = post.content.clone();
    preview.truncate(200);
    preview.push_str("...");

    view! {
        <Box style="display: flex; flex-direction: column; align-items: start; padding-top: 8px; padding-bottom: 0px; min-height: 100%; min-width: 100%;">
            <h2 style="color: var(--primary-text-color);">{post.title}</h2>
            <p style="color: var(--secondary-text-color); margin-top: 0px; margin-bottom: 0px;">
                {preview}
            </p>
            <p style="color: var(--secondary-text-color); margin-top: 8px; margin-bottom: 8px; font-size: 13px;">
                {format!("{}", post.created_at.format("%-d. %b. %Y"))}
            </p>
            <div style="min-width: 100%; height: 1px; background-color: #cccccc;"></div>
        </Box>
    }
}
