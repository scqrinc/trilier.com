use chrono::{DateTime, Local};
use serde_json::json;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Document, Element, Event, HtmlFormElement, HtmlInputElement, Node, Request, RequestInit,
    Response, Window,
};

mod env;
mod utils;

const QUERY_SELECTOR_ERR: &str = "Selector is not resolved.";

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn dom_content_loaded() -> Result<(), JsValue> {
    let document: Document = get_document().unwrap();

    let src_select: Element = document
        .query_selector(".src select")
        .expect(QUERY_SELECTOR_ERR)
        .unwrap();

    let src_auto_detect_opt = document.create_element("option")?;
    src_auto_detect_opt.set_attribute("value", "").unwrap();
    src_auto_detect_opt.set_text_content(Some("(Auto detect)"));
    src_select.append_child(&src_auto_detect_opt)?;

    let dst_select: Element = document
        .query_selector(".dst select")
        .expect(QUERY_SELECTOR_ERR)
        .unwrap();
    for lang in utils::LANGS.iter() {
        let src_opt = document.create_element("option")?;
        src_opt.set_attribute("value", lang.code).unwrap();
        // web_sys::console::log_1(&JsValue::from_str(lang.code));
        src_opt.set_text_content(Some(lang.label));
        let dst_opt = src_opt.clone_node_with_deep(true)?;
        src_select.append_child(&src_opt)?;
        dst_select.append_child(&dst_opt)?;
    }

    set_copywrite_year();

    Ok(())
}

// #[wasm_bindgen]
// pub fn form_submit(e: Event) -> Result<(), JsValue> {
//     // std::panic::set_hook(Box::new(console_error_panic_hook::hook));

//     e.prevent_default();

//     let success_callback = Closure::wrap(Box::new(move |resp: Response| {
//         web_sys::console::log_1(&resp);
//     }) as Box<dyn FnMut(Response)>);
//     let error_callback = Closure::wrap(Box::new(move |resp: Response| {
//         web_sys::console::log_1(&resp);
//     }) as Box<dyn FnMut(Response)>);
//     // let form_data = FormData::new_with_form(e.target);
//     let form_data = FormData::new().unwrap();
//     form_data.append_with_str("content", "test").unwrap();
//     ajax("POST", "/api/search", &form_data, success_callback.as_ref().unchecked_ref(), error_callback.as_ref().unchecked_ref());
//     // escape from error: Uncaught Error: closure invoked recursively or destroyed already
//     success_callback.forget();
//     error_callback.forget();
//
//     Ok(())
// }
// // struct HttpRequestParam<'a> {
// //     key: &'a str,
// //     value: &'a str,
// // }
// fn ajax(method: &str, url: &str, form_data: &FormData,
//     success_callback: &js_sys::Function, error_callback: &js_sys::Function) {
//     let xhr = XmlHttpRequest::new().unwrap();
//     // xhr.open(method, url);
//     xhr.open_with_async(method, url, true).unwrap();
//     // todo: convert "multipart/form-data" to:
//     // xhr.set_request_header("Content-Type", "application/x-www-form-urlencoded");
//     xhr.set_onloadend(Some(success_callback));
//     xhr.set_onerror(Some(error_callback));
//     web_sys::console::log_1(form_data);
//     // xhr.send_with_opt_str(Some("...test...")).unwrap();
//     xhr.send_with_opt_form_data(Some(form_data)).unwrap();
// }

