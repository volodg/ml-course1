use crate::app_state::{DrawingState, ReadyState, SavedState};
use crate::html::Visibility;
use itertools::Itertools;

pub trait Draw {
    fn draw(&self);
}

impl Draw for DrawingState {
    fn draw(&self) {
        self.html_dom.canvas.set_visible(true);
        self.html_dom.undo_btn.set_visible(true);
        self.html_dom.student_input.set_display(false);
        self.html_dom.advance_btn.set_inner_html("NEXT");

        self.html_dom.context.clear_rect(
            0.0,
            0.0,
            self.html_dom.canvas.width().into(),
            self.html_dom.canvas.height().into(),
        );

        let mut empty = true;

        for path in self.curr_path() {
            if path.is_empty() {
                continue;
            }
            empty = false;

            for (from, to) in path.iter().tuple_windows() {
                self.html_dom.context.begin_path();
                self.html_dom.context.set_line_width(3.0);
                self.html_dom.context.set_line_cap("round");
                self.html_dom.context.set_line_join("round");

                self.html_dom.context.move_to(from.x as f64, from.y as f64);
                self.html_dom.context.line_to(to.x as f64, to.y as f64);

                self.html_dom.context.stroke();
            }
        }

        self.html_dom.undo_btn.set_disabled(empty);

        let label = self.get_current_label();
        self.html_dom
            .instructions_spn
            .set_inner_html(std::format!("Please draw a {label}").as_str());
    }
}

impl Draw for ReadyState {
    fn draw(&self) {
        self.html_dom.canvas.set_visible(false);
        self.html_dom.undo_btn.set_visible(false);

        self.html_dom.instructions_spn.set_inner_html("Thank you!");
        self.html_dom.advance_btn.set_inner_html("SAVE");
    }
}

impl Draw for SavedState {
    fn draw(&self) {
        self.html_dom.advance_btn.set_display(false);
        self.html_dom.instructions_spn.set_inner_html(
            "Take you downloaded file and place it along side the others in the dataset!",
        );
    }
}
