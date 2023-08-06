use commons::network::NeuralNetwork;
use wasm_bindgen::JsValue;
use web_sys::window;

#[allow(dead_code)]
pub fn save_best_brain(brain: &NeuralNetwork) -> Result<(), JsValue> {
    let window = window().expect("");

    let storage = window.local_storage()?.unwrap();

    let json = serde_json::to_string(&brain)
        .map_err(|err| JsValue::from_str(std::format!("{:?}", err).as_str()))?;

    storage.set_item("bestBrain", &json)?;

    Ok(())
}

#[allow(dead_code)]
pub fn discard_best_brain() -> Result<(), JsValue> {
    let window = window().expect("");

    let storage = window.local_storage()?.unwrap();

    storage.remove_item("bestBrain")
}
