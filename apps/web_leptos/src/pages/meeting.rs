use leptos::*;
use leptos_router::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::components::video_room::VideoRoom;

#[component]
pub fn MeetingRoom() -> impl IntoView {
    let params = use_params_map();
    let room = move || params.with(|p| p.get("room").cloned().unwrap_or_default());
    let (token_data, set_token_data) = create_signal(None::<api::VideoTokenResp>);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    let room_val = room();
    spawn_local({
        let r = room_val.clone();
        async move {
            match api::get_video_token(&r, 0).await {
                Ok(d) => set_token_data.set(Some(d)),
                Err(e) => set_error.set(Some(e)),
            }
            set_loading.set(false);
        }
    });

    view! {
        <div class="h-full">
            <div class="mb-4 flex items-center justify-between animate-fade-in-down">
                <div class="flex items-center gap-3">
                    <A href="/" class="btn-icon text-gray-500 hover:text-teal-600">
                        <span class="text-lg">"\u{2190}"</span>
                    </A>
                    <div>
                        <h2 class="text-lg font-semibold text-gray-900 flex items-center gap-2">
                            <span>"\u{1F3A5}"</span>
                            {room()}
                        </h2>
                        <p class="text-xs text-gray-500">"Agora Video Channel"</p>
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <span class="turquoise-badge">"Live"</span>
                </div>
            </div>

            <div class="h-[calc(100%-4rem)]">
                {move || -> View {
                    if loading.get() {
                        view! {
                            <div class="flex items-center justify-center h-full">
                                <div class="text-center animate-pulse">
                                    <div class="w-16 h-16 mx-auto mb-4 rounded-full flex items-center justify-center"
                                        style="background: linear-gradient(135deg, #009999, #005c5c)">
                                        <span class="text-white text-2xl animate-spin-slow">"\u{1F3A5}"</span>
                                    </div>
                                    <p class="text-gray-600 font-medium">"Joining channel..."</p>
                                    <p class="text-sm text-gray-400 mt-1">"Connecting to Agora"</p>
                                </div>
                            </div>
                        }.into_view()
                    } else if error.get().is_some() {
                        let err_msg = error.get().unwrap_or_else(|| "Failed to join channel".into());
                        view! {
                            <div class="flex items-center justify-center h-full">
                                <div class="glow-card p-8 text-center max-w-md animate-scale-in">
                                    <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-red-100 flex items-center justify-center">
                                        <span class="text-red-500 text-2xl">"\u{26A0}"</span>
                                    </div>
                                    <h3 class="text-lg font-semibold text-gray-900 mb-2">"Connection Error"</h3>
                                    <p class="text-red-500 mb-6 text-sm">{err_msg}</p>
                                    <A href="/" class="btn-primary">"Go Back"</A>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        match token_data.get() {
                            Some(td) => {
                                view! {
                                    <VideoRoom
                                        channel=td.channel
                                        token=td.token
                                        app_id=td.app_id
                                        uid=td.uid
                                        on_leave=|| {}
                                    />
                                }.into_view()
                            }
                            None => view! { <div></div> }.into_view()
                        }
                    }
                }}
            </div>
        </div>
    }
}
