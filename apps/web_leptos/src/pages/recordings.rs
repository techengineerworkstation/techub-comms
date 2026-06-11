use leptos::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;

#[component]
pub fn RecordingsPage() -> impl IntoView {
    let (recordings, set_recordings) = create_signal(Vec::<api::ArchiveResp>::new());
    let (loading, set_loading) = create_signal(false);
    let (room_filter, set_room_filter) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);

    view! {
        <div class="max-w-4xl mx-auto animate-page-enter">
            <div class="flex items-center gap-3 mb-8">
                <div class="w-12 h-12 rounded-xl flex items-center justify-center text-white"
                    style="background: linear-gradient(135deg, #c4a06a, #8c7340)">
                    <span class="text-2xl">"\u{1F4C1}"</span>
                </div>
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">"Recordings"</h1>
                    <p class="text-sm text-gray-500">"View and manage your meeting recordings"</p>
                </div>
            </div>

            // Search bar
            <div class="glow-card p-6 mb-6 animate-fade-in-up">
                <div class="flex gap-4">
                    <input type="text"
                        placeholder="Enter room name to search recordings..."
                        prop:value=room_filter
                        on:input=move |ev| set_room_filter.set(event_target_value(&ev))
                        class="input flex-1"
                    />
                    <button class="btn-primary px-6 flex items-center gap-2"
                        disabled=move || loading.get()
                        on:click=move |_| {
                            let rm = room_filter.get();
                            if rm.is_empty() { return; }
                            set_loading.set(true);
                            set_error.set(None);
                            spawn_local(async move {
                                match api::list_archives(&rm).await {
                                    Ok(r) => { set_recordings.set(r.archives); }
                                    Err(e) => { set_error.set(Some(e)); }
                                }
                                set_loading.set(false);
                            });
                        }
                    >
                        {move || if loading.get() {
                            view! { <span class="animate-spin">"\u{1F504}"</span> <span>"Searching..."</span> }.into_view()
                        } else {
                            view! { <span>"\u{1F50D}"</span> <span>"Search"</span> }.into_view()
                        }}
                    </button>
                </div>
            </div>

            // Error message
            {move || error.get().map(|e| {
                view! {
                    <div class="mb-4 p-4 rounded-lg bg-red-50 text-red-600 border border-red-200 text-sm animate-shake">
                        <span>"\u{26A0} " {e}</span>
                    </div>
                }
            })}

            // Recordings list
            <div class="space-y-3 stagger-children">
                {move || {
                    let recs = recordings.get();
                    if recs.is_empty() && !loading.get() {
                        view! {
                            <div class="glow-card p-12 text-center animate-fade-in-up">
                                <div class="w-20 h-20 mx-auto mb-4 rounded-full bg-beige-100 flex items-center justify-center">
                                    <span class="text-3xl text-beige-400">"\u{1F4C1}"</span>
                                </div>
                                <h3 class="text-lg font-semibold text-gray-900 mb-2">"No recordings yet"</h3>
                                <p class="text-sm text-gray-500">"Start a meeting and record it to see recordings here"</p>
                            </div>
                        }.into_view()
                    } else {
                        recs.into_iter().map(|rec| {
                            let status_color = match rec.status.as_str() {
                                "started" => "turquoise-badge",
                                "stopped" | "uploaded" => "metallic-badge",
                                _ => "bg-gray-100 text-gray-600 px-3 py-1 rounded-full text-xs font-semibold",
                            };
                            view! {
                                <div class="glow-card p-5 flex items-center gap-4 animate-fade-in-up">
                                    <div class="w-12 h-12 rounded-xl flex items-center justify-center text-white flex-shrink-0"
                                        style="background: linear-gradient(135deg, #009999, #005c5c)">
                                        <span class="text-xl">"\u{1F3AC}"</span>
                                    </div>
                                    <div class="flex-1 min-w-0">
                                        <h3 class="font-semibold text-gray-900 truncate">{rec.name.clone()}</h3>
                                        <div class="flex items-center gap-3 mt-1">
                                            <span class="text-xs text-gray-500">{rec.created_at.clone()}</span>
                                            {if let Some(dur) = rec.duration {
                                                view! { <span class="text-xs text-gray-400">{format!("{:.1}s", dur)}</span> }
                                            } else {
                                                view! { <span></span> }
                                            }}
                                        </div>
                                    </div>
                                    <span class=status_color>{rec.status.clone()}</span>
                                    <div class="flex gap-2">
                                        {if let Some(url) = rec.url.clone() {
                                            view! {
                                                <a href=url target="_blank" rel="noopener noreferrer"
                                                    class="btn-secondary text-sm px-3 py-1.5">
                                                    "\u{25B6} Play"
                                                </a>
                                            }.into_view()
                                        } else {
                                            view! { <div></div> }.into_view()
                                        }}
                                    </div>
                                </div>
                            }
                        }).collect_view()
                    }
                }}
            </div>
        </div>
    }
}
