use crate::app_state::{ReadyState, SavedState};
use crate::html::HtmlDom;

pub trait Save {
    type View;

    fn save(&self) -> SavedState<Self::View>;
}

impl Save for ReadyState<HtmlDom> {
    type View = HtmlDom;

    fn save(&self) -> SavedState<Self::View> {
        // TODO save here
        SavedState::create(self)
    }
}
