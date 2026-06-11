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
    let (dtmf_buf, set_dtmf_buf) = create_signal(String::new());

    let handle_call = move |ct: &'static str| {
        let to = phone.get();
        let t = text.get();
        if to.is_empty() { set_status.set(Some("Please enter a phone number".into())); return; }
        set_status.set(Some("Initiating call...".into()));
        spawn_local(async move {
            let r = match ct {
                "simple" => api::create_call(&to, Some(&t)).await,
                "ivr" => api::create_call(&to, None).await,
                _ => api::create_call(&to, Some("Conference room")).await,
            };
            match r {
                Ok(c) => {
                    set_active_call.set(Some(c.uuid));
                    set_status.set(Some("Connected".into()));
                }
                Err(e) => set_status.set(Some(format!("Error: {}", e))),
            }
        });
    };

    view! {
        <div class="max-w-2xl mx-auto animate-page-enter">
            <div class="flex items-center gap-3 mb-8">
                <div class="w-12 h-12 rounded-xl flex items-center justify-center text-white"
                    style="background: linear-gradient(135deg, #c4a06a, #8c7340)">
                    <span class="text-2xl">"\u{1F4DE}"</span>
                </div>
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">"Voice Calls"</h1>
                    <p class="text-sm text-gray-500">"Make calls, send TTS, and manage IVR"</p>
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
                        <label class="block text-sm font-medium text-gray-700 mb-1.5">"TTS Message"</label>
                        <textarea
                            placeholder="Enter text to speak..."
                            prop:value=text
                            on:input=move |ev| set_text.set(event_target_value(&ev))
                            class="input min-h-[100px]"
                        ></textarea>
                    </div>
                    <div class="grid grid-cols-3 gap-3">
                        <button class="btn-primary flex items-center justify-center gap-2 py-3"
                            on:click=move |_| handle_call("simple")>
                            <span>"\u{1F4DE}"</span> "Call"
                        </button>
                        <button class="btn-secondary flex items-center justify-center gap-2 py-3"
                            on:click=move |_| handle_call("ivr")>
                            <span>"\u{260E}"</span> "IVR"
                        </button>
                        <button class="btn-secondary flex items-center justify-center gap-2 py-3"
                            on:click=move |_| handle_call("conference")>
                            <span>"\u{1F465}"</span> "Conference"
                        </button>
                    </div>
                </div>
            </div>

            {move || -> View {
                if active_call.get().is_some() {
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

                            <div class="flex gap-3 mb-6">
                                <button
                                    class=move || if is_muted.get() {
                                        "flex-1 flex items-center justify-center gap-2 py-3 rounded-lg bg-red-100 text-red-600 font-medium transition-all duration-200 hover:bg-red-200"
                                    } else {
                                        "flex-1 flex items-center justify-center gap-2 py-3 rounded-lg bg-beige-100 text-gray-700 font-medium transition-all duration-200 hover:bg-beige-200"
                                    }
                                    on:click=move |_| {
                                        if let Some(uuid) = active_call.get() {
                                            let a = if is_muted.get() { "unmute" } else { "mute" };
                                            spawn_local(async move { let _ = api::modify_call(&uuid, a).await; });
                                            set_muted.update(|v| *v = !*v);
                                        }
                                    }
                                >
                                    <span class="text-lg">{move || if is_muted.get() { "\u{1F507}" } else { "\u{1F3A4}" }}</span>
                                    {move || if is_muted.get() { "Unmute" } else { "Mute" }}
                                </button>

                                <button class="flex-1 flex items-center justify-center gap-2 py-3 rounded-lg bg-teal-100 text-teal-700 font-medium transition-all duration-200 hover:bg-teal-200"
                                    on:click=move |_| {
                                        if let (Some(uuid), t) = (active_call.get(), text.get()) {
                                            spawn_local(async move { let _ = api::play_tts(&uuid, &t).await; });
                                        }
                                    }
                                >
                                    <span class="text-lg">"\u{1F50A}"</span> "Play TTS"
                                </button>

                                <button class="btn-danger flex-1 flex items-center justify-center gap-2 py-3"
                                    on:click=move |_| {
                                        if let Some(uuid) = active_call.get() {
                                            spawn_local(async move { let _ = api::modify_call(&uuid, "hangup").await; });
                                            set_active_call.set(None);
                                            set_status.set(Some("Call ended".into()));
                                        }
                                    }
                                >
                                    <span class="text-lg">"\u{1F6AB}"</span> "Hang Up"
                                </button>
                            </div>

                            <div class="border-t border-beige-100 pt-6">
                                <h3 class="text-sm font-medium text-gray-700 mb-3">"DTMF Keypad"</h3>
                                <div class="grid grid-cols-4 gap-2 mb-4">
                                    {["1","2","3","4","5","6","7","8","9","*","0","#"].iter().map(|d| {
                                        let ds = d.to_string();
                                        view! {
                                            <button
                                                class="p-4 rounded-lg font-mono text-xl font-bold transition-all duration-150 animate-keypress cursor-pointer"
                                                style="background: linear-gradient(145deg, #f5ead6, #ecdcb8); box-shadow: var(--shadow-metallic)"
                                                on:click=move |_| set_dtmf_buf.update(|v| v.push_str(&ds))
                                            >
                                                {*d}
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>

                                {move || -> View {
                                    if !dtmf_buf.get().is_empty() {
                                        view! {
                                            <div class="flex gap-2 animate-fade-in-up">
                                                <input type="text"
                                                    prop:value=dtmf_buf
                                                    readonly
                                                    class="input flex-1 font-mono text-lg tracking-widest"
                                                />
                                                <button class="btn-primary px-6" on:click=move |_| {
                                                    if let Some(uuid) = active_call.get() {
                                                        let d = dtmf_buf.get();
                                                        spawn_local(async move { let _ = api::send_dtmf_api(&uuid, &d).await; });
                                                        set_dtmf_buf.set(String::new());
                                                    }
                                                }>"Send"</button>
                                                <button class="btn-secondary px-4" on:click=move |_| set_dtmf_buf.set(String::new())>"Clear"</button>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! { <div></div> }.into_view()
                                    }
                                }}
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
                }
            }}
        </div>
    }
}
