use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    let location = use_location();

    let nav_items = vec![
        ("/", "Home", "\u{2302}", "Dashboard & quick actions"),
        ("/voice", "Voice Calls", "\u{1F4DE}", "Make & manage calls"),
        ("/messages", "Texts", "\u{1F4AC}", "SMS, WhatsApp, MMS"),
        ("/recordings", "Recordings", "\u{1F4C1}", "Meeting recordings"),
    ];

    view! {
        <aside class="glow-sidebar w-64 flex flex-col h-screen animate-fade-in-left">
            // Brand header with metallic gradient
            <div class="p-6 border-b border-beige-100 relative overflow-hidden">
                <div class="absolute inset-0 bg-gradient-to-br from-teal-500/5 to-beige-100/30 pointer-events-none"></div>
                <div class="relative">
                    <h1 class="text-2xl font-bold metallic-text leading-tight">"Techub"</h1>
                    <p class="text-sm text-gray-500 mt-1">"Comms Platform"</p>
                    <div class="mt-2 flex items-center gap-2">
                        <div class="status-online animate-status-online"></div>
                        <span class="text-xs text-gray-400">"Connected"</span>
                    </div>
                </div>
            </div>

            // Navigation with glow-tab effects
            <nav class="flex-1 p-4 space-y-1 stagger-children overflow-y-auto">
                {nav_items.into_iter().map(|(path, label, icon, desc)| {
                    let current = location.pathname.get();
                    let is_active = current == path || (path != "/" && current.starts_with(path));

                    view! {
                        <A
                            href=path
                            class=move || {
                                let base = "glow-tab animate-nav-item";
                                if is_active { format!("{} active", base) } else { base.to_string() }
                            }
                        >
                            <span class="text-xl w-8 h-8 flex items-center justify-center rounded-lg transition-all duration-200"
                                style=if is_active {
                                    "background: linear-gradient(135deg, rgba(0,153,153,0.15), rgba(0,153,153,0.05))"
                                } else { "" }>
                                {icon}
                            </span>
                            <div class="flex-1 min-w-0">
                                <span class="block text-sm font-medium">{label}</span>
                                <span class="block text-xs text-gray-400 truncate">{desc}</span>
                            </div>
                            {if is_active {
                                view! { <div class="w-1.5 h-1.5 rounded-full bg-teal-500 animate-glow"></div> }
                            } else {
                                view! { <div></div> }
                            }}
                        </A>
                    }
                }).collect_view()}
            </nav>

            // User profile section with metallic card
            <div class="p-4 border-t border-beige-100">
                <div class="glow-card p-3 flex items-center gap-3">
                    <div class="w-10 h-10 rounded-full flex items-center justify-center text-white text-sm font-bold shadow-md"
                        style="background: linear-gradient(135deg, #009999, #005c5c)">
                        "U"
                    </div>
                    <div class="flex-1 min-w-0">
                        <p class="text-sm font-semibold text-gray-900 truncate">"User"</p>
                        <div class="flex items-center gap-1.5">
                            <div class="status-online animate-status-online" style="width:6px;height:6px"></div>
                            <p class="text-xs text-gray-500">"Online"</p>
                        </div>
                    </div>
                    <button class="btn-icon text-gray-400 hover:text-teal-600 text-sm">"\u{2699}"</button>
                </div>
            </div>
        </aside>
    }
}
