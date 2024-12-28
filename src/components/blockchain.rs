use leptos::prelude::*;

use crate::{components::block::Block, models::block_model::BlockModel, utils::mine_block};

#[component]
pub fn Blockchain() -> impl IntoView {
    let (blocks, set_blocks) = signal(vec![BlockModel::create_genesis_block()]);

    let handle_mining = move |_| {
        let mut blocks_stack = blocks.get().clone();

        if let Some(last_block) = blocks_stack.last_mut() {
            let new_block = mine_block(last_block);
            blocks_stack.push(new_block);
            set_blocks(blocks_stack.clone());
        }
    };

    view! {
        <div class="max-w-full md:px-6 py-3 px-3">
            <For
                each=move || blocks.get()
                key=|block| block.hash.clone()
                let:block
            >
                <Block block_model=block />
            </For>
            <div class="flex flex-row justify-center items-center w-full bg-orange-400 hover:bg-orange-300 border-none rounded-md p-2">
                <button on:click=handle_mining>
                    <div class="flex flex-row justify-center items-center space-x-2">
                        <div>
                            <img src="./assets/Mining.png" alt="Mining" class="max-w-[32px] max-h-[32px]" />
                        </div>
                        <div class="font-semibold">
                            Start Mining
                        </div>
                    </div>
                </button>
            </div>
        </div>
    }
}
