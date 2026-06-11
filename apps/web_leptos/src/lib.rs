mod pages;
mod components;
mod api;

use leptos::*;
use leptos_router::*;
use leptos_meta::*;

use components::layout::Layout;
use pages::landing::LandingPage;
use pages::meeting::MeetingRoom;
use pages::voice::VoicePage;
use pages::messages::MessagesPage;
use pages::recordings::RecordingsPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <Routes>
                <Route path="" view=Layout>
                    <Route path="" view=LandingPage/>
                    <Route path="meeting/:room" view=MeetingRoom/>
                    <Route path="voice" view=VoicePage/>
                    <Route path="messages" view=MessagesPage/>
                    <Route path="recordings" view=RecordingsPage/>
                </Route>
            </Routes>
        </Router>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).ok();
    mount_to_body(|| view! { <App/> });
}
