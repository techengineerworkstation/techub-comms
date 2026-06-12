use leptos::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;

#[component]
pub fn VoicePage() -> impl IntoView {
    let (phone, set_phone) = create_signal(String::new());
    let (channel, set_channel) = create_signal(String::new());
    let (active_call, set_active_call) = create_signal(None::<String>);
    let (call_id, set_call_id) = create_signal(None::<String>);
    let (status, set_status) = create_signal(None::<String>);
    let (is_muted, set_muted) = create_signal(false);
    let (region, set_region) = create_signal("AREA_CODE_NA".to_string());

    let regions = vec![
        ("AREA_CODE_NA", "North America (+1)"),
        ("AREA_CODE_EU", "Europe (+44)"),
        ("AREA_CODE_AS", "Asia (+86)"),
        ("AREA_CODE_JP", "Japan (+81)"),
        ("AREA_CODE_IN", "India (+91)"),
    ];

    view! {
        <div class="max-w-2xl mx-auto animate-page-enter">
            <div class="flex items-center gap-3 mb-8">
                <div class="w-12 h-12 rounded-xl flex items-center justify-center text-white"
                    style="background: linear-gradient(135deg, #c4a06a, #8c7340)">
                    <span class="text-2xl">"\u{1F4DE}"</span>
                </div>
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">"Voice Calls"</h1>
                    <p class="text-sm text-gray-500">"PSTN phone calls via Agora SIP Gateway"</p>
                </div>
            </div>

            // PSTN Outbound Call Card
            <div class="glow-card p-8 mb-6 animate-fade-in-up">
                <h2 class="text-xl font-semibold mb-6 flex items-center gap-2">
                    <span>"\u{1F4DE}"</span> "Call a Phone Number"
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
                            prop:value=channel
                            on:input=move |ev| set_channel.set(event_target_value(&ev))
                            class="input text-base"
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1.5">"Region"</label>
                        <select
                            class="input"
                            on:change=move |ev| set_region.set(event_target_value(&ev))
                        >
                            {regions.into_iter().map(|(code, name)| {
                                view! { <option value=code>{name}</option> }
                            }).collect_view()}
                        </select>
                    </div>
                    <div class="grid grid-cols-2 gap-3">
                        <button class="btn-primary flex items-center justify-center gap-2 py-3"
                            on:click=move |_| {
                                let to = phone.get();
                                let ch = channel.get();
                                let r = region.get();
                                if to.is_empty() || ch.is_empty() { return; }
                                set_status.set(Some("Initiating PSTN call...".into()));
                                spawn_local(async move {
                                    match api::pstn_outbound_call(&to, "+1800222333", &ch, &r).await {
                                        Ok(resp) => {
                                            if resp.success {
                                                set_active_call.set(Some(ch.clone()));
                                                set_call_id.set(resp.call_id);
                                                set_status.set(Some("Phone call connected!".into()));
                                            } else {
                                                set_status.set(Some(format!("Call failed: {}", resp.reason.unwrap_or_default())));
                                            }
                                        }
                                        Err(e) => set_status.set(Some(format!("Error: {}", e))),
                                    }
                                });
                            }>
                            <span>"\u{1F4DE}"</span> "Call Phone"
                        </button>
                        <button class="btn-secondary flex items-center justify-center gap-2 py-3"
                            on:click=move |_| {
                                let ch = channel.get();
                                let r = region.get();
                                if ch.is_empty() { return; }
                                spawn_local(async move {
                                    match api::pstn_inbound(&ch, &r).await {
                                        Ok(resp) => {
                                            set_status.set(Some(format!("Dial {} and enter PIN: {}", resp.display, resp.pin)));
                                        }
                                        Err(e) => set_status.set(Some(format!("Error: {}", e))),
                                    }
                                });
                            }>
                            <span>"\u{1F511}"</span> "Get Inbound Number"
                        </button>
                    </div>
                </div>
            </div>

            // Active Call Panel
            {move || if active_call.get().is_some() {
                view! {
                    <div class="glow-card p-8 mb-6 animate-scale-in">
                        <div class="flex items-center justify-between mb-6">
                            <h2 class="text-xl font-semibold flex items-center gap-2">
                                <span class="w-3 h-3 bg-green-400 rounded-full animate-status-online"></span>
                                "Active PSTN Call"
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
                                    if let Some(cid) = call_id.get() {
                                        spawn_local(async move {
                                            let _ = api::pstn_end_call(&cid).await;
                                        });
                                    }
                                    set_active_call.set(None);
                                    set_call_id.set(None);
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

            // PSTN Info Card
            <div class="glow-card p-6 animate-fade-in-up" style="animation-delay: 200ms">
                <h3 class="font-semibold text-gray-900 mb-3 flex items-center gap-2">
                    <span>"\u{2139}"</span> "PSTN Calling"
                </h3>
                <div class="space-y-2 text-sm text-gray-600">
                    <p>"<strong>Outbound:</strong> Enter a phone number and channel name. The system will call the phone and connect them to your Agora channel."</p>
                    <p>"<strong>Inbound:</strong> Get a phone number and PIN that others can dial to join your channel from a regular phone."</p>
                    <p>"<strong>Regions:</strong> Choose the region closest to where the call recipient is located for best call quality."</p>
                </div>
            </div>
        </div>
    }
}
