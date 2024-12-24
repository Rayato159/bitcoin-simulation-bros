use leptos::prelude::*;

use crate::components::block::Block;

#[component]
pub fn BlockPage() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full items-center justify-center">
            <div class="max-w-4xl mx-auto">
                <Block />
            </div>
        </div>
    }
}
