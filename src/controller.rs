use crate::Model;
use crate::View;

pub struct Controller {
    model: Model,
    view: View,
}

impl Controller {
    pub fn new(model: Model, view: View) -> Self {
        Self { model, view }
    }

    pub fn tick(&mut self) {
        self.view.draw_model(&mut self.model);
    }
}