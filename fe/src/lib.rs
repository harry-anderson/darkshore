use model::User;
use wasm_bindgen::prelude::*;
use spa::Spa;

mod spa;
mod model;
mod util;
mod router;
mod pages;



#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let log_cfg = tracing_wasm::WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::DEBUG)
        .build();
    tracing_wasm::set_as_global_default_with_config(log_cfg);

    let user = User::fetch("Pauan").await.ok();

    let app = Spa::new("Pauan", user);

    dominator::append_dom(&dominator::body(), Spa::render(app));

    Ok(())
}
