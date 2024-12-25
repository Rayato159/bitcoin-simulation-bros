use leptos::prelude::*;

use crate::components::blockchain::Blockchain;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full items-center justify-center">
            <div class="max-w-4xl mx-auto">
                <Blockchain />
            </div>
        </div>
    }
}
