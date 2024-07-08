use leptonic::prelude::*;
use leptos::*;

use crate::{
    backend_client::client::BackendClient, post::{
        post::Post,
        post_card::PostCard,
    }
};

#[component]
pub fn HomePage() -> impl IntoView {
    let (page, set_page) = create_signal(1);
    let (max_page, set_max_page) = create_signal(i32::MAX);
    let (posts, set_posts): (ReadSignal<Vec<Post>>, WriteSignal<Vec<Post>>) = create_signal(vec![]);
    let posts_resource = create_resource(
        move || page.get(),
        move |page| async move {
            let per = 10;
            match BackendClient::default().fetch_posts(page, per).await {
                Ok(post_response) => {
                    set_max_page.set(post_response.meta().total_pages());
                    post_response.data()
                }
                Err(_e) => vec![],
            }
        },
    );
    let posts_loading = posts_resource.loading();

    window_event_listener(ev::scroll, move |_| {
        let inner_height_value = window().inner_height().ok();
        let body = document().body();
        let scroll_y = window().scroll_y().ok();

        if let(Some(inner_height_value), Some(body), Some(scroll_y)) = (inner_height_value, body, scroll_y) {
            if let Some(inner_height) = inner_height_value.as_f64() {
                if (body.scroll_height() as f64) - (inner_height + scroll_y) < 10.0 {
                    if max_page.get() > page.get() {
                        set_page.update(|page| *page += 1);
                    }
                }
            }
        }
    });

    view! {
        <Grid spacing=Size::Px(0) style="margin-bottom: 60px;">
            <Transition fallback=move || {}>
                {move || {
                    posts_resource
                        .get()
                        .map(|fetched_posts| {
                            set_posts.update(|posts| posts.extend(fetched_posts));
                            view! {
                                <For each=move || posts.get() key=|post| post.id let:post>
                                    <Row>
                                        <Col md=2 sm=2 xs=0>
                                            <div></div>
                                        </Col>
                                        <Col md=8 sm=8 xs=12 style="justify-content: center;">
                                            <PostCard post=post/>
                                        </Col>
                                        <Col md=2 sm=2 xs=0>
                                            <div></div>
                                        </Col>
                                    </Row>
                                </For>
                            }
                        })
                }}

            </Transition>
        </Grid>
        {move || {
            if posts_loading.get() {
                view! {
                    <Row>
                        <Col md=12>
                            <div>Loading...</div>
                        </Col>
                    </Row>
                }
            } else {
                view! {
                    <Row>
                        <Col md=12>
                            <div></div>
                        </Col>
                    </Row>
                }
            }
        }}
    }
}
