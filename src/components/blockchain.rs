use leptos::{logging::log, prelude::*};

use crate::{
    models::{block_model::BlockModel, coin_base::CoinBase, hashing_model::HashingModel},
    utils::generate_random_transactions,
};

const DIFFICULTY: usize = 4;
const INIT_MINING_REWARD: f64 = 50.0;
const DECRESE_RATE: f64 = 0.5;

#[component]
pub fn Blockchain() -> impl IntoView {
    let (blocks, set_blocks) = signal(vec![BlockModel::create_genesis_block()]);

    let handle_mining = move |_| {
        let target = "0".repeat(DIFFICULTY);

        let mut blocks_stack = blocks.get();

        if let Some(last_block) = blocks_stack.last_mut() {
            last_block.timestamp = chrono::Utc::now().timestamp() as u64;
            last_block.nonce = 0;
            last_block.hash = "0".repeat(64);

            loop {
                let hashing_model = HashingModel {
                    index: last_block.index,
                    nonce: last_block.nonce,
                    coin_base: last_block.coin_base.clone(),
                    transactions: last_block.transactions.clone(),
                    timestamp: last_block.timestamp,
                    previous_hash: last_block.previous_hash.clone(),
                };

                let hash = hashing_model.hash_calulation();

                log!("Mining... nonce: {:?}, {:?}", last_block.nonce, hash);

                if hash.starts_with(&target) {
                    last_block.hash = hash.clone();

                    if last_block.index % 4 == 0 {
                        let reward =
                            last_block.coin_base.reward - (INIT_MINING_REWARD * DECRESE_RATE);

                        let new_block = BlockModel {
                            index: last_block.index + 1,
                            nonce: 0,
                            coin_base: CoinBase {
                                miner: "Lookhin".to_string(),
                                reward,
                            },
                            transactions: generate_random_transactions(),
                            timestamp: chrono::Utc::now().timestamp() as u64,
                            previous_hash: hash,
                            hash: "0".repeat(64),
                        };

                        blocks_stack.push(new_block);
                        set_blocks(blocks_stack);

                        break;
                    } else {
                        let reward = last_block.coin_base.reward;

                        let new_block = BlockModel {
                            index: last_block.index + 1,
                            nonce: 0,
                            coin_base: CoinBase {
                                miner: "Lookhin".to_string(),
                                reward,
                            },
                            transactions: generate_random_transactions(),
                            timestamp: chrono::Utc::now().timestamp() as u64,
                            previous_hash: hash,
                            hash: "0".repeat(64),
                        };

                        blocks_stack.push(new_block);
                        set_blocks(blocks_stack);

                        break;
                    }
                }

                last_block.nonce += 1;
            }
        }
    };

    view! {
        <div class="max-w-full md:px-6 py-3 px-3">
            <For
                each=move || blocks.get()
                key=|block| block.index
                let:block
            >
                <div class="flex flex-col space-x-4 my-4 border-2 border-white p-4">
                    <div class="flex flex-col space-y-2">
                        <div class="flex flex-row items-center space-x-2">
                            <span class="font-bold p-1">Index:</span>
                            <span class="p-1">{block.index}</span>
                        </div>
                        <div class="flex flex-row items-center space-x-2">
                            <span class="font-bold p-1">Nonce:</span>
                            <span class="p-1">{block.nonce}</span>
                        </div>
                        <div class="flex flex-row items-center space-x-2">
                            <span class="font-bold p-1">Coin Base:</span>
                            <span class="p-1">{block.coin_base.miner.clone()} -> <span class="font-semibold">{format!("{:.2}", block.coin_base.reward)} BTC</span></span>
                        </div>
                        <div class="flex flex-col justify-center space-x-2">
                            <span class="font-bold p-1">Transactions:</span>
                            <For
                                each=move || block.transactions.clone()
                                key=|t| format!("{}-{}-{}", t.timestamp, t.sender, t.receiver)
                                let:t
                            >
                                <li class="md:text-base text-sm font-light">
                                    <span>{t.sender.clone()} -> {t.receiver.clone()}:</span>
                                    <span class="font-semibold ml-2">{format!("{:.2}", t.amount)} BTC</span>
                                </li>
                            </For>
                        </div>
                        <div class="flex flex-row items-center space-x-2">
                            <span class="font-bold p-1">Timestamp:</span>
                            <span class="p-1">{block.timestamp}</span>
                        </div>
                        <div class="flex flex-row items-center space-x-2">
                            <span class="font-bold p-1">Previous Hash:</span>
                            <span class="p-1">{block.previous_hash.clone()}</span>
                        </div>
                        <div class="flex flex-row items-center space-x-2">
                            <span class="font-bold p-1">Hash:</span>
                            <span class="p-1">{block.hash.clone()}</span>
                        </div>
                        <div class="flex flex-row items-center space-x-2">
                            <button on:click=handle_mining class="ml-1 bg-orange-400 border-none rounded-md p-2 font-semibold mt-2">Start Mining</button>
                        </div>
                    </div>
                </div>
            </For>
        </div>
    }
}