#[wasm_bindgen]
pub async fn form_submit(e: Event) -> Result<(), JsValue> {
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    e.prevent_default();

    let mut opts = RequestInit::new();
    // GET/HEAD cannot have req body
    // opts.method("GET");
    opts.method("POST");
    // opts.mode(RequestMode::SameOrigin);

    // let form_data = FormData::new().unwrap();
    // // let form_data = FormData::new_with_form(e.target());
    // form_data.append_with_str("content", "test").unwrap();
    // // // form_data.append_with_str("response", e.target().unwrap().name()).unwrap();
    // // // opts.body(Some(&JsValue::from_str("test")));
    // // let document: Document = get_document().unwrap();
    // // let form: HtmlFormElement = e.target().unwrap().dyn_ref::<HtmlFormElement>().unwrap().clone();
    // // let form_data = FormData::new_with_form(&form).unwrap();
    // opts.body(Some(&form_data));

    let form: HtmlFormElement = e.target().unwrap().unchecked_into::<HtmlFormElement>();
    let req_js_str = get_req_body(form);
    opts.body(Some(&req_js_str));

    let url = "/api/search";

    let request = Request::new_with_str_and_init(&url, &opts)?;
    // request.headers().set("Content-Type", "application/x-www-form-urlencoded").unwrap();
    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` is a `Response` object.
    // assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let resp_js_json: JsValue = JsFuture::from(resp.json()?).await?;
    let resp_json: env::SearchResults = resp_js_json.into_serde::<env::SearchResults>().unwrap();

    let document: Document = get_document().unwrap();
    let results = document.create_element("div").unwrap();
    for x in resp_json.results {
        let src_lang: &str = &x.src_lang;
        let dst_lang: &str = &x.dst_lang;
        let translated_text: &str = &x.translated_text;
        let result = get_result_row(src_lang, dst_lang, translated_text);
        results.append_child(&result)?;
    }
    let result_container: Element = document
        .query_selector("#app main .ret")
        .expect(QUERY_SELECTOR_ERR)
        .unwrap();
    for idx in 0..result_container.class_list().length() {
        results.set_class_name(&result_container.class_list().get(idx).unwrap());
    }
    result_container
        .parent_node()
        .unwrap()
        .replace_child(&results, &result_container)
        .unwrap();

    Ok(())
}

fn get_req_body(form: HtmlFormElement) -> js_sys::JsString {
    let original_text = form
        .query_selector("input[name=\"q\"]")
        .expect(QUERY_SELECTOR_ERR)
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value();
    let src_lang = form
        .query_selector("select[name=\"src-lang\"] option:checked")
        .expect(QUERY_SELECTOR_ERR)
        .unwrap()
        .get_attribute("value")
        .unwrap();
    let dst_lang_elms = form
        .query_selector_all("select[name=\"dst-langs[]\"] option:checked")
        .expect(QUERY_SELECTOR_ERR);
    let mut dst_langs = Vec::<String>::new();
    for i in 0..dst_lang_elms.length() {
        let dst_lang_node: Node = dst_lang_elms.item(i).unwrap();
        // let dst_lang_elm: Element = dst_lang_node.dyn_into().unwrap();
        let dst_lang_elm = dst_lang_node.unchecked_into::<Element>();
        dst_langs.push(dst_lang_elm.get_attribute("value").unwrap());
    }
    let req_json = json!(env::SearchParams {
        original_text: original_text.to_string(),
        src_lang: src_lang.to_string(),
        dst_langs: dst_langs,
    });
    let req_js_json = JsValue::from_serde(&req_json).unwrap();
    js_sys::JSON::stringify(&req_js_json).unwrap()
}

fn get_result_row(src_lang: &str, dst_lang: &str, translated_text: &str) -> Element {
    let document: Document = get_document().unwrap();
    let result_tmpl = document
        .get_element_by_id("ret-tmpl")
        .expect(QUERY_SELECTOR_ERR);

    let result: Element = document.create_element("div").unwrap();
    result.set_class_name("row");
    result.set_inner_html(&result_tmpl.inner_html());
    result
        .query_selector(".src-lang")
        .expect(QUERY_SELECTOR_ERR)
        .unwrap()
        .set_text_content(Some(src_lang));
    result
        .query_selector(".dst-lang")
        .expect(QUERY_SELECTOR_ERR)
        .unwrap()
        .set_text_content(Some(dst_lang));
    result
        .query_selector(".translated-text")
        .expect(QUERY_SELECTOR_ERR)
        .unwrap()
        .set_text_content(Some(translated_text));
    let detail_link = result
        .query_selector(".detail-link")
        .expect(QUERY_SELECTOR_ERR)
        .unwrap();
    let detail_link_href = format!(
        "{}q={}&lr=lang_{}",
        detail_link.get_attribute("href").unwrap(),
        translated_text,
        dst_lang
    );
    detail_link
        .set_attribute("href", &detail_link_href)
        .unwrap();

    result
}

fn get_document() -> Option<Document> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    // let body: HtmlElement = document.body().expect("document should have a body");
    Some(document)
}

fn set_copywrite_year() {
    let dt: DateTime<Local> = Local::now();
    let yyyy: String = dt.format("%Y").to_string();
    let document: Document = get_document().unwrap();
    let elm: Element = document
        .query_selector("#app footer .copyright-year")
        .expect(QUERY_SELECTOR_ERR)
        .unwrap();
    elm.set_text_content(Some(yyyy.clone().as_str()));
}
