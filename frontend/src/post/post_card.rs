use chrono::TimeZone;
use leptos::{component, view, IntoView};

use super::post::Post;

#[component]
pub fn PostCard<Tz>(post: Post<Tz>) -> impl IntoView
where
    Tz: TimeZone,
{
    view! {
        <h2>{post.title}</h2>
        <p>{post.content}</p>
        <p>Created at: {post.created_at.to_rfc2822()}</p>
    }
}
