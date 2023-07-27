use crate::chart_models::{
    get_data_bounds, DataTransformation, DragInto, Options, Sample, SampleStyleType,
};
use crate::graphics::{ContextExt, DrawTextParams};
use crate::html::AddListener;
use commons::math::{lerp, Bounds, Point};
use commons::utils::OkExt;
use js_sys::Array;
use js_sys::Math::sign;
use std::cell::RefCell;
use std::f64::consts::FRAC_PI_2;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{
    window, CanvasRenderingContext2d, Element, HtmlCanvasElement, MouseEvent, WheelEvent,
};

pub struct Chart {
    samples: Vec<Sample>,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    margin: f64,
    transparency: f64,
    data_trans: DataTransformation,
    drag_info: DragInto,
    pixel_bounds: Bounds,
    data_bounds: Bounds,
    default_data_bounds: Bounds,
    options: Options,
    hovered_sample: Option<Sample>,
    selected_sample: Option<Sample>,
    dynamic_point: Option<(Point, String, Sample)>,
    on_click: Option<Rc<RefCell<dyn FnMut(Option<&Sample>)>>>,
}

impl Chart {
    pub fn create(container: Element, mut options: Options) -> Result<Rc<RefCell<Self>>, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;

        canvas.set_width(options.size);
        canvas.set_height(options.size);
        canvas.style().set_property("background-color", "white")?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        container.append_child(&canvas)?;

        let data_trans = DataTransformation {
            offset: Point::default(),
            scale: 1.0,
        };
        let drag_info = DragInto::default();

        let margin = options.size as f64 * 0.11;
        let transparency = options.transparency.unwrap_or(1.0);
        let pixel_bounds = Self::get_pixels_bounds(&canvas, margin);

        if options.icon == SampleStyleType::Image {
            CanvasRenderingContext2d::generate_images(&mut options.styles)?;
        }

        let result = Self {
            samples: vec![],
            canvas,
            context,
            margin,
            transparency,
            data_trans,
            drag_info,
            pixel_bounds,
            data_bounds: Bounds::zero(),
            default_data_bounds: Bounds::zero(),
            options,
            hovered_sample: None,
            selected_sample: None,
            dynamic_point: None,
            on_click: None,
        };

        let result = Rc::new(RefCell::new(result));

        Self::subscribe(&result)?;

