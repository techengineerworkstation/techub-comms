use leptos::*;

#[derive(Clone, Debug)]
struct ChatMessage {
    sender: String,
    text: String,
    time: String,
    is_local: bool,
}

#[component]
pub fn ChatPanel() -> impl IntoView {
    let (messages, set_messages) = create_signal(vec![
        ChatMessage { sender: "System".into(), text: "Chat connected via OpenTok signals".into(), time: "now".into(), is_local: false },
    ]);
    let (input_text, set_input_text) = create_signal(String::new());

    view! {
        <div class="flex flex-col h-full">
            // Header
            <div class="p-4 border-b border-beige-100" style="background: var(--gradient-header)">
                <h3 class="font-semibold text-gray-900 flex items-center gap-2">
                    <span>"\u{1F4AC}"</span> "Meeting Chat"
                </h3>
                <p class="text-xs text-gray-500 mt-1">"Messages are sent to all participants"</p>
            </div>

            // Messages list
            <div class="flex-1 overflow-y-auto p-4 space-y-3">
                <For
                    each=move || messages.get()
                    key=|m| m.text.clone()
                    children=move |msg: ChatMessage| {
                        view! {
                            <div class=if msg.is_local { "animate-message-in flex flex-col items-end" } else { "animate-message-in" }>
                                <div class=if msg.is_local {
                                    "bg-gradient-to-br from-teal-500 to-teal-600 text-white rounded-xl rounded-br-sm px-4 py-2.5 max-w-[85%] shadow-md"
                                } else if msg.sender == "System" {
                                    "bg-beige-100 text-gray-600 rounded-xl px-4 py-2 max-w-[85%] text-sm italic"
                                } else {
                                    "bg-white border border-beige-200 text-gray-900 rounded-xl rounded-bl-sm px-4 py-2.5 max-w-[85%] shadow-sm"
                                }>
                                    {if !msg.is_local && msg.sender != "System" {
                                        view! { <p class="text-xs font-semibold text-teal-600 mb-1">{msg.sender.clone()}</p> }.into_view()
                                    } else { view! { <div></div> }.into_view() }}
                                    <p class="text-sm">{msg.text.clone()}</p>
                                </div>
                                <span class="text-xs text-gray-400 mt-1 px-1">{msg.time.clone()}</span>
                            </div>
                        }
                    }
                />
            </div>

            // Input area
            <div class="p-4 border-t border-beige-100">
                <form on:submit=move |ev: web_sys::SubmitEvent| {
                    ev.prevent_default();
                    let text = input_text.get();
                    if !text.trim().is_empty() {
                        set_messages.update(|msgs| msgs.push(ChatMessage {
                            sender: "You".into(),
                            text: text.clone(),
                            time: "now".into(),
                            is_local: true,
                        }));
                        set_input_text.set(String::new());
                    }
                } class="flex gap-2">
                    <input type="text"
                        placeholder="Type a message..."
                        prop:value=input_text
                        on:input=move |ev| set_input_text.set(event_target_value(&ev))
                        class="input flex-1 text-sm"
                    />
                    <button type="submit" class="btn-primary px-4 text-sm">"Send"</button>
                </form>
            </div>
        </div>
    }
}
