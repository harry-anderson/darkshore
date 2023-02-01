use model::User;
use wasm_bindgen::prelude::*;
use spa::Spa;
use wasm_cookies::cookies;
use web_sys::HtmlDocument;

mod spa;
mod model;
mod util;
mod router;
mod pages;

const SID: &str = "darkshore.sid";

fn document() -> HtmlDocument {
    use wasm_bindgen::JsCast;
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<HtmlDocument>()
        .unwrap()
}

fn cookie_string() -> String {
    document().cookie().unwrap()
}


#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let log_cfg = tracing_wasm::WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::DEBUG)
        .build();
    tracing_wasm::set_as_global_default_with_config(log_cfg);

    let user = User::fetch("Pauan").await.ok();

    let app = Spa::new("Pauan", user);

    let session_id = cookies::get(&cookie_string(), SID);
    tracing::debug!("sid: {:?}", session_id);

    dominator::append_dom(&dominator::body(), Spa::render(app));

    Ok(())
}
