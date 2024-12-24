use core::time;

use leptos::{html, logging::log, prelude::*};

use crate::models::{block_model::BlockModel, hashing_model::HashingModel};

const DIFFICULTY: usize = 4;

#[component]
pub fn Block() -> impl IntoView {
    let (block, set_block) = signal(BlockModel::create_genesis_block());
    let (timestamp, set_timestamp) = signal(chrono::Utc::now().timestamp() as u64);
    let (nonce, set_nonce) = signal(0);
    let (hash, set_hash) = signal("0".repeat(64));

    let handle_mining = move |_| {
        set_timestamp(chrono::Utc::now().timestamp() as u64);
        if nonce.get() != 0 {
            set_nonce(0);
        }
        if hash.get() != "0".repeat(64) {
            set_hash("0".repeat(64));
        }
        let target = "0".repeat(DIFFICULTY);

        let mut block = block.read().clone();
        block.timestamp = timestamp.get();

        loop {
            block.nonce = nonce.get();
            set_nonce(block.nonce + 1);

            let hashing_model = HashingModel {
                index: block.index,
                nonce: block.nonce,
                coin_base: block.coin_base.clone(),
                transactions: block.transactions.clone(),
                timestamp: block.timestamp,
                previous_hash: block.previous_hash.clone(),
            };

            let hash = hashing_model.hash_calulation();

            log!("Mining... nonce: {:?}, {:?}", nonce.get(), hash);

            if hash.starts_with(&target) {
                block.hash = hash.clone();
                set_hash(hash);
                set_block(block);
                break;
            }
        }
    };

    view! {
        <div class="max-w-full md:px-6 py-3 px-3">
            <div class="flex flex-col space-x-4">
                <div class="flex flex-col space-y-2">
                    <div class="flex flex-row space-x-2">
                        <span class="font-bold">Index:</span>
                        <span>{block.read().index}</span>
                    </div>
                    <div class="flex flex-row items-center space-x-2">
                        <span class="font-bold">Nonce:</span>
                        <input class="text-black w-full p-1" type="text"
                            value=nonce
                        />
                    </div>
                    <div class="flex flex-row space-x-2">
                        <span class="font-bold">Timestamp:</span>
                        <span>{timestamp}</span>
                    </div>
                    <div class="flex flex-row space-x-2">
                        <span class="font-bold">Previous Hash:</span>
                        <span>{block.read().previous_hash.clone()}</span>
                    </div>
                    <div class="flex flex-row space-x-2">
                        <span class="font-bold">Hash:</span>
                        <input class="text-black w-full p-1" type="text"
                            value=hash
                        />
                    </div>
                    <div>
                        <button on:click=handle_mining class="bg-orange-400 border-none rounded-md p-2 font-semibold">"Let's Mining"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
