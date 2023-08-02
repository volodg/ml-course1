use crate::chart_models::{
    get_data_bounds, DataTransformation, DragInto, Options, Sample, SampleStyleType,
};
use crate::graphics::{ContextExt, DrawTextParams};
use crate::subscribers::AddListener;
use crate::subscribers::HtmlElementExt;
use commons::geometry::{get_nearest, remap_2d_point, Point2D, Point2DView, PointN};
use commons::math::lerp::lerp;
use commons::math::{Bounds, PointExt};
use commons::utils::OkExt;
use js_sys::Array;
use js_sys::Math::sign;
use std::cell::RefCell;
use std::f64::consts::FRAC_PI_2;
use std::rc::{Rc, Weak};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{
    window, CanvasRenderingContext2d, Element, HtmlCanvasElement, MouseEvent, WheelEvent,
};

pub struct Chart {
    samples: Vec<Sample>,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    overlay_canvas: HtmlCanvasElement,
    overlay_context: CanvasRenderingContext2d,
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
    dynamic_point: Option<(PointN, String, Vec<Sample>)>,
    on_click: Option<Rc<RefCell<dyn FnMut(Option<&Sample>)>>>,
    weak_self: Option<Weak<RefCell<Chart>>>,
}

impl Chart {
    pub fn create(container: Element, options: Options) -> Result<Rc<RefCell<Self>>, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;

        canvas.set_width(options.size as u32);
        canvas.set_height(options.size as u32);
        canvas.style().set_property("background-color", "white")?;
        canvas.style().set_property("pointer-events", "none")?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        context.set_image_smoothing_enabled(false);

        container.append_child(&canvas)?;

        let overlay_canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;

        overlay_canvas.set_width(options.size as u32);
        overlay_canvas.set_height(options.size as u32);
        overlay_canvas
            .style()
            .set_property("position", "absolute")?;
        overlay_canvas.style().set_property("left", "0px")?;

        let overlay_context = overlay_canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        overlay_context.set_image_smoothing_enabled(false);

        container.append_child(&overlay_canvas)?;

        let data_trans = DataTransformation {
            offset: Point2D::default(),
            scale: 1.0,
        };
        let drag_info = DragInto::default();

        let margin = options.size as f64 * 0.11;
        let transparency = options.transparency.unwrap_or(1.0);
        let pixel_bounds = Self::get_pixels_bounds(&canvas, margin);

        let result = Self {
            samples: vec![],
            canvas,
            context,
            overlay_canvas,
            overlay_context,
            margin,
            transparency,
            data_trans,
            drag_info,
            pixel_bounds,
            data_bounds: Bounds::default(),
            default_data_bounds: Bounds::default(),
            options,
            hovered_sample: None,
            selected_sample: None,
            dynamic_point: None,
            on_click: None,
            weak_self: None,
        };

        let result = Rc::new(RefCell::new(result));

        result.borrow_mut().weak_self = Some(Rc::downgrade(&result));
        Self::subscribe(&result)?;

