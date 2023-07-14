use plotters::prelude::{BLACK, BLUE, ChartBuilder, Color, DiscreteRanged, IntoDrawingArea, IntoLinspace, LineSeries, SurfaceSeries, WHITE};
use plotters_canvas::CanvasBackend;
use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, Document, Element, HtmlCanvasElement};

pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)]
pub struct HtmlDom {
    pub document: Document,
    pub container: Element,
    canvas: HtmlCanvasElement,
}

impl HtmlDom {
    #[allow(dead_code)]
    pub fn create() -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();
        let container = document.get_element_by_id("container").unwrap();

        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        Self {
            document,
            container,
            canvas
        }
        .ok()
    }

    pub fn draw_chart(&self, pitch: f64, yaw: f64) -> DrawResult<()> {
        let area = CanvasBackend::with_canvas_object(self.canvas.clone())
            .unwrap()
            .into_drawing_area();
        area.fill(&WHITE)?;

        let x_axis = (-3.0..3.0).step(0.1);
        let z_axis = (-3.0..3.0).step(0.1);

        let mut chart =
            ChartBuilder::on(&area).build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone())?;

        chart.with_projection(|mut pb| {
            pb.yaw = yaw;
            pb.pitch = pitch;
            pb.scale = 0.7;
            pb.into_matrix()
        });

        chart.configure_axes().draw()?;

        chart.draw_series(
            SurfaceSeries::xoz(x_axis.values(), z_axis.values(), |x:f64, z:f64| {
                (x * x + z * z).cos()
            })
                .style(&BLUE.mix(0.2)),
        )?;

        chart.draw_series(LineSeries::new(
            (-100..100)
                .map(|y| y as f64 / 40.0)
                .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
            &BLACK,
        ))?;

        Ok(())
    }
}