        result.ok()
    }

    pub fn set_on_click(&mut self, on_click: Rc<RefCell<dyn FnMut(Option<&Sample>)>>) {
        self.on_click = Some(on_click);
    }

    pub fn set_samples(&mut self, samples: Vec<Sample>) {
        self.data_bounds = get_data_bounds(&samples);
        self.default_data_bounds = self.data_bounds.clone();
        self.samples = samples;
    }

    pub fn show_dynamic_point(
        &mut self,
        point: Option<(Point, String, Sample)>,
    ) -> Result<(), JsValue> {
        self.dynamic_point = point;
        self.draw()
    }

    pub fn samples(&self) -> &[Sample] {
        self.samples.as_ref()
    }

    pub fn select_sample(&mut self, sample: Option<&Sample>) -> Result<(), JsValue> {
        self.selected_sample = sample.map(|x| x.clone());
        self.draw()?;
        Ok(())
    }

    fn subscribe(chart: &Rc<RefCell<Self>>) -> Result<(), JsValue> {
        let chart_copy = chart.clone();
        chart
            .borrow()
            .canvas
            .add_listener("mousedown", move |event: MouseEvent| {
                let mut chart = chart_copy.borrow_mut();
                let data_loc = chart.get_mouse(&event, true);
                chart.drag_info.start = data_loc;
                chart.drag_info.dragging = true;
                chart.drag_info.end = Point::default();
                chart.drag_info.offset = Point::default();
            })?;
        let chart_copy = chart.clone();
        chart
            .borrow()
            .canvas
            .add_listener("mousemove", move |event: MouseEvent| {
                let mut chart = chart_copy.borrow_mut();
                if chart.drag_info.dragging {
                    let data_loc = chart.get_mouse(&event, true);
                    chart.drag_info.end = data_loc;
                    chart.drag_info.offset = (chart.drag_info.start.clone()
                        - chart.drag_info.end.clone())
                    .scale(chart.data_trans.scale * chart.data_trans.scale);
                    let new_offset =
                        chart.data_trans.offset.clone() + chart.drag_info.offset.clone();
                    let new_scale = chart.data_trans.scale;
                    chart.update_data_bounds(new_offset, new_scale);
                }

                let pixel_location = chart.get_mouse(&event, false);
                let pixel_points = chart
                    .samples
                    .iter()
                    .map(|sample| sample.point.remap(&chart.data_bounds, &chart.pixel_bounds))
                    .collect::<Vec<_>>();

                let nearest_sample = pixel_location
                    .get_nearest(&pixel_points)
                    .map(|x| chart.samples[x].clone());
                chart.hovered_sample = if let Some(nearest_sample) = nearest_sample {
                    let distance = nearest_sample
                        .point
                        .remap(&chart.data_bounds, &chart.pixel_bounds)
                        .distance(&pixel_location);
                    if distance < (chart.margin / 2.0) {
                        Some(nearest_sample)
                    } else {
                        None
                    }
                } else {
                    None
                };

                chart.draw().expect("")
            })?;

        let chart_copy = chart.clone();
        chart
            .borrow()
            .canvas
            .add_listener("mouseup", move |_event: MouseEvent| {
                let mut chart = chart_copy.borrow_mut();
                if chart.drag_info.dragging {
                    chart.data_trans.offset =
                        chart.data_trans.offset.clone() + chart.drag_info.offset.clone();
                    chart.drag_info.dragging = false;
                }
            })?;
        let chart_copy = chart.clone();
        chart
            .borrow()
            .canvas
            .add_listener("wheel", move |event: WheelEvent| {
                let mut chart = chart_copy.borrow_mut();
                let dir = sign(event.delta_y());
                let step = 0.02;
                chart.data_trans.scale += dir * step;
                chart.data_trans.scale = step.max(chart.data_trans.scale.min(2.0));
                let new_offset = chart.data_trans.offset.clone();
                let new_scale = chart.data_trans.scale;
                chart.update_data_bounds(new_offset, new_scale);
                chart.draw().expect("");
                event.prevent_default();
            })?;
        let chart_copy = chart.clone();
        chart.borrow().canvas.on_click(move |_event: MouseEvent| {
            if chart_copy.borrow().drag_info.offset != Point::default() {
                return;
            }

            let hovered_sample = chart_copy.borrow().hovered_sample.clone();
            let selected_sample = if let Some(hovered_sample) = hovered_sample {
                if chart_copy.borrow().selected_sample.as_ref() == Some(&hovered_sample) {
                    None
                } else {
                    Some(hovered_sample.clone())
                }
            } else {
                None
            };
            chart_copy.borrow_mut().selected_sample = selected_sample.clone();

            let on_click = chart_copy.borrow().on_click.clone();
            if let Some(on_click) = on_click {
                on_click.borrow_mut()(selected_sample.as_ref())
            }

            chart_copy.borrow().draw().expect("");
        })
    }

    fn update_data_bounds(&mut self, offset: Point, scale: f64) {
        self.data_bounds.left = self.default_data_bounds.left + offset.x;
        self.data_bounds.right = self.default_data_bounds.right + offset.x;
        self.data_bounds.top = self.default_data_bounds.top + offset.y;
        self.data_bounds.bottom = self.default_data_bounds.bottom + offset.y;

        let center = Point {
            x: lerp(self.data_bounds.left, self.data_bounds.right, 0.5),
            y: lerp(self.data_bounds.top, self.data_bounds.bottom, 0.5),
        };

        self.data_bounds.left = lerp(center.x, self.data_bounds.left, scale * scale);
        self.data_bounds.right = lerp(center.x, self.data_bounds.right, scale * scale);
        self.data_bounds.top = lerp(center.y, self.data_bounds.top, scale * scale);
        self.data_bounds.bottom = lerp(center.y, self.data_bounds.bottom, scale * scale);
    }

    fn get_mouse(&self, event: &MouseEvent, is_data_space: bool) -> Point {
        let rect = self.canvas.get_bounding_client_rect();
        let pixel_loc = Point {
            x: event.client_x() as f64 - rect.left(),
            y: event.client_y() as f64 - rect.top(),
        };

        if is_data_space {
            return pixel_loc.remap(&self.pixel_bounds, &self.default_data_bounds);
        }

        pixel_loc
    }

    fn get_pixels_bounds(canvas: &HtmlCanvasElement, margin: f64) -> Bounds {
        Bounds {
            left: margin,
            right: canvas.width() as f64 - margin,
            top: margin,
            bottom: canvas.height() as f64 - margin,
        }
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        self.context.set_global_alpha(self.transparency);
        self.draw_samples(&self.samples)?;
        self.context.set_global_alpha(1.0);

        if let Some(hovered_sample) = self.hovered_sample.as_ref() {
            self.emphasize_samples(hovered_sample, "white")?;
        }

        if let Some(selected_sample) = self.selected_sample.as_ref() {
            self.emphasize_samples(selected_sample, "yellow")?;
        }

        if let Some((dynamic_point, label, sample)) = self.dynamic_point.as_ref() {
            let pixel_location = dynamic_point.remap(&self.data_bounds, &self.pixel_bounds);
            self.context.draw_point_with_color_and_size(
                &pixel_location,
                "rgba(255,255,255,0.7)",
                10000000.0,
            )?;

            self.context.begin_path();
            self.context.move_to(pixel_location.x, pixel_location.y);
            let line_to = sample.point.remap(&self.data_bounds, &self.pixel_bounds);
            self.context.line_to(line_to.x, line_to.y);
            self.context.stroke();

            self.context.draw_image(
                &self
                    .options
                    .styles
                    .get(label)
                    .expect("")
                    .image
                    .clone()
                    .expect(""),
                &pixel_location,
            )?;
        }

        self.draw_axis()?;

        Ok(())
    }

    fn draw_axis(&self) -> Result<(), JsValue> {
        self.context
            .clear_rect(0.0, 0.0, self.canvas.width() as f64, self.margin);
        self.context
            .clear_rect(0.0, 0.0, self.margin, self.canvas.height() as f64);
        self.context.clear_rect(
            self.canvas.width() as f64 - self.margin,
            0.0,
            self.margin,
            self.canvas.height() as f64,
        );
        self.context.clear_rect(
            0.0,
            self.canvas.height() as f64 - self.margin,
            self.canvas.width() as f64,
            self.margin,
        );

        // Draw X Axis text
        {
            self.context.draw_text_with_params(
                self.options.axis_labels[0].as_str(),
                &Point {
                    x: self.canvas.width() as f64 / 2.0,
                    y: self.pixel_bounds.bottom + self.margin / 2.0,
                },
                DrawTextParams {
                    size: (self.margin * 0.6) as u32,
                    ..DrawTextParams::default()
                },
            )?;
        }

        // Draw Y Axis text
        {
            self.context.save();
            self.context.translate(
                self.pixel_bounds.left - self.margin / 2.0,
                self.canvas.height() as f64 / 2.0,
            )?;
            self.context.rotate(-FRAC_PI_2)?;

            self.context.draw_text_with_params(
                self.options.axis_labels[1].as_str(),
                &Point::default(),
                DrawTextParams {
                    size: (self.margin * 0.6) as u32,
                    ..DrawTextParams::default()
                },
            )?;

            self.context.restore();
        }

        // Draw Axis
        {
            self.context.begin_path();
            self.context
                .move_to(self.pixel_bounds.left, self.pixel_bounds.top);
            self.context
                .line_to(self.pixel_bounds.left, self.pixel_bounds.bottom);
            self.context
                .line_to(self.pixel_bounds.right, self.pixel_bounds.bottom);
            let array = Array::of2(&JsValue::from(5), &JsValue::from(4));
            self.context.set_line_dash(&array)?;
            self.context.set_line_width(2.0);
            self.context
                .set_stroke_style(&JsValue::from_str("lightgray"));
            self.context.stroke();
            self.context.set_line_dash(&Array::new())?;
        }

        {
            // Draw x0 scale
            let data_min = Point {
                x: self.pixel_bounds.left,
                y: self.pixel_bounds.bottom,
            }
            .remap(&self.pixel_bounds, &self.data_bounds);
            self.context.draw_text_with_params(
                std::format!("{:.2}", data_min.x).as_str(),
                &Point {
                    x: self.pixel_bounds.left,
                    y: self.pixel_bounds.bottom,
                },
                DrawTextParams {
                    size: (self.margin * 0.3) as u32,
                    align: "left".to_owned(),
                    v_align: "top".to_owned(),
                    ..DrawTextParams::default()
                },
            )?;

            // Draw y0 scale
            self.context.save();
            self.context
                .translate(self.pixel_bounds.left, self.pixel_bounds.bottom)?;
            self.context.rotate(-FRAC_PI_2)?;
            self.context.draw_text_with_params(
                std::format!("{:.2}", data_min.y).as_str(),
                &Point::default(),
                DrawTextParams {
                    size: (self.margin * 0.3) as u32,
                    align: "left".to_owned(),
                    v_align: "bottom".to_owned(),
                    ..DrawTextParams::default()
                },
            )?;

            self.context.restore();
        }

        {
            // Draw x[-1] scale
            let data_max = Point {
                x: self.pixel_bounds.right,
                y: self.pixel_bounds.bottom,
            }
            .remap(&self.pixel_bounds, &self.data_bounds);
            self.context.draw_text_with_params(
                std::format!("{:.2}", data_max.x).as_str(),
                &Point {
                    x: self.pixel_bounds.right,
                    y: self.pixel_bounds.bottom,
                },
                DrawTextParams {
                    size: (self.margin * 0.3) as u32,
                    align: "right".to_owned(),
                    v_align: "top".to_owned(),
                    ..DrawTextParams::default()
                },
            )?;

            // Draw y[-1] scale
            self.context.save();
            self.context
                .translate(self.pixel_bounds.left, self.pixel_bounds.top)?;
            self.context.rotate(-FRAC_PI_2)?;
            self.context.draw_text_with_params(
                std::format!("{:.2}", data_max.y).as_str(),
                &Point::default(),
                DrawTextParams {
                    size: (self.margin * 0.3) as u32,
                    align: "right".to_owned(),
                    v_align: "bottom".to_owned(),
                    ..DrawTextParams::default()
                },
            )?;

            self.context.restore();
        }

        Ok(())
    }

    fn emphasize_samples(&self, sample: &Sample, color: &str) -> Result<(), JsValue> {
        let pixel_location = sample.point.remap(&self.data_bounds, &self.pixel_bounds);
        let gradient = self.context.create_radial_gradient(
            pixel_location.x,
            pixel_location.y,
            0.0,
            pixel_location.x,
            pixel_location.y,
            self.margin,
        )?;
        gradient.add_color_stop(0.0, color)?;
        gradient.add_color_stop(1.0, "rgba(255, 255, 255, 0)")?;

        self.context.draw_point_with_gradient_and_size(
            &pixel_location,
            &gradient,
            self.margin * 2.0,
        )?;

        self.draw_samples(&[sample.clone()])?;

        Ok(())
    }

    fn draw_samples(&self, samples: &[Sample]) -> Result<(), JsValue> {
        for sample in samples {
            let pixel_location = sample.point.remap(&self.data_bounds, &self.pixel_bounds);
            let style = self.options.styles.get(&sample.label).expect("");
            match self.options.icon {
                SampleStyleType::Text => self.context.draw_text_with_params(
                    &style.text,
                    &pixel_location,
                    DrawTextParams {
                        size: 20,
                        ..DrawTextParams::default()
                    },
                )?,
                SampleStyleType::Dot => self
                    .context
                    .draw_point_with_color(&pixel_location, &style.color)?,
                SampleStyleType::Image => self
                    .context
                    .draw_image(&style.image.as_ref().expect(""), &pixel_location)?,
            }
        }

        Ok(())
    }
}
