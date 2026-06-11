use leptos::*;
use leptos_router::*;
use crate::components::{sidebar::Sidebar, header::Header};

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div class="flex h-screen bg-beige-50 overflow-hidden">
            <Sidebar/>
            <div class="flex-1 flex flex-col overflow-hidden">
                <div class="relative">
                    <Header/>
                </div>
                <main class="flex-1 overflow-auto p-6 animate-page-enter">
                    <Outlet/>
                </main>
            </div>
        </div>
    }
}
