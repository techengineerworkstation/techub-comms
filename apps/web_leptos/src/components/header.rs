use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let (show_notifications, set_show_notifications) = create_signal(false);

    view! {
        <header class="glow-header h-16 flex items-center justify-between px-6 animate-fade-in-down">
            <div class="flex items-center gap-3">
                <h2 class="text-lg font-semibold text-gray-900">"Welcome to Techub Comms"</h2>
                <span class="turquoise-badge animate-pop-in">"v1.0"</span>
            </div>

            <div class="flex items-center gap-3">
                // Search bar with glow focus
                <div class="relative hidden md:block">
                    <input type="text"
                        placeholder="Search..."
                        class="input pl-9 w-48 text-sm"
                        style="padding-top:0.375rem;padding-bottom:0.375rem"
                    />
                    <span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm">"\u{1F50D}"</span>
                </div>

                // Notification bell with glow
                <button
                    class="btn-icon relative text-gray-500 hover:text-teal-600 transition-colors duration-200"
                    on:click=move |_| set_show_notifications.update(|v| *v = !*v)
                >
                    <span class="text-lg">"\u{1F514}"</span>
                    <span class="absolute -top-0.5 -right-0.5 w-4 h-4 bg-red-500 text-white text-xs rounded-full flex items-center justify-center animate-bounce-in">
                        "3"
                    </span>
                </button>

                // Settings with glow
                <button class="btn-icon text-gray-500 hover:text-teal-600 transition-colors duration-200">
                    <span class="text-lg">"\u{2699}"</span>
                </button>

                // User avatar with metallic border
                <div class="w-8 h-8 rounded-full flex items-center justify-center text-white text-xs font-bold shadow-md ml-2 cursor-pointer hover:shadow-glow-teal transition-shadow duration-300"
                    style="background: linear-gradient(135deg, #009999, #005c5c); border: 2px solid var(--color-beige-200)">
                    "U"
                </div>
            </div>
        </header>

        // Notification dropdown
        {move || if show_notifications.get() {
            view! {
                <div class="absolute right-6 top-16 w-80 glow-card p-0 z-50 animate-modal-in overflow-hidden">
                    <div class="p-4 border-b border-beige-100" style="background: var(--gradient-header)">
                        <h3 class="font-semibold text-gray-900">"Notifications"</h3>
                    </div>
                    <div class="max-h-64 overflow-y-auto">
                        <div class="p-3 border-b border-beige-50 hover:bg-beige-50 transition-colors cursor-pointer">
                            <p class="text-sm font-medium text-gray-900">"New text received"</p>
                            <p class="text-xs text-gray-500 mt-1">"2 minutes ago"</p>
                        </div>
                        <div class="p-3 border-b border-beige-50 hover:bg-beige-50 transition-colors cursor-pointer">
                            <p class="text-sm font-medium text-gray-900">"Call recording ready"</p>
                            <p class="text-xs text-gray-500 mt-1">"15 minutes ago"</p>
                        </div>
                        <div class="p-3 hover:bg-beige-50 transition-colors cursor-pointer">
                            <p class="text-sm font-medium text-gray-900">"Team meeting in 30 min"</p>
                            <p class="text-xs text-gray-500 mt-1">"1 hour ago"</p>
                        </div>
                    </div>
                    <div class="p-3 border-t border-beige-100 text-center">
                        <button class="text-sm text-teal-600 hover:text-teal-700 font-medium">"View all"</button>
                    </div>
                </div>
            }
        } else {
            view! { <div></div> }
        }}
    }
}
