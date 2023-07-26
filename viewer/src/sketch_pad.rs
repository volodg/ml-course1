use std::cell::RefCell;
use std::rc::Rc;
use commons::utils::OkExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, Element};

pub struct SketchPad {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    on_update: Rc<RefCell<dyn FnMut()>>
}

impl SketchPad {
    pub fn create(container_id: &str) -> Result<Self, JsValue> {
        let document = window().expect("").document().expect("");
        let container = document.get_element_by_id(container_id).unwrap();

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;
        let size = 400;
        canvas.set_width(size);
        canvas.set_height(size);
        canvas.style().set_property("background-color", "white")?;
        canvas.style().set_property("box-shadow", "0px 0px 10px 2px black")?;

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

        /*

        this.onUpdate=onUpdate;
        this.reset();

        this.#addEventListeners();*/

        Self { canvas, context }.ok()
    }

    /*
       reset(){
      this.paths=[];
      this.isDrawing=false;
      this.#redraw();
   }

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

   #getMouse=(evt)=>{
      const rect=this.canvas.getBoundingClientRect();
      return [
         Math.round(evt.clientX-rect.left),
         Math.round(evt.clientY-rect.top)
      ];
   }
     */

}
