use leptos::{logging::log, prelude::*};

use crate::{
    components::block::Block,
    models::{block_model::BlockModel, coin_base::CoinBase, hashing_model::HashingModel},
    utils::generate_random_transactions,
};

const DIFFICULTY: usize = 4;
const INIT_MINING_REWARD: f64 = 50.0;
const DECRESE_RATE: f64 = 0.5;

#[derive(Debug, Clone)]
pub struct MiningArgs {
    pub blocks: Vec<BlockModel>,
}

#[component]
pub fn Blockchain() -> impl IntoView {
    let (blocks, set_blocks) = signal(vec![BlockModel::create_genesis_block()]);

    let handle_mining = move |_| {
        let target = "0".repeat(DIFFICULTY);
        let mut blocks_stack = blocks.get().clone();

        if let Some(last_block) = blocks_stack.last_mut() {
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
                last_block.hash = hash.clone();

                log!("Mining... nonce: {:?}, hash: {:?}", last_block.nonce, hash);

                if hash.starts_with(&target) {
                    let reward = if last_block.index % 4 == 0 {
                        last_block.coin_base.reward - (INIT_MINING_REWARD * DECRESE_RATE)
                    } else {
                        last_block.coin_base.reward
                    };

                    let new_block = BlockModel {
                        index: last_block.index + 1,
                        nonce: 0,
                        coin_base: CoinBase {
                            miner: "Lookhin".to_string(),
                            reward,
                        },
                        transactions: generate_random_transactions(),
                        timestamp: chrono::Utc::now().timestamp() as u64,
                        previous_hash: hash.clone(),
                        hash: "0".repeat(64),
                    };

                    blocks_stack.push(new_block);
                    set_blocks(blocks_stack.clone());
                    break;
                }

                last_block.nonce += 1;
            }

            log!("Blocks State {:#?}", blocks.get());
        }
    };

    view! {
        <div class="max-w-full md:px-6 py-3 px-3">
            <For
                each=move || blocks.get()
                key=|block| block.index
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
