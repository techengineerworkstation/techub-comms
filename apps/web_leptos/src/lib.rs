mod pages;
mod components;
mod api;

use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use wasm_bindgen::JsCast;

use components::layout::Layout;
use pages::login::LoginPage;
use pages::landing::LandingPage;
use pages::meeting::MeetingRoom;
use pages::voice::VoicePage;
use pages::messages::MessagesPage;
use pages::recordings::RecordingsPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (is_authenticated, set_authenticated) = create_signal(false);
    let (token, set_token) = create_signal(None::<String>);

    // Check for existing token on load
    create_effect(move |_| {
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Ok(doc) = doc.dyn_into::<web_sys::HtmlDocument>() {
                    if let Ok(cookies) = doc.cookie() {
                        for cookie in cookies.split(';') {
                            let cookie = cookie.trim();
                            if cookie.starts_with("techub_token=") {
                                let t = cookie[13..].to_string();
                                if !t.is_empty() {
                                    set_token.set(Some(t));
                                    set_authenticated.set(true);
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    view! {
        <Router>
            {move || if is_authenticated.get() {
                view! {
                    <Routes>
                        <Route path="" view=Layout>
                            <Route path="" view=LandingPage/>
                            <Route path="meeting/:room" view=MeetingRoom/>
                            <Route path="voice" view=VoicePage/>
                            <Route path="messages" view=MessagesPage/>
                            <Route path="recordings" view=RecordingsPage/>
                        </Route>
                    </Routes>
                }.into_view()
            } else {
                view! {
                    <LoginPage set_authenticated=set_authenticated set_token=set_token/>
                }.into_view()
            }}
        </Router>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).ok();
    mount_to_body(|| view! { <App/> });
}
