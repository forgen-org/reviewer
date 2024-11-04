use icondata as i;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Layout<F, IV>(nav: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <div class="flex overflow-hidden flex-col h-screen">
            <nav class="flex flex-wrap justify-between items-center py-1 px-2 bg-slate-50">
                {nav()} <div class="flex" />
                <button
                    class="p-1.5 text-sm text-center text-white rounded-md border border-transparent shadow-sm transition-all hover:shadow focus:shadow-none active:shadow-none disabled:shadow-none disabled:opacity-50 disabled:pointer-events-none bg-slate-800 hover:bg-slate-700 focus:bg-slate-700 active:bg-slate-700"
                    type="button"
                >
                    <Icon class="w-4 h-4" icon=i::LuSettings />
                </button>
            </nav>
            {children()}
        </div>
    }
}
