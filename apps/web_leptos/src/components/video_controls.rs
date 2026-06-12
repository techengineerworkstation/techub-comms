use leptos::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;

#[component]
pub fn VideoControls(
    is_muted: ReadSignal<bool>,
    set_muted: WriteSignal<bool>,
    is_camera_off: ReadSignal<bool>,
    set_camera_off: WriteSignal<bool>,
    is_screen_sharing: ReadSignal<bool>,
    set_screen_sharing: WriteSignal<bool>,
    is_recording: ReadSignal<bool>,
    set_recording: WriteSignal<bool>,
    is_chat_open: ReadSignal<bool>,
    set_chat_open: WriteSignal<bool>,
    is_participants_open: ReadSignal<bool>,
    set_participants_open: WriteSignal<bool>,
    channel: String,
    on_leave: impl Fn() + 'static,
) -> impl IntoView {
    let channel_for_rec = channel.clone();

    view! {
        <div class="h-24 bg-white border-t border-beige-100 flex items-center justify-center gap-2 px-6"
            style="background: linear-gradient(180deg, #ffffff 0%, #fdf8f0 100%)">

            <button
                class=move || if is_muted.get() {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-gradient-to-br from-red-500 to-red-600 text-white shadow-md hover:shadow-lg transition-all duration-200 cursor-pointer"
                } else {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-white text-gray-600 border border-beige-200 hover:bg-beige-50 hover:shadow-md transition-all duration-200 cursor-pointer"
                }
                on:click=move |_| set_muted.update(|v| *v = !*v)
            >
                <span class="text-xl">{move || if is_muted.get() { "\u{1F507}" } else { "\u{1F3A4}" }}</span>
                <span class="text-xs">{move || if is_muted.get() { "Unmute" } else { "Mute" }}</span>
            </button>

            <button
                class=move || if is_camera_off.get() {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-gradient-to-br from-red-500 to-red-600 text-white shadow-md hover:shadow-lg transition-all duration-200 cursor-pointer"
                } else {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-white text-gray-600 border border-beige-200 hover:bg-beige-50 hover:shadow-md transition-all duration-200 cursor-pointer"
                }
                on:click=move |_| set_camera_off.update(|v| *v = !*v)
            >
                <span class="text-xl">{move || if is_camera_off.get() { "\u{1F4F7}" } else { "\u{1F4F9}" }}</span>
                <span class="text-xs">{move || if is_camera_off.get() { "Start Cam" } else { "Stop Cam" }}</span>
            </button>

            <button
                class=move || if is_screen_sharing.get() {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-gradient-to-br from-teal-500 to-teal-600 text-white shadow-md hover:shadow-lg transition-all duration-200 cursor-pointer"
                } else {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-white text-gray-600 border border-beige-200 hover:bg-beige-50 hover:shadow-md transition-all duration-200 cursor-pointer"
                }
                on:click=move |_| set_screen_sharing.update(|v| *v = !*v)
            >
                <span class="text-xl">"\u{1F5A5}"</span>
                <span class="text-xs">{move || if is_screen_sharing.get() { "Stop Share" } else { "Share" }}</span>
            </button>

            <button
                class=move || if is_recording.get() {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-gradient-to-br from-red-500 to-red-600 text-white shadow-md hover:shadow-lg transition-all duration-200 cursor-pointer"
                } else {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-white text-gray-600 border border-beige-200 hover:bg-beige-50 hover:shadow-md transition-all duration-200 cursor-pointer"
                }
                on:click=move |_| {
                    let ch = channel_for_rec.clone();
                    if is_recording.get() {
                        spawn_local(async move { let _ = api::stop_recording("", "").await; });
                        set_recording.set(false);
                    } else {
                        spawn_local(async move { let _ = api::start_recording(&ch, 0).await; });
                        set_recording.set(true);
                    }
                }
            >
                <span class=move || if is_recording.get() { "text-xl animate-recording" } else { "text-xl" }>
                    "\u{23FA}"
                </span>
                <span class="text-xs">{move || if is_recording.get() { "Stop Rec" } else { "Record" }}</span>
            </button>

            <div class="mx-2 h-12 w-px bg-beige-200"></div>

            <button
                class=move || if is_participants_open.get() {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-gradient-to-br from-teal-500 to-teal-600 text-white shadow-md hover:shadow-lg transition-all duration-200 cursor-pointer"
                } else {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-white text-gray-600 border border-beige-200 hover:bg-beige-50 hover:shadow-md transition-all duration-200 cursor-pointer"
                }
                on:click=move |_| {
                    set_participants_open.update(|v| *v = !*v);
                    if is_participants_open.get() { set_chat_open.set(false); }
                }
            >
                <span class="text-xl">"\u{1F465}"</span>
                <span class="text-xs">"People"</span>
            </button>

            <button
                class=move || if is_chat_open.get() {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-gradient-to-br from-teal-500 to-teal-600 text-white shadow-md hover:shadow-lg transition-all duration-200 cursor-pointer"
                } else {
                    "flex flex-col items-center gap-1 p-3 rounded-xl bg-white text-gray-600 border border-beige-200 hover:bg-beige-50 hover:shadow-md transition-all duration-200 cursor-pointer"
                }
                on:click=move |_| {
                    set_chat_open.update(|v| *v = !*v);
                    if is_chat_open.get() { set_participants_open.set(false); }
                }
            >
                <span class="text-xl">"\u{1F4AC}"</span>
                <span class="text-xs">"Chat"</span>
            </button>

            <div class="mx-2 h-12 w-px bg-beige-200"></div>

            <button
                class="flex flex-col items-center gap-1 p-3 rounded-xl bg-gradient-to-br from-red-500 to-red-600 text-white shadow-md hover:shadow-lg transition-all duration-200 cursor-pointer"
                on:click=move |_| on_leave()
            >
                <span class="text-xl">"\u{1F6AB}"</span>
                <span class="text-xs">"Leave"</span>
            </button>
        </div>
    }
}
