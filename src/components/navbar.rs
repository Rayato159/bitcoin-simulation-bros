use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="block sticky top-0 max-w-full md:px-6 py-3 px-3 bg-mygrey-1 z-30">
            <div class="flex flex-row justify-between">
                <a href="/">
                    <div class="flex flex-row items-center justify-items-center hover: cursor-pointer font-semibold space-x-2">
                        <div>
                            <img class="max-h-8" src="./assets/Bitcoin.png" alt="Bitcoin" />
                        </div>
                        <div class="md:text-xl text-xs">
                            "Bitcoin Simulation Bros"
                        </div>
                    </div>
                </a>
                <div class="flex flex-row items-center">
                    <a href="/block">
                        <div class="md:text-xl text-sm font-light md:mr-4 mr-2 hover:cursor-pointer hover:text-gray-500">"Block"</div>
                    </a>
                    <a href="/blockchain">
                        <div class="md:text-xl text-sm font-light md:mr-4 mr-2 hover:cursor-pointer hover:text-gray-500">"Blockchain"</div>
                    </a>
                </div>
            </div>
        </nav>
    }
}
