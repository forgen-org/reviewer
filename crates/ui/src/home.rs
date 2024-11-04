use charts::SunburstChart;
use client::GitlabClient;
use dotenvy_macro::dotenv;
use leptos::*;
use log::*;
use wasm_bindgen::prelude::*;

use crate::layout::Layout;

#[component]
pub fn Home() -> impl IntoView {
    let (author, set_author) = create_signal("all".to_string());

    let change_requests = create_resource(
        || (),
        |_| async move {
            let client = GitlabClient::new(
                dotenv!("GITLAB_ACCESS_TOKEN").to_string(),
                dotenv!("GITLAB_PROJECT").to_string(),
            );
            client.fetch().await
        },
    );

    let filtered_change_requests = create_memo(move |_| {
        let change_requests = change_requests.get();
        match (change_requests, author.get().as_str()) {
            (Some(change_requests), "all") => Some(change_requests),
            (Some(change_requests), author) => Some(
                change_requests
                    .into_iter()
                    .filter(|value| value.author == author)
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        }
    });

    let on_click = Closure::wrap(Box::new(move |params| {
        let name = js_sys::Reflect::get(&params, &"name".into())
            .expect("Object should have 'name' property")
            .dyn_into::<js_sys::JsString>()
            .expect("'on' should be a string");
        let name = name.as_string().unwrap();
        let mut parts = name.split('/');
        let merge_request_id = parts.next().unwrap();
        let change_request_id = parts.next().unwrap();
        let url = format!(
            "https://gitlab.com/archipels-managed/connect-monorepo/-/merge_requests/{}/#note_{}",
            merge_request_id, change_request_id
        );
        web_sys::window().unwrap().open_with_url(&url).unwrap();
    }) as Box<dyn Fn(JsValue)>);

    create_effect(move |_| {
        info!("not ready :(");
        if let Some(change_requests) = filtered_change_requests.get() {
            let chart = SunburstChart::new(change_requests);
            chart.render("chart", &on_click);
        }
    });

    view! {
        <Layout nav=|| {
            view! {
                <div>
                    <label class="mr-2 text-sm text-slate-600">Filter by author</label>
                    <select
                        class="py-2 pr-8 pl-3 text-sm bg-white rounded border shadow-sm transition duration-300 appearance-none cursor-pointer focus:shadow-md focus:outline-none text-red placeholder:text-slate-400 text-slate-700 border-slate-200 ease hover:border-slate-400 focus:border-slate-400"
                        on:change=move |ev| {
                            let new_value = event_target_value(&ev);
                            set_author(new_value);
                        }
                        prop:value=move || author.get().to_string()
                    >
                        <option value="all">"Everyone"</option>
                        <option value="Filip_Razek">"Filip"</option>
                        <option value="Loanenem">"Loane"</option>
                        <option value="ahanot">"Alexandre"</option>
                        <option value="alexis-besson">"Alexis"</option>
                        <option value="cpagnoux">"Chris"</option>
                        <option value="mathieu-qe">"Mathieu"</option>
                        <option value="n2jn-archipels">"Nico D"</option>
                        <option value="nlapointe-archipels">"Nico L"</option>
                        <option value="yohann-poli">"Yohann"</option>
                    </select>
                </div>
            }
        }>
            <div class="flex-grow p-1" id="chart"></div>
        </Layout>
    }
}
