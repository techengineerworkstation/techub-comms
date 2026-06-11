use leptos::*;

#[derive(Clone, Debug)]
pub struct Toast {
    pub id: u32,
    pub message: String,
    pub toast_type: ToastType,
}

#[derive(Clone, Debug)]
pub enum ToastType {
    Success,
    Error,
    Info,
}

#[component]
pub fn ToastContainer(toasts: ReadSignal<Vec<Toast>>, set_toasts: WriteSignal<Vec<Toast>>) -> impl IntoView {
    view! {
        <div class="fixed top-4 right-4 z-50 space-y-2">
            <For
                each=move || toasts.get()
                key=|t| t.id
                children=move |toast| {
                    let id = toast.id;
                    let (exiting, set_exiting) = create_signal(false);
                    let bg = match toast.toast_type {
                        ToastType::Success => "bg-gradient-to-r from-teal-500 to-teal-600",
                        ToastType::Error => "bg-gradient-to-r from-red-500 to-red-600",
                        ToastType::Info => "bg-gradient-to-r from-beige-500 to-beige-600",
                    };
                    let icon = match toast.toast_type {
                        ToastType::Success => "\u{2705}",
                        ToastType::Error => "\u{274C}",
                        ToastType::Info => "\u{2139}",
                    };

                    // Auto-dismiss after 4s
                    set_timeout(move || {
                        set_exiting.set(true);
                        set_timeout(move || {
                            set_toasts.update(|t| t.retain(|x| x.id != id));
                        }, std::time::Duration::from_millis(300));
                    }, std::time::Duration::from_millis(4000));

                    view! {
                        <div class=format!(
                            "{} {} rounded-lg px-4 py-3 text-white text-sm font-medium shadow-lg flex items-center gap-2 min-w-[280px]",
                            bg,
                            if exiting.get() { "animate-toast-out" } else { "animate-toast-in" }
                        )>
                            <span>{icon}</span>
                            <span class="flex-1">{toast.message}</span>
                            <button
                                class="text-white/70 hover:text-white transition-colors"
                                on:click=move |_| {
                                    set_exiting.set(true);
                                    set_timeout(move || {
                                        set_toasts.update(|t| t.retain(|x| x.id != id));
                                    }, std::time::Duration::from_millis(300));
                                }
                            >
                                "\u{2715}"
                            </button>
                        </div>
                    }
                }
            />
        </div>
    }
}
