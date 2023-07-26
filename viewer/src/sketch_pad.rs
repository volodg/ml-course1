use commons::math::Point;
use commons::utils::OkExt;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::html::AddListener;
use web_sys::{window, CanvasRenderingContext2d, Element, HtmlCanvasElement, MouseEvent};

pub struct SketchPad {
    #[allow(dead_code)]
    canvas: HtmlCanvasElement,
    #[allow(dead_code)]
    context: CanvasRenderingContext2d,
    #[allow(dead_code)]
    on_update: Option<Rc<RefCell<dyn FnMut()>>>,
    #[allow(dead_code)]
    paths: Vec<Vec<Point>>,
    #[allow(dead_code)]
    is_drawing: bool,
}

impl SketchPad {
    pub fn create(container_id: &str) -> Result<Rc<RefCell<Self>>, JsValue> {
        let document = window().expect("").document().expect("");
        let container = document.get_element_by_id(container_id).unwrap();

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;
        let size = 400;
        canvas.set_width(size);
        canvas.set_height(size);
        canvas.style().set_property("background-color", "white")?;
        canvas
            .style()
            .set_property("box-shadow", "0px 0px 10px 2px black")?;

        container.append_child(&canvas)?;

        let line_break = document.create_element("br")?.dyn_into::<Element>()?;
        container.append_child(&line_break)?;

        let undo_btn = document.create_element("button")?.dyn_into::<Element>()?;
        undo_btn.set_inner_html("UNDO");
        container.append_child(&undo_btn)?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let result = Rc::new(RefCell::new(Self {
            canvas,
            context,
            on_update: None,
            paths: vec![],
            is_drawing: false,
        }));

        Self::add_event_listeners(&result)?;

        result.ok()
    }

    #[allow(dead_code)]
    pub fn set_on_update(&mut self, on_update: Rc<RefCell<dyn FnMut()>>) {
        self.on_update = Some(on_update)
    }

    /*
        #getMouse=(evt)=>{
       const rect=this.canvas.getBoundingClientRect();
       return [
          Math.round(evt.clientX-rect.left),
          Math.round(evt.clientY-rect.top)
       ];
    }
     */
    fn get_mouse(&self, event: MouseEvent) -> Point {
        let rect = self.canvas.get_bounding_client_rect();
        Point {
            x: event.client_x() as f64 - rect.left(),
            y: event.client_y() as f64 - rect.right(),
        }
    }

    fn add_event_listeners(sketch_pad: &Rc<RefCell<Self>>) -> Result<(), JsValue> {
        let sketch_pad_copy = sketch_pad.clone();
        sketch_pad
            .borrow()
            .canvas
            .add_listener("mousedown", move |event: MouseEvent| {
                let mut sketch_pad = sketch_pad_copy.borrow_mut();
                let mouse = sketch_pad.get_mouse(event);
                sketch_pad.paths.push(vec![mouse]);
                sketch_pad.is_drawing = true;
            })?;

        Ok(())
    }

    /*
    #addEventListeners(){
       this.canvas.onmousedown=(evt)=>{
          const mouse=this.#getMouse(evt);
          this.paths.push([mouse]);
          this.isDrawing=true;
       }
       this.canvas.onmousemove=(evt)=>{
          if(this.isDrawing){
             const mouse=this.#getMouse(evt);
             const lastPath=this.paths[this.paths.length-1];
             lastPath.push(mouse);
             this.#redraw();
          }
       }
       document.onmouseup=()=>{
          this.isDrawing=false;
       }
       this.canvas.ontouchstart=(evt)=>{
          const loc=evt.touches[0];
          this.canvas.onmousedown(loc);
       }
       this.canvas.ontouchmove=(evt)=>{
          const loc=evt.touches[0];
          this.canvas.onmousemove(loc);
       }
       document.ontouchend=()=>{
          document.onmouseup();
       }
       this.undoBtn.onclick=()=>{
          this.paths.pop();
          this.#redraw();
       }
    }

    #redraw(){
       this.ctx.clearRect(0,0,
          this.canvas.width,this.canvas.height);
       draw.paths(this.ctx,this.paths);
       if(this.paths.length>0){
          this.undoBtn.disabled=false;
       }else{
          this.undoBtn.disabled=true;
       }
       this.triggerUpdate();
    }

    triggerUpdate(){
       if(this.onUpdate){
          this.onUpdate(this.paths);
       }
    }
      */
}
