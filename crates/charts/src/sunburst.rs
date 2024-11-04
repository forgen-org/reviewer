use std::collections::HashMap;

use charming::{
    element::{Emphasis, EmphasisFocus, ItemStyle, Label, LabelAlign, LabelPosition, Sort},
    series::{Sunburst, SunburstLevel, SunburstNode},
    Chart, WasmRenderer,
};
use client::ChangeRequest;
use log::*;
use wasm_bindgen::prelude::*;

// use crate::ChartSize;

pub struct SunburstChart {
    chart: Chart,
    // on_click: Option<&'static Closure<dyn Fn(JsValue)>>,
    // size: ChartSize,
}

impl SunburstChart {
    pub fn new(change_requests: Vec<ChangeRequest>) -> Self {
        let mut category_map: HashMap<String, HashMap<String, Vec<&ChangeRequest>>> =
            HashMap::new();

        // Group change requests by category and subcategory
        for change in &change_requests {
            let category = change
                .category
                .clone()
                .unwrap_or_else(|| "Uncategorized".to_string());
            let sub_category = change
                .sub_category
                .clone()
                .unwrap_or_else(|| "Uncategorized".to_string());

            category_map
                .entry(category)
                .or_default()
                .entry(sub_category)
                .or_default()
                .push(change);
        }

        let mut data = Vec::new();
        for (category, subcategories) in category_map {
            let mut category_children = Vec::new();

            for (sub_category, changes) in subcategories {
                let sub_category_children: Vec<SunburstNode> = changes
                    .iter()
                    .map(|change| {
                        SunburstNode::new(format!("{}/{}", change.merge_request_id, change.id))
                            .value(1.0)
                    })
                    .collect();

                category_children
                    .push(SunburstNode::new(sub_category).children(sub_category_children));
            }

            data.push(SunburstNode::new(category).children(category_children));
        }

        SunburstChart {
            chart: Chart::new().series(
                Sunburst::new()
                    .radius(("0%", "95%"))
                    .emphasis(Emphasis::new().focus(EmphasisFocus::Ancestor))
                    .sort(Sort::None)
                    .levels(vec![
                        SunburstLevel::new(),
                        SunburstLevel::new()
                            .r0("15%")
                            .r("35%")
                            .item_style(ItemStyle::new().border_width(2))
                            .label(Label::new().rotate("tangential")),
                        SunburstLevel::new()
                            .r0("35%")
                            .r("70%")
                            .label(Label::new().align(LabelAlign::Right)),
                        SunburstLevel::new()
                            .r0("70%")
                            .r("72%")
                            .item_style(ItemStyle::new().border_width(3))
                            .label(
                                Label::new()
                                    .position(LabelPosition::Outside)
                                    .padding((3, 3, 3, 3))
                                    .silent(false),
                            ),
                    ])
                    .data(data),
            ),
            // on_click: None,
            // size: ChartSize::fullscreen().unwrap(),
        }
    }

    // pub fn on_click<F: Fn(String) + 'static>(mut self, on_click: F) -> Self {
    //     let closure = Closure::wrap(Box::new(move |params| {
    //         let name = js_sys::Reflect::get(&params, &"name".into())
    //             .expect("Object should have 'name' property")
    //             .dyn_into::<js_sys::JsString>()
    //             .expect("'on' should be a string");
    //         let name = name.as_string().unwrap();
    //         on_click(name);
    //     }) as Box<dyn Fn(JsValue)>);
    //     self.on_click = Some(closure);
    //     self
    // }

    pub fn render(&self, id: &str, on_click: &Closure<dyn Fn(JsValue)>) {
        info!("id: {}", id);
        let elem = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id)
            .unwrap();
        let height = elem.client_height();
        let width = elem.client_width();
        let renderer = WasmRenderer::new(width as u32, height as u32);
        let chart = renderer.render(id, &self.chart).unwrap();

        info!("height: {}, width: {}", height, width);

        let js_function: &js_sys::Function = on_click.as_ref().unchecked_ref();
        let js_value: JsValue = chart.clone();
        let on = js_sys::Reflect::get(&js_value, &"on".into())
            .expect("Object should have 'on' method")
            .dyn_into::<js_sys::Function>()
            .expect("'on' should be a function");
        on.call2(&js_value, &"click".into(), js_function)
            .expect("Failed to call 'on' method");

        {
            let resize_closure = Closure::wrap(Box::new({
                move |entries: Vec<JsValue>| {
                    let entry = entries.first().unwrap();
                    let entry = entry
                        .clone()
                        .dyn_into::<web_sys::ResizeObserverEntry>()
                        .unwrap();
                    let new_height = entry.content_rect().height() as u32;
                    let new_width = entry.content_rect().width() as u32;

                    info!("new height: {}, width: {}", new_height, new_width);

                    let _ = &chart.resize(
                        Size {
                            height: new_height,
                            width: new_width,
                        }
                        .into(),
                    );
                }
            }) as Box<dyn FnMut(Vec<JsValue>)>);

            let resize =
                web_sys::ResizeObserver::new(resize_closure.as_ref().unchecked_ref()).unwrap();
            resize.observe(&elem);
            resize_closure.forget();
        }
    }
}

#[wasm_bindgen]
pub struct Size {
    pub height: u32,
    pub width: u32,
}
