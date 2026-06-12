use leptos::*;

#[component]
pub fn RecordingsPage() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto animate-page-enter">
            <div class="flex items-center gap-3 mb-8">
                <div class="w-12 h-12 rounded-xl flex items-center justify-center text-white"
                    style="background: linear-gradient(135deg, #c4a06a, #8c7340)">
                    <span class="text-2xl">"\u{1F4C1}"</span>
                </div>
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">"Recordings"</h1>
                    <p class="text-sm text-gray-500">"Agora cloud recording management"</p>
                </div>
            </div>

            <div class="glow-card p-6 mb-6 animate-fade-in-up">
                <div class="flex gap-4">
                    <input type="text"
                        placeholder="Enter channel name to search recordings..."
                        class="input flex-1"
                    />
                    <button class="btn-primary px-6 flex items-center gap-2">
                        <span>"\u{1F50D}"</span> <span>"Search"</span>
                    </button>
                </div>
            </div>

            <div class="glow-card p-12 text-center animate-fade-in-up">
                <div class="w-20 h-20 mx-auto mb-4 rounded-full bg-beige-100 flex items-center justify-center">
                    <span class="text-3xl text-beige-400">"\u{1F4C1}"</span>
                </div>
                <h3 class="text-lg font-semibold text-gray-900 mb-2">"No recordings yet"</h3>
                <p class="text-sm text-gray-500">"Start a meeting and record it to see recordings here"</p>
            </div>
        </div>
    }
}
