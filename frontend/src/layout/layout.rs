use leptonic::prelude::*;
use leptos::{component, view, Children, IntoView};

use crate::gnb::gnb::Gnb;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div>
            <Gnb/>
            <main>{children()}</main>
        </div>
    }
}
