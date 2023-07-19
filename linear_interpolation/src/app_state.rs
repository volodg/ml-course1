use crate::html::HtmlDom;
use wasm_bindgen::JsValue;
use web_sys::{AudioContext, OscillatorNode};

pub struct AppState {
    pub html: HtmlDom,
    pub audio_context: Option<AudioContext>,
    pub oscillator: Option<OscillatorNode>,
}

impl AppState {
    pub fn create(html: HtmlDom) -> Self {
        Self {
            html,
            audio_context: None,
            oscillator: None,
        }
    }

    pub fn init_audio(&mut self) -> Result<(), JsValue> {
        match self.oscillator.as_ref() {
            Some(oscillator) => {
                oscillator.stop()?;

                self.audio_context = None;
                self.oscillator = None
            }
            None => {
                let audio_context = AudioContext::new()?;
                let oscillator = audio_context.create_oscillator()?;
                oscillator.frequency().set_value(200.0);
                oscillator.start()?;

                let node = audio_context.create_gain()?;
                node.gain().set_value(0.1);
                oscillator.connect_with_audio_node(&node)?;
                node.connect_with_audio_node(&audio_context.destination())?;

                self.audio_context = Some(audio_context);
                self.oscillator = Some(oscillator)
            }
        }
        Ok(())
    }
}
