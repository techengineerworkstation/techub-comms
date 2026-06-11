use leptos::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::components::video_controls::VideoControls;
use crate::components::chat_panel::ChatPanel;
use crate::components::participant_list::ParticipantList;

#[derive(Clone, Debug)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub has_video: bool,
    pub has_audio: bool,
}

#[component]
pub fn VideoRoom(
    room: String,
    session_id: String,
    token: String,
    api_key: String,
    on_leave: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let (is_recording, set_recording) = create_signal(false);
    let (captions_id, set_captions_id) = create_signal(None::<String>);
    let (is_chat_open, set_chat_open) = create_signal(false);
    let (is_participants_open, set_participants_open) = create_signal(false);
    let (is_muted, set_muted) = create_signal(false);
    let (is_camera_off, set_camera_off) = create_signal(false);
    let (is_screen_sharing, set_screen_sharing) = create_signal(false);
    let participants = create_signal(vec![
        Participant { id: "local".into(), name: "You".into(), has_video: true, has_audio: true },
    ]).0;

    let _room_clone = room.clone();
    spawn_local(async move {
        log::info!("Session ready");
    });

    let on_leave_clone = on_leave.clone();
    let room_for_controls = room.clone();

    view! {
        <div class="flex flex-col h-full animate-page-enter">
            <div class="flex-1 flex overflow-hidden">
                <div class="flex-1 p-4 overflow-auto">
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 h-full auto-rows-fr">
                        <div class="relative bg-gray-900 rounded-xl overflow-hidden aspect-video glow-card animate-video-enter">
                            <div id="publisher" class="w-full h-full flex items-center justify-center">
                                <div class="text-center">
                                    <div class="w-20 h-20 mx-auto rounded-full flex items-center justify-center text-white text-3xl font-bold mb-3"
                                        style="background: linear-gradient(135deg, #009999, #005c5c)">
                                        "U"
                                    </div>
                                    <p class="text-white text-sm opacity-75">"Camera initializing..."</p>
                                </div>
                            </div>
                            <div class="absolute bottom-3 left-3 bg-black/50 backdrop-blur-sm text-white text-sm px-3 py-1.5 rounded-lg flex items-center gap-2">
                                <span class="w-2 h-2 bg-green-400 rounded-full animate-status-online"></span>
                                "You"
                            </div>
                        </div>

                        <div class="relative bg-gray-900 rounded-xl overflow-hidden aspect-video glow-card animate-video-enter" style="animation-delay: 100ms">
                            <div class="w-full h-full flex items-center justify-center">
                                <div class="text-center">
                                    <div class="w-20 h-20 mx-auto rounded-full flex items-center justify-center text-white text-3xl font-bold mb-3 opacity-50"
                                        style="background: linear-gradient(135deg, #c4a06a, #8c7340)">
                                        "?"
                                    </div>
                                    <p class="text-white text-sm opacity-50">"Waiting for participants..."</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                {move || -> View {
                    if is_chat_open.get() {
                        view! {
                            <div class="w-80 border-l border-beige-100 bg-white animate-panel-slide">
                                <ChatPanel/>
                            </div>
                        }.into_view()
                    } else if is_participants_open.get() {
                        view! {
                            <div class="w-80 border-l border-beige-100 bg-white animate-panel-slide">
                                <ParticipantList participants=participants/>
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}
            </div>

            <VideoControls
                is_muted=is_muted
                set_muted=set_muted
                is_camera_off=is_camera_off
                set_camera_off=set_camera_off
                is_screen_sharing=is_screen_sharing
                set_screen_sharing=set_screen_sharing
                is_recording=is_recording
                set_recording=set_recording
                is_chat_open=is_chat_open
                set_chat_open=set_chat_open
                is_participants_open=is_participants_open
                set_participants_open=set_participants_open
                captions_id=captions_id
                set_captions_id=set_captions_id
                room=room_for_controls
                on_leave=on_leave_clone
            />
        </div>
    }
}
