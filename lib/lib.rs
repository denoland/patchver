use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn patchver(
  input: Vec<u8>,
  channel: String,
) -> Result<Vec<u8>, JsValue> {
  console_error_panic_hook::set_once();

  let mut out = Vec::new();
  patchver::patchver(input, channel, &mut out).map_err(|e| JsValue::from_str(&e.to_string()))?;
  Ok(out)
}