        result.ok()
    }

    pub fn set_on_click(&mut self, on_click: Rc<RefCell<dyn FnMut(Option<&Sample>)>>) {
        self.on_click = Some(on_click);
    }

    pub fn set_samples(&mut self, samples: Vec<Sample>) {
        self.data_bounds = get_data_bounds(&samples).unwrap_or(Bounds::default());
        self.default_data_bounds = self.data_bounds.clone();
        self.samples = samples;
    }

    pub fn show_dynamic_point(
        &mut self,
        point: Option<(PointN, String, Vec<Sample>)>,
    ) -> Result<(), JsValue> {
        self.dynamic_point = point;
        self.draw_overlay()
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
            .overlay_canvas
            .add_listener("pointerdown", move |event: MouseEvent| {
                let mut chart = chart_copy.borrow_mut();
                let data_loc = chart.get_mouse(&event, true);
                chart.drag_info.start = data_loc;
                chart.drag_info.dragging = true;
                chart.drag_info.end = Point2D::default();
                chart.drag_info.offset = Point2D::default();
                Ok(())
            })?;
        let chart_copy = chart.clone();
        chart
            .borrow()
            .overlay_canvas
            .add_listener("pointermove", move |event: MouseEvent| {
                let mut chart = chart_copy.borrow_mut();
                if chart.drag_info.dragging {
                    let data_loc = chart.get_mouse(&event, true);
                    chart.drag_info.end = data_loc;
                    chart.drag_info.offset = (chart.drag_info.start.clone()
                        - chart.drag_info.end.clone())
                    .multiply(chart.data_trans.scale * chart.data_trans.scale);
                    let new_offset =
                        chart.data_trans.offset.clone() + chart.drag_info.offset.clone();
                    let new_scale = chart.data_trans.scale;
                    chart.update_data_bounds(new_offset, new_scale);
                }

                let pixel_location = chart.get_mouse(&event, false);
                let pixel_points = chart
                    .samples
                    .iter()
                    .map(|sample| {
                        remap_2d_point(&sample.point, &chart.data_bounds, &chart.pixel_bounds)
                    })
                    .map(|point| vec![point.x, point.y])
                    .collect::<Vec<_>>();

                let nearest_sample =
                    get_nearest(&vec![pixel_location.x, pixel_location.y], &pixel_points)
                        .first()
                        .map(|x| chart.samples[*x].clone());
                chart.hovered_sample = if let Some(nearest_sample) = nearest_sample {
                    let distance = remap_2d_point(
                        &nearest_sample.point,
                        &chart.data_bounds,
                        &chart.pixel_bounds,
                    )
                    .distance(&pixel_location);
                    if distance < (chart.margin / 2.0) {
                        Some(nearest_sample)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if chart.drag_info.dragging {
                    chart.draw().expect("");
                    chart.draw_overlay()
                } else {
                    chart.draw_overlay()
                }
            })?;

        let chart_copy = chart.clone();
        chart
            .borrow()
            .overlay_canvas
            .add_listener("pointerup", move |_event: MouseEvent| {
                let mut chart = chart_copy.borrow_mut();
                if chart.drag_info.dragging {
                    chart.data_trans.offset =
                        chart.data_trans.offset.clone() + chart.drag_info.offset.clone();
                    chart.drag_info.dragging = false;
                }
                Ok(())
            })?;
        let chart_copy = chart.clone();
        chart
            .borrow()
            .overlay_canvas
            .add_listener("wheel", move |event: WheelEvent| {
                let mut chart = chart_copy.borrow_mut();
                let dir = sign(event.delta_y());
                let step = 0.02;
                let scale = 1.0 + dir * step;
                let new_scale = chart.data_trans.scale * scale;
                chart.data_trans.scale = new_scale;
                let offset = chart.data_trans.offset.clone();
                chart.update_data_bounds(offset, new_scale);
                chart.draw()?;
                chart.draw_overlay()?;
                event.prevent_default();
                Ok(())
            })?;
        let chart_copy = chart.clone();
        chart
            .borrow()
            .overlay_canvas
            .on_click(move |_event: MouseEvent| {
                if chart_copy.borrow().drag_info.offset != Point2D::default() {
                    return Ok(());
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

                chart_copy.borrow().draw()?;
                chart_copy.borrow().draw_overlay()
            })
    }

    fn update_data_bounds(&mut self, offset: Point2D, scale: f64) {
        self.data_bounds.left = self.default_data_bounds.left + offset.x;
        self.data_bounds.right = self.default_data_bounds.right + offset.x;
        self.data_bounds.top = self.default_data_bounds.top + offset.y;
        self.data_bounds.bottom = self.default_data_bounds.bottom + offset.y;

        let center = Point2D {
            x: lerp(self.data_bounds.left, self.data_bounds.right, 0.5),
            y: lerp(self.data_bounds.top, self.data_bounds.bottom, 0.5),
        };

        self.data_bounds.left = lerp(center.x, self.data_bounds.left, scale * scale);
        self.data_bounds.right = lerp(center.x, self.data_bounds.right, scale * scale);
        self.data_bounds.top = lerp(center.y, self.data_bounds.top, scale * scale);
        self.data_bounds.bottom = lerp(center.y, self.data_bounds.bottom, scale * scale);
    }

    fn get_mouse(&self, event: &MouseEvent, is_data_space: bool) -> Point2D {
        let rect = self.canvas.get_bounding_client_rect();
        let pixel_loc = Point2D {
            x: event.client_x() as f64 - rect.left(),
            y: event.client_y() as f64 - rect.top(),
        };

        if is_data_space {
            return remap_2d_point(
                &vec![pixel_loc.x, pixel_loc.y],
                &self.pixel_bounds,
                &self.default_data_bounds,
            );
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

    fn draw_overlay(&self) -> Result<(), JsValue> {
        self.overlay_context.clear_rect(
            0.0,
            0.0,
            self.overlay_canvas.width().into(),
            self.overlay_canvas.height().into(),
        );

        if let Some(hovered_sample) = self.hovered_sample.as_ref() {
            self.emphasize_samples(hovered_sample, "white")?;
        }

        if let Some(selected_sample) = self.selected_sample.as_ref() {
            self.emphasize_samples(selected_sample, "yellow")?;
        }

        self.show_nearest(&self.overlay_context)?;

        self.draw_axis(&self.overlay_context)?;

        Ok(())
    }

    fn show_nearest(&self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        if let Some((dynamic_point, label, samples)) = self.dynamic_point.as_ref() {
            let pixel_location =
                remap_2d_point(&dynamic_point, &self.data_bounds, &self.pixel_bounds);
            context.draw_point_with_color_and_size(
                &pixel_location,
                "rgba(255,255,255,0.7)",
                10000000.0,
            )?;

            context.set_stroke_style(&JsValue::from_str("black"));

            context.begin_path();
            for sample in samples {
                context.move_to(pixel_location.x, pixel_location.y);
                let line_to = remap_2d_point(&sample.point, &self.data_bounds, &self.pixel_bounds);
                context.line_to(line_to.x, line_to.y);
            }
            context.stroke();

            context.draw_image_at_center(
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

        Ok(())
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        // Draw background
        if let Some(background) = &self.options.background {
            let top_left = remap_2d_point(&vec![0.0, 1.0], &self.data_bounds, &self.pixel_bounds);
            let size = (self.canvas.width() as f64 - self.margin * 2.0)
                / (self.data_trans.scale * self.data_trans.scale);
            self.context
                .draw_image_with_size(background, &top_left, size)?;

            let weak_self = self.weak_self.clone().expect("");
            background.on_load(move || {
                let chart = weak_self.upgrade().expect("");
                chart.borrow().draw().expect("");
            });
        }

        self.context.set_global_alpha(self.transparency);
        self.draw_samples(&self.samples, &self.context)?;
        self.context.set_global_alpha(1.0);

        self.draw_axis(&self.context)?;

        Ok(())
    }

    fn draw_axis(&self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        context.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.margin);
        context.clear_rect(0.0, 0.0, self.margin, self.canvas.height() as f64);
        context.clear_rect(
            self.canvas.width() as f64 - self.margin,
            0.0,
            self.margin,
            self.canvas.height() as f64,
        );
        context.clear_rect(
            0.0,
            self.canvas.height() as f64 - self.margin,
            self.canvas.width() as f64,
            self.margin,
        );

        // Draw X Axis text
        {
            context.draw_text_with_params(
                self.options.axis_labels[0].as_str(),
                &Point2D {
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
            context.save();
            context.translate(
                self.pixel_bounds.left - self.margin / 2.0,
                self.canvas.height() as f64 / 2.0,
            )?;
            context.rotate(-FRAC_PI_2)?;

            context.draw_text_with_params(
                self.options.axis_labels[1].as_str(),
                &Point2D::default(),
                DrawTextParams {
                    size: (self.margin * 0.6) as u32,
                    ..DrawTextParams::default()
                },
            )?;

            context.restore();
        }

        // Draw Axis
        {
            context.begin_path();
            context.move_to(self.pixel_bounds.left, self.pixel_bounds.top);
            context.line_to(self.pixel_bounds.left, self.pixel_bounds.bottom);
            context.line_to(self.pixel_bounds.right, self.pixel_bounds.bottom);
            let array = Array::of2(&JsValue::from(5), &JsValue::from(4));
            context.set_line_dash(&array)?;
            context.set_line_width(2.0);
            context.set_stroke_style(&JsValue::from_str("lightgray"));
            context.stroke();
            context.set_line_dash(&Array::new())?;
        }

        {
            // Draw x0 scale
            let data_min = remap_2d_point(
                &vec![self.pixel_bounds.left, self.pixel_bounds.bottom],
                &self.pixel_bounds,
                &self.data_bounds,
            );
            context.draw_text_with_params(
                std::format!("{:.2}", data_min.x).as_str(),
                &Point2D {
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
            context.save();
            context.translate(self.pixel_bounds.left, self.pixel_bounds.bottom)?;
            context.rotate(-FRAC_PI_2)?;
            context.draw_text_with_params(
                std::format!("{:.2}", data_min.y).as_str(),
                &Point2D::default(),
                DrawTextParams {
                    size: (self.margin * 0.3) as u32,
                    align: "left".to_owned(),
                    v_align: "bottom".to_owned(),
                    ..DrawTextParams::default()
                },
            )?;

            context.restore();
        }

        {
            // Draw x[-1] scale
            let data_max = remap_2d_point(
                &vec![self.pixel_bounds.right, self.pixel_bounds.top],
                &self.pixel_bounds,
                &self.data_bounds,
            );
            context.draw_text_with_params(
                std::format!("{:.2}", data_max.x).as_str(),
                &Point2D {
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
            context.save();
            context.translate(self.pixel_bounds.left, self.pixel_bounds.top)?;
            context.rotate(-FRAC_PI_2)?;
            context.draw_text_with_params(
                std::format!("{:.2}", data_max.y).as_str(),
                &Point2D::default(),
                DrawTextParams {
                    size: (self.margin * 0.3) as u32,
                    align: "right".to_owned(),
                    v_align: "bottom".to_owned(),
                    ..DrawTextParams::default()
                },
            )?;

            context.restore();
        }

        Ok(())
    }

    fn emphasize_samples(&self, sample: &Sample, color: &str) -> Result<(), JsValue> {
        let pixel_location = remap_2d_point(&sample.point, &self.data_bounds, &self.pixel_bounds);
        let gradient = self.overlay_context.create_radial_gradient(
            pixel_location.x,
            pixel_location.y,
            0.0,
            pixel_location.x,
            pixel_location.y,
            self.margin,
        )?;
        gradient.add_color_stop(0.0, color)?;
        gradient.add_color_stop(1.0, "rgba(255, 255, 255, 0)")?;

        self.overlay_context.draw_point_with_gradient_and_size(
            &pixel_location,
            &gradient,
            self.margin * 2.0,
        )?;

        self.draw_samples(&[sample.clone()], &self.overlay_context)?;

        Ok(())
    }

    fn draw_samples(
        &self,
        samples: &[Sample],
        context: &CanvasRenderingContext2d,
    ) -> Result<(), JsValue> {
        for sample in samples {
            let pixel_location =
                remap_2d_point(&sample.point, &self.data_bounds, &self.pixel_bounds);
            let style = self.options.styles.get(&sample.label).expect("");
            match self.options.icon {
                SampleStyleType::Text => context.draw_text_with_params(
                    &style.text,
                    &pixel_location,
                    DrawTextParams {
                        size: 20,
                        ..DrawTextParams::default()
                    },
                )?,
                SampleStyleType::Dot => {
                    context.draw_point_with_color(&pixel_location, &style.color)?
                }
                SampleStyleType::Image => context
                    .draw_image_at_center(&style.image.as_ref().expect(""), &pixel_location)?,
            }
        }

        Ok(())
    }
}
