use leptos::prelude::*;

use crate::models::block_model::BlockModel;

#[component]
pub fn Block(block_model: BlockModel) -> impl IntoView {
    view! {
        <div class="flex flex-col space-x-4 my-4 border-2 border-white rounded-md p-4">
            <div class="flex flex-col space-y-2">
                <div class="flex flex-row items-center space-x-2">
                    <span class="font-bold p-1">Index:</span>
                    <span class="p-1">{block_model.index}</span>
                </div>
                <div class="flex flex-row items-center space-x-2">
                    <span class="font-bold p-1">Nonce:</span>
                    <span class="p-1">{block_model.nonce}</span>
                </div>
                <div class="flex flex-row items-center space-x-2">
                    <span class="font-bold p-1">Coin Base:</span>
                    <span class="p-1">{block_model.coin_base.miner.clone()} -> <span class="font-semibold">{format!("{:.8}", block_model.coin_base.reward)} BTC</span></span>
                </div>
                <div class="flex flex-col justify-center space-x-2">
                    <span class="font-bold p-1">Transactions:</span>
                    <For
                        each=move || block_model.transactions.clone()
                        key=|t| format!("{}-{}-{}", t.timestamp, t.sender, t.receiver)
                        let:t
                    >
                        <li class="md:text-base text-sm font-light">
                            <span>{t.sender.clone()} -> {t.receiver.clone()}:</span>
                            <span class="font-semibold ml-2">{format!("{:.8}", t.amount)} BTC</span>
                        </li>
                    </For>
                </div>
                <div class="flex flex-row items-center space-x-2">
                    <span class="font-bold p-1">Timestamp:</span>
                    <span class="p-1">{block_model.timestamp}</span>
                </div>
                <div class="flex flex-row items-center space-x-2">
                    <span class="font-bold p-1">Previous Hash:</span>
                    <span class="p-1">{block_model.previous_hash.clone()}</span>
                </div>
                <div class="flex flex-row items-center space-x-2">
                    <span class="font-bold p-1">Hash:</span>
                    <span class="p-1">{block_model.hash.clone()}</span>
                </div>
            </div>
        </div>
    }
}
