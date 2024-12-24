use crate::components::navbar::Navbar;
use crate::pages::block_chain::BlockChain;
use crate::pages::block_page::BlockPage;
use crate::pages::home::Home;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Navbar/>
            <main>
            <Routes fallback=|| "Not found.">
                <Route
                    path=path!("/")
                    view=Home
                />
                <Route
                    path=path!("/block")
                    view=BlockPage
                />
                <Route
                    path=path!("/blockchain")
                    view=BlockChain
                />
            </Routes>
            </main>
        </Router>
    }
}
