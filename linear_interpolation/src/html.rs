use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{
    window, AudioContext, CanvasRenderingContext2d, Document, HtmlCanvasElement, OscillatorNode,
};

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub context: CanvasRenderingContext2d,
    pub canvas: HtmlCanvasElement,
    pub audio_context: Option<AudioContext>,
    pub oscillator: Option<OscillatorNode>,
}

impl HtmlDom {
    pub fn create() -> Result<Self, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("myCanvas").unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let width: u32 = window.inner_width().expect("").as_f64().unwrap() as u32;
        canvas.set_width(width);

        let height: u32 = window.inner_height().expect("").as_f64().unwrap() as u32;
        canvas.set_height(height);

        Self {
            document,
            context,
            canvas,
            audio_context: None,
            oscillator: None,
        }
        .ok()
    }

    pub fn init_audio(&mut self) -> Result<(), JsValue> {
        if self.audio_context.is_none() {
            let audio_context = AudioContext::new()?;
            let osc = audio_context.create_oscillator()?;
            osc.frequency().set_value(200.0);
            osc.start()?;

            let node = audio_context.create_gain()?;
            node.gain().set_value(0.1);
            osc.connect_with_audio_node(&node)?;
            node.connect_with_audio_node(&audio_context.destination())?;

            self.audio_context = Some(audio_context);
            self.oscillator = Some(osc)
        }
        Ok(())
    }
}
