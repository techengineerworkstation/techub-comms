use leptos::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;

#[component]
pub fn MessagesPage() -> impl IntoView {
    let (to, set_to) = create_signal(String::new());
    let (text, set_text) = create_signal(String::new());
    let (channel, set_channel) = create_signal("sms".to_string());
    let (status, set_status) = create_signal(None::<String>);
    let (sending, set_sending) = create_signal(false);

    let channels = vec![
        ("sms", "SMS", "\u{1F4E7}", "from-teal-500 to-teal-600"),
        ("whatsapp", "WhatsApp", "\u{1F4AC}", "from-green-500 to-green-600"),
        ("mms", "MMS", "\u{1F5BC}", "from-beige-400 to-beige-600"),
    ];

    view! {
        <div class="max-w-2xl mx-auto animate-page-enter">
            <div class="flex items-center gap-3 mb-8">
                <div class="w-12 h-12 rounded-xl flex items-center justify-center text-white"
                    style="background: linear-gradient(135deg, #007a7a, #005c5c)">
                    <span class="text-2xl">"\u{1F4AC}"</span>
                </div>
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">"Messages"</h1>
                    <p class="text-sm text-gray-500">"Send SMS, WhatsApp, and MMS messages"</p>
                </div>
            </div>

            <div class="glow-card p-8 animate-fade-in-up">
                <h2 class="text-xl font-semibold mb-6 flex items-center gap-2">
                    <span>"\u{2709}"</span> "Send a Message"
                </h2>

                <div class="flex gap-2 mb-6">
                    {channels.into_iter().map(|(key, label, icon, gradient)| {
                        let k = key.to_string();
                        let k2 = k.clone();
                        view! {
                            <button
                                class=move || {
                                    let base = "flex items-center gap-2 px-5 py-2.5 rounded-lg font-medium text-sm transition-all duration-200";
                                    if channel.get() == k2 {
                                        format!("{} bg-gradient-to-br {} text-white shadow-md", base, gradient)
                                    } else {
                                        format!("{} bg-beige-100 text-gray-700 hover:bg-beige-200 hover:shadow-sm", base)
                                    }
                                }
                                on:click=move |_| set_channel.set(k.clone())
                            >
                                <span>{icon}</span>
                                <span>{label}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>

                <form on:submit=move |ev: web_sys::SubmitEvent| {
                    ev.prevent_default();
                    let t = to.get();
                    let m = text.get();
                    if t.is_empty() || m.is_empty() { return; }
                    set_sending.set(true);
                    let ch = channel.get();
                    spawn_local(async move {
                        let r = match ch.as_str() {
                            "whatsapp" => api::send_whatsapp_api(&t, &m).await,
                            _ => api::send_sms_api(&t, &m).await,
                        };
                        match r {
                            Ok(r) => {
                                set_status.set(Some(format!("Message sent! ID: {}", r.message_id)));
                                set_text.set(String::new());
                            }
                            Err(e) => set_status.set(Some(format!("Error: {}", e))),
                        }
                        set_sending.set(false);
                    });
                } class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1.5">"To"</label>
                        <input type="tel"
                            placeholder="+1 234 567 8901"
                            prop:value=to
                            on:input=move |ev| set_to.set(event_target_value(&ev))
                            class="input text-base"
                            required
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1.5">"Message"</label>
                        <textarea
                            placeholder="Type your message..."
                            prop:value=text
                            on:input=move |ev| set_text.set(event_target_value(&ev))
                            class="input min-h-[120px]"
                            required
                        ></textarea>
                    </div>
                    <button type="submit"
                        class="btn-primary w-full py-3 text-base flex items-center justify-center gap-2"
                        disabled=move || sending.get()
                    >
                        {move || if sending.get() {
                            view! { <span class="animate-spin">"\u{1F504}"</span> <span>"Sending..."</span> }.into_view()
                        } else {
                            view! { <span>"Send Message"</span> }.into_view()
                        }}
                    </button>
                </form>

                {move || status.get().map(|s| {
                    let is_error = s.starts_with("Error");
                    view! {
                        <div class=if is_error {
                            "mt-4 p-4 rounded-lg text-sm bg-red-50 text-red-600 border border-red-200 animate-shake"
                        } else {
                            "mt-4 p-4 rounded-lg text-sm bg-teal-50 text-teal-700 border border-teal-200 animate-fade-in-up"
                        }>
                            <div class="flex items-center gap-2">
                                <span>{if is_error { "\u{274C}" } else { "\u{2705}" }}</span>
                                <span>{s}</span>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
