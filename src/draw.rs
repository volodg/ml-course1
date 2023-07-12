use crate::app_state::{DrawingState, ReadyState, SavedState};
use crate::html::{HtmlDom, Visibility};
use itertools::Itertools;

pub trait Draw {
    fn draw(&self);
}

impl Draw for DrawingState<HtmlDom> {
    fn draw(&self) {
        self.view.canvas.set_visible(true);
        self.view.undo_btn.set_visible(true);
        self.view.student_input.set_display(false);
        self.view.advance_btn.set_inner_html("NEXT");

        self.view.context.clear_rect(
            0.0,
            0.0,
            self.view.canvas.width().into(),
            self.view.canvas.height().into(),
        );

        let mut empty = true;

        for path in self.curr_path() {
            if path.is_empty() {
                continue;
            }
            empty = false;

            for (from, to) in path.iter().tuple_windows() {
                self.view.context.begin_path();
                self.view.context.set_line_width(3.0);
                self.view.context.set_line_cap("round");
                self.view.context.set_line_join("round");

                self.view.context.move_to(from.x as f64, from.y as f64);
                self.view.context.line_to(to.x as f64, to.y as f64);

                self.view.context.stroke();
            }
        }

        self.view.undo_btn.set_disabled(empty);

        let label = self.get_current_label();
        self.view
            .instructions_spn
            .set_inner_html(std::format!("Please draw a {label}").as_str());
    }
}

impl Draw for ReadyState<HtmlDom> {
    fn draw(&self) {
        self.view.canvas.set_visible(false);
        self.view.undo_btn.set_visible(false);

        self.view.instructions_spn.set_inner_html("Thank you!");
        self.view.advance_btn.set_inner_html("SAVE");
    }
}

impl Draw for SavedState<HtmlDom> {
    fn draw(&self) {
        self.view.advance_btn.set_display(false);
        self.view.instructions_spn.set_inner_html(
            "Take you downloaded file and place it along side the others in the dataset!",
        );
    }
}
