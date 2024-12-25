use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="block sticky top-0 max-w-full md:px-6 py-3 px-3 bg-mygrey-1 z-30">
            <div class="flex flex-row justify-center items-center ">
                <a href="/">
                    <div class="flex flex-row items-center hover: cursor-pointer font-semibold space-x-2">
                        <div>
                            <img class="max-h-8" src="./assets/Bitcoin.png" alt="Bitcoin" />
                        </div>
                        <div class="md:text-xl text-xs">
                            "Bitcoin Simulation Bros"
                        </div>
                    </div>
                </a>
            </div>
        </nav>
    }
}
