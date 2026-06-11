use leptos::*;
use crate::components::video_room::Participant;

#[component]
pub fn ParticipantList(participants: ReadSignal<Vec<Participant>>) -> impl IntoView {
    view! {
        <div class="flex flex-col h-full">
            // Header
            <div class="p-4 border-b border-beige-100" style="background: var(--gradient-header)">
                <h3 class="font-semibold text-gray-900 flex items-center gap-2">
                    <span>"\u{1F465}"</span> "Participants"
                </h3>
                <p class="text-xs text-gray-500 mt-1">
                    {move || format!("{} in meeting", participants.get().len())}
                </p>
            </div>

            // Participant list
            <div class="flex-1 overflow-y-auto p-4 space-y-2">
                <For
                    each=move || participants.get()
                    key=|p| p.id.clone()
                    children=move |p: Participant| {
                        let initials: String = p.name.chars().take(2).collect();
                        let colors = if p.id == "local" {
                            "from-teal-500 to-teal-600"
                        } else {
                            "from-beige-400 to-beige-600"
                        };

                        view! {
                            <div class="glow-card p-3 flex items-center gap-3 animate-fade-in-up">
                                <div class=format!(
                                    "w-10 h-10 rounded-full flex items-center justify-center text-white text-sm font-bold bg-gradient-to-br {} shadow-md",
                                    colors
                                )>
                                    {initials}
                                </div>
                                <div class="flex-1 min-w-0">
                                    <p class="text-sm font-medium text-gray-900 truncate">
                                        {p.name.clone()}
                                        {if p.id == "local" {
                                            view! { <span class="text-xs text-gray-400 ml-1">"(You)"</span> }
                                        } else { view! { <span></span> } }}
                                    </p>
                                    <div class="flex items-center gap-2 mt-0.5">
                                        {if p.has_audio {
                                            view! { <span class="text-xs text-teal-600">"\u{1F3A4} Mic on"</span> }
                                        } else {
                                            view! { <span class="text-xs text-red-400">"\u{1F507} Muted"</span> }
                                        }}
                                        {if p.has_video {
                                            view! { <span class="text-xs text-teal-600">"\u{1F4F9} Cam on"</span> }
                                        } else {
                                            view! { <span class="text-xs text-red-400">"\u{1F4F7} Cam off"</span> }
                                        }}
                                    </div>
                                </div>
                                <button class="btn-icon text-gray-400 hover:text-gray-600 text-sm">"\u{22EF}"</button>
                            </div>
                        }
                    }
                />
            </div>

            // Invite link
            <div class="p-4 border-t border-beige-100">
                <button class="btn-secondary w-full text-sm flex items-center justify-center gap-2">
                    <span>"\u{1F517}"</span> "Copy Invite Link"
                </button>
            </div>
        </div>
    }
}
