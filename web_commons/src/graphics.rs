use crate::chart_models::{Point, SampleStyle};
use std::collections::HashMap;
use std::f64::consts::TAU;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

pub struct DrawTextParams {
    pub align: String,
    pub v_align: String,
    pub size: u32,
    pub color: String,
}

impl Default for DrawTextParams {
    fn default() -> Self {
        Self {
            align: "center".to_owned(),
            v_align: "middle".to_owned(),
            size: 10,
            color: "black".to_owned(),
        }
    }
}

pub trait ContextExt {
    fn draw_text(&self, text: &str, location: &Point) -> Result<(), JsValue>;
    fn draw_text_with_params(
        &self,
        text: &str,
        location: &Point,
        params: DrawTextParams,
    ) -> Result<(), JsValue>;

    fn draw_point(&self, location: &Point) -> Result<(), JsValue>;
    fn draw_point_with_color(&self, location: &Point, color: &str) -> Result<(), JsValue>;
    fn draw_point_with_color_and_size(
        &self,
        location: &Point,
        color: &str,
        size: f64,
    ) -> Result<(), JsValue>;

    fn generate_images(styles: &mut HashMap<String, SampleStyle>) -> Result<(), JsValue>;
    fn generate_images_with_size(
        styles: &mut HashMap<String, SampleStyle>,
        size: u32,
    ) -> Result<(), JsValue>;

    fn draw_image(&self, image: &HtmlImageElement, location: &Point) -> Result<(), JsValue>;
}

impl ContextExt for CanvasRenderingContext2d {
    fn draw_text(&self, text: &str, location: &Point) -> Result<(), JsValue> {
        self.draw_text_with_params(text, location, DrawTextParams::default())
    }

    fn draw_text_with_params(
        &self,
        text: &str,
        location: &Point,
        params: DrawTextParams,
    ) -> Result<(), JsValue> {
        self.set_text_align(&params.align);
        self.set_text_baseline(&params.v_align);
        self.set_font(std::format!("bold {}px Courier", params.size).as_str());
        self.set_fill_style(&JsValue::from_str(params.color.as_str()));
        self.fill_text(text, location.x, location.y)?;

        Ok(())
    }

    fn draw_point(&self, location: &Point) -> Result<(), JsValue> {
        self.draw_point_with_color(location, "black")
    }

    fn draw_point_with_color(&self, location: &Point, color: &str) -> Result<(), JsValue> {
        self.draw_point_with_color_and_size(location, color, 8.0)
    }

    fn draw_point_with_color_and_size(
        &self,
        location: &Point,
        color: &str,
        size: f64,
    ) -> Result<(), JsValue> {
        self.begin_path();
        self.set_fill_style(&JsValue::from_str(color));
        self.arc(location.x, location.y, size / 2.0, 0.0, TAU)?;
        self.fill();

        Ok(())
    }

    fn generate_images(styles: &mut HashMap<String, SampleStyle>) -> Result<(), JsValue> {
        Self::generate_images_with_size(styles, 20)
    }

    fn generate_images_with_size(
        styles: &mut HashMap<String, SampleStyle>,
        size: u32,
    ) -> Result<(), JsValue> {
        let document = window().unwrap().document().unwrap();

        for style in styles.values_mut() {
            let canvas = document
                .create_element("canvas")?
                .dyn_into::<HtmlCanvasElement>()?;
            canvas.set_width(size);
            canvas.set_height(size);

            let context = canvas
                .get_context("2d")?
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()?;
            context.begin_path();
            context.set_text_align("center");
            context.set_text_baseline("middle");
            context.set_font(std::format!("{}px Courier", size).as_str());
            context.fill_text(&style.text, size as f64 / 2.0, size as f64 / 2.0)?;

            let image = document
                .create_element("image")
                .unwrap()
                .dyn_into::<HtmlImageElement>()
                .unwrap();
            image.set_src(&canvas.to_data_url()?);

            style.image = Some(image);
        }

        Ok(())
    }

    fn draw_image(&self, image: &HtmlImageElement, location: &Point) -> Result<(), JsValue> {
        self.begin_path();
        self.draw_image_with_html_image_element_and_dw_and_dh(
            image,
            location.x - image.width() as f64 / 2.0,
            location.y - image.height() as f64 / 2.0,
            image.width() as f64,
            image.height() as f64,
        )?;
        self.fill();

        Ok(())
    }
}
