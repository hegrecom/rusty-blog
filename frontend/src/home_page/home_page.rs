use leptonic::prelude::*;
use leptos::*;

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
        <Grid spacing=Size::Px(
            0,
        )>

            {(0..10)
                .map(|_| {
                    let cloned_post = post.clone();
                    view! {
                        <Row>
                            <Col md=3 sm=3 xs=3>
                                <div></div>
                            </Col>
                            <Col md=6 sm=6 xs=6>
                                <PostCard post=cloned_post/>
                            </Col>
                            <Col md=3 sm=3 xs=3>
                                <div></div>
                            </Col>
                        </Row>
                    }
                })
                .collect_view()}
        </Grid>
    }
}
