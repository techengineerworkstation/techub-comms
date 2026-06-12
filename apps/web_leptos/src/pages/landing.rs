use leptos::*;
use leptos_router::*;

#[component]
pub fn LandingPage() -> impl IntoView {
    let (room_name, set_room_name) = create_signal(String::new());
    let navigate = use_navigate();

    let actions = vec![
        ("Start Video Call", "\u{1F4F9}", "from-teal-500 to-teal-600", "High-quality video conferencing"),
        ("Voice Call", "\u{1F4DE}", "from-beige-400 to-beige-600", "Crystal clear voice calls"),
        ("Send Text", "\u{1F4AC}", "from-teal-600 to-teal-700", "SMS, WhatsApp, and MMS"),
        ("Group Meeting", "\u{1F465}", "from-beige-500 to-beige-700", "Team collaboration rooms"),
    ];

    view! {
        <div class="max-w-4xl mx-auto">
            <div class="text-center mb-12 animate-fade-in-up">
                <h1 class="text-4xl font-bold text-gray-900 mb-4 leading-tight">
                    "Welcome to "
                    <span class="metallic-text">"Techub Comms"</span>
                </h1>
                <p class="text-lg text-gray-600 max-w-2xl mx-auto">
                    "Connect with your team through voice, text and video. Enterprise-grade communications at your utilization."
                </p>
            </div>

            <div class="glow-card p-8 mb-8 animate-fade-in-up" style="animation-delay: 100ms">
                <div class="flex items-center gap-3 mb-6">
                    <div class="w-10 h-10 rounded-lg flex items-center justify-center text-white"
                        style="background: linear-gradient(135deg, #009999, #005c5c)">
                        <span class="text-xl">"\u{1F3A5}"</span>
                    </div>
                    <div>
                        <h2 class="text-xl font-semibold text-gray-900">"Join a Meeting"</h2>
                        <p class="text-sm text-gray-500">"Enter a room name to join or create a meeting"</p>
                    </div>
                </div>

                <form on:submit={
                    let nav = navigate.clone();
                    move |ev: web_sys::SubmitEvent| {
                        ev.prevent_default();
                        let r = room_name.get();
                        if !r.trim().is_empty() {
                            nav(&format!("/meeting/{}", r.trim()), Default::default());
                        }
                    }
                } class="flex gap-4">
                    <input type="text"
                        placeholder="Enter room name..."
                        prop:value=room_name
                        on:input=move |ev| set_room_name.set(event_target_value(&ev))
                        class="input flex-1 text-base"
                    />
                    <button type="submit" class="btn-primary px-8 text-base">
                        "Join Room"
                    </button>
                </form>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 stagger-children mb-8">
                {actions.into_iter().map(|(label, icon, gradient, desc)| {
                    let nav = navigate.clone();
                    let lbl = label.to_string();
                    view! {
                        <button
                            class="glow-card p-6 text-left group cursor-pointer"
                            on:click=move |_| {
                                let path = if lbl == "Start Video Call" {
                                    format!("/meeting/room-{}", (js_sys::Date::now() as u64) % 100000)
                                } else if lbl == "Voice Call" { "/voice".to_string() }
                                else if lbl == "Send Text" { "/messages".to_string() }
                                else { "/meeting/team-standup".to_string() };
                                nav(&path, Default::default());
                            }
                        >
                            <div class=format!(
                                "w-14 h-14 rounded-xl flex items-center justify-center mb-4 bg-gradient-to-br {} shadow-md group-hover:shadow-lg transition-all duration-300 group-hover:scale-110",
                                gradient
                            )>
                                <span class="text-white text-2xl">{icon}</span>
                            </div>
                            <h3 class="font-semibold text-gray-900 mb-1">{label}</h3>
                            <p class="text-sm text-gray-500">{desc}</p>
                        </button>
                    }
                }).collect_view()}
            </div>

            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 animate-fade-in-up" style="animation-delay: 400ms">
                <div class="glow-card p-4 text-center">
                    <p class="text-2xl font-bold metallic-text">"HD"</p>
                    <p class="text-xs text-gray-500 mt-1">"Video Quality"</p>
                </div>
                <div class="glow-card p-4 text-center">
                    <p class="text-2xl font-bold metallic-text">"E2E"</p>
                    <p class="text-xs text-gray-500 mt-1">"Encryption"</p>
                </div>
                <div class="glow-card p-4 text-center">
                    <p class="text-2xl font-bold metallic-text">"6+"</p>
                    <p class="text-xs text-gray-500 mt-1">"Participants"</p>
                </div>
                <div class="glow-card p-4 text-center">
                    <p class="text-2xl font-bold metallic-text">"24/7"</p>
                    <p class="text-xs text-gray-500 mt-1">"Availability"</p>
                </div>
            </div>
        </div>
    }
}
