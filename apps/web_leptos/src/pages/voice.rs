use leptos::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;

#[component]
pub fn VoicePage() -> impl IntoView {
    let (phone, set_phone) = create_signal(String::new());
    let (text, set_text) = create_signal(String::new());
    let (active_call, set_active_call) = create_signal(None::<String>);
    let (status, set_status) = create_signal(None::<String>);
    let (is_muted, set_muted) = create_signal(false);

    view! {
        <div class="max-w-2xl mx-auto animate-page-enter">
            <div class="flex items-center gap-3 mb-8">
                <div class="w-12 h-12 rounded-xl flex items-center justify-center text-white"
                    style="background: linear-gradient(135deg, #c4a06a, #8c7340)">
                    <span class="text-2xl">"\u{1F4DE}"</span>
                </div>
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">"Voice Calls"</h1>
                    <p class="text-sm text-gray-500">"Agora-powered voice communications"</p>
                </div>
            </div>

            <div class="glow-card p-8 mb-6 animate-fade-in-up">
                <h2 class="text-xl font-semibold mb-6 flex items-center gap-2">
                    <span>"\u{1F4DE}"</span> "Make a Call"
                </h2>
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1.5">"Phone Number"</label>
                        <input type="tel"
                            placeholder="+1 234 567 8901"
                            prop:value=phone
                            on:input=move |ev| set_phone.set(event_target_value(&ev))
                            class="input text-base"
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1.5">"Channel Name"</label>
                        <input type="text"
                            placeholder="Enter channel name"
                            prop:value=text
                            on:input=move |ev| set_text.set(event_target_value(&ev))
                            class="input text-base"
                        />
                    </div>
                    <div class="grid grid-cols-2 gap-3">
                        <button class="btn-primary flex items-center justify-center gap-2 py-3"
                            on:click=move |_| {
                                let to = phone.get();
                                let ch = text.get();
                                if to.is_empty() { return; }
                                set_status.set(Some("Initiating call...".into()));
                                spawn_local(async move {
                                    match api::initiate_call(&to, Some(&ch)).await {
                                        Ok(c) => {
                                            set_active_call.set(Some(c.channel));
                                            set_status.set(Some("Connected".into()));
                                        }
                                        Err(e) => set_status.set(Some(format!("Error: {}", e))),
                                    }
                                });
                            }>
                            <span>"\u{1F4DE}"</span> "Voice Call"
                        </button>
                        <button class="btn-secondary flex items-center justify-center gap-2 py-3"
                            on:click=move |_| {
                                let ch = text.get();
                                if ch.is_empty() { return; }
                                set_active_call.set(Some(ch));
                                set_status.set(Some("Channel joined".into()));
                            }>
                            <span>"\u{1F465}"</span> "Join Channel"
                        </button>
                    </div>
                </div>
            </div>

            {move || if active_call.get().is_some() {
                view! {
                    <div class="glow-card p-8 mb-6 animate-scale-in">
                        <div class="flex items-center justify-between mb-6">
                            <h2 class="text-xl font-semibold flex items-center gap-2">
                                <span class="w-3 h-3 bg-green-400 rounded-full animate-status-online"></span>
                                "Active Call"
                            </h2>
                            <span class="turquoise-badge animate-glow">"Connected"</span>
                        </div>

                        {move || status.get().map(|s| {
                            view! { <p class="text-sm text-teal-600 mb-4 flex items-center gap-2"><span>"\u{2139}"</span>{s}</p> }
                        })}

                        <div class="flex gap-3">
                            <button
                                class=move || if is_muted.get() {
                                    "flex-1 flex items-center justify-center gap-2 py-3 rounded-lg bg-red-100 text-red-600 font-medium transition-all duration-200 hover:bg-red-200"
                                } else {
                                    "flex-1 flex items-center justify-center gap-2 py-3 rounded-lg bg-beige-100 text-gray-700 font-medium transition-all duration-200 hover:bg-beige-200"
                                }
                                on:click=move |_| set_muted.update(|v| *v = !*v)
                            >
                                <span class="text-lg">{move || if is_muted.get() { "\u{1F507}" } else { "\u{1F3A4}" }}</span>
                                {move || if is_muted.get() { "Unmute" } else { "Mute" }}
                            </button>

                            <button class="btn-danger flex-1 flex items-center justify-center gap-2 py-3"
                                on:click=move |_| {
                                    set_active_call.set(None);
                                    set_status.set(Some("Call ended".into()));
                                }
                            >
                                <span class="text-lg">"\u{1F6AB}"</span> "Hang Up"
                            </button>
                        </div>
                    </div>
                }.into_view()
            } else {
                (move || status.get().map(|s| {
                    view! {
                        <div class="glow-card p-4 text-center text-sm text-gray-600 animate-fade-in-up">
                            {s}
                        </div>
                    }.into_view()
                }).unwrap_or_else(|| view! { <div></div> }.into_view()))()
            }}
        </div>
    }
}
