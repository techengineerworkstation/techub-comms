use leptos::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

#[component]
pub fn LoginPage(set_authenticated: WriteSignal<bool>, set_token: WriteSignal<Option<String>>) -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);
    let (is_register, set_is_register) = create_signal(false);
    let (display_name, set_display_name) = create_signal(String::new());

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let email_val = email.get();
        let password_val = password.get();
        let name_val = display_name.get();
        let registering = is_register.get();

        if email_val.is_empty() || password_val.is_empty() {
            set_error.set(Some("Email and password are required".into()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            let url = if registering { "/api/auth/register" } else { "/api/auth/login" };
            let body = if registering {
                serde_json::json!({"email": email_val, "password": password_val, "display_name": name_val})
            } else {
                serde_json::json!({"email": email_val, "password": password_val})
            };

            let opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_mode(RequestMode::Cors);
            opts.set_body(&wasm_bindgen::JsValue::from_str(&body.to_string()));

            let req = Request::new_with_str_and_init(url, &opts).unwrap();
            req.headers().set("Content-Type", "application/json").ok();
            req.headers().set("Accept", "application/json").ok();

            let win = web_sys::window().unwrap();
            match JsFuture::from(win.fetch_with_request(&req)).await {
                Ok(resp_val) => {
                    let resp: Response = resp_val.dyn_into().unwrap();
                    if resp.ok() {
                        if let Ok(json_val) = JsFuture::from(resp.json().unwrap()).await {
                            if let Ok(data) = serde_wasm_bindgen::from_value::<serde_json::Value>(json_val) {
                                if let Some(token) = data.get("token").and_then(|t| t.as_str()) {
                                    // Store token in cookie
                                    let doc = web_sys::window().unwrap().document().unwrap();
                                    let html_doc = doc.dyn_into::<web_sys::HtmlDocument>().unwrap();
                                    let cookie = format!("techub_token={}; path=/; max-age=86400; SameSite=Lax", token);
                                    html_doc.set_cookie(&cookie).ok();
                                    set_token.set(Some(token.to_string()));
                                    set_authenticated.set(true);
                                }
                            }
                        }
                    } else {
                        if let Ok(json_val) = JsFuture::from(resp.json().unwrap()).await {
                            if let Ok(data) = serde_wasm_bindgen::from_value::<serde_json::Value>(json_val) {
                                let msg = data.get("error").and_then(|e| e.as_str()).unwrap_or("Authentication failed");
                                set_error.set(Some(msg.to_string()));
                            }
                        } else {
                            set_error.set(Some("Authentication failed".into()));
                        }
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Network error: {:?}", e)));
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center" style="background: linear-gradient(135deg, #fdf8f0 0%, #e6f7f7 50%, #f5ead6 100%)">
            <div class="w-full max-w-md animate-fade-in-up">
                // Logo/Brand
                <div class="text-center mb-8">
                    <div class="w-16 h-16 mx-auto mb-4 rounded-2xl flex items-center justify-center text-white text-3xl font-bold shadow-lg"
                        style="background: linear-gradient(135deg, #009999, #005c5c)">
                        "T"
                    </div>
                    <h1 class="text-3xl font-bold metallic-text">"Techub Comms"</h1>
                    <p class="text-gray-500 mt-2">"Enterprise Communications Platform"</p>
                </div>

                // Login/Register Card
                <div class="glow-card p-8">
                    // Tab switcher
                    <div class="flex mb-6 bg-beige-100 rounded-lg p-1">
                        <button
                            class=move || if !is_register.get() {
                                "flex-1 py-2 rounded-md text-sm font-medium bg-white text-teal-700 shadow-sm transition-all"
                            } else {
                                "flex-1 py-2 rounded-md text-sm font-medium text-gray-500 transition-all"
                            }
                            on:click=move |_| set_is_register.set(false)
                        >
                            "Login"
                        </button>
                        <button
                            class=move || if is_register.get() {
                                "flex-1 py-2 rounded-md text-sm font-medium bg-white text-teal-700 shadow-sm transition-all"
                            } else {
                                "flex-1 py-2 rounded-md text-sm font-medium text-gray-500 transition-all"
                            }
                            on:click=move |_| set_is_register.set(true)
                        >
                            "Register"
                        </button>
                    </div>

                    <form on:submit=handle_submit>
                        {move || if is_register.get() {
                            view! {
                                <div class="mb-4 animate-fade-in-up">
                                    <label class="block text-sm font-medium text-gray-700 mb-1.5">"Display Name"</label>
                                    <input type="text"
                                        placeholder="Your name"
                                        prop:value=display_name
                                        on:input=move |ev| set_display_name.set(event_target_value(&ev))
                                        class="input"
                                    />
                                </div>
                            }.into_view()
                        } else {
                            view! { <div></div> }.into_view()
                        }}

                        <div class="mb-4">
                            <label class="block text-sm font-medium text-gray-700 mb-1.5">"Email"</label>
                            <input type="email"
                                placeholder="your@email.com"
                                prop:value=email
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                class="input"
                                required
                            />
                        </div>

                        <div class="mb-6">
                            <label class="block text-sm font-medium text-gray-700 mb-1.5">"Password"</label>
                            <input type="password"
                                placeholder="Enter your password"
                                prop:value=password
                                on:input=move |ev| set_password.set(event_target_value(&ev))
                                class="input"
                                required
                            />
                        </div>

                        // Error message
                        {move || error.get().map(|e| {
                            view! {
                                <div class="mb-4 p-3 rounded-lg bg-red-50 text-red-600 text-sm border border-red-200 animate-shake">
                                    {e}
                                </div>
                            }
                        })}

                        <button type="submit"
                            class="btn-primary w-full py-3 text-base"
                            disabled=move || loading.get()
                        >
                            {move || if loading.get() {
                                view! { <span class="animate-spin inline-block mr-2">"\u{1F504}"</span> "Processing..." }.into_view()
                            } else if is_register.get() {
                                "Create Account".into_view()
                            } else {
                                "Sign In".into_view()
                            }}
                        </button>
                    </form>
                </div>

                <p class="text-center text-xs text-gray-400 mt-6">
                    "Authorized access only. Contact admin for registration."
                </p>
            </div>
        </div>
    }
}
