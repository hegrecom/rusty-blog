use chrono::TimeZone;
use leptonic::prelude::*;
use leptos::{component, view, IntoView};

use super::post::Post;

#[component]
pub fn PostCard<Tz>(post: Post<Tz>) -> impl IntoView
where
    Tz: TimeZone + 'static,
{
    view! {
        <Box style="display: flex; flex-direction: column; align-items: center; padding: 16px; min-height: 100%; min-width: 100%;">
            <h2>{post.title}</h2>
            <p>{post.content}</p>
            <p>Created at: {post.created_at.to_rfc2822()}</p>
        </Box>
    }
}
