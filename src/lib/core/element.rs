use gtk::prelude::*;
use crate::widgets::{
    audio::Audio,
    clock::Clock,
    layout::Layout, network::Network,
};

// Enum готовых GTK-объектов
#[derive(Clone)]
pub enum BuiltWidget {
    Clock(Clock),
    Audio(Audio),
    Layout(Layout),
    Network(Network)
    // Battery(Battery),
}

impl BuiltWidget {
    // Возвращаем gtk::Widget по значению
    pub fn gtk_widget(&self) -> gtk::Widget {
        match self {
            BuiltWidget::Clock(w) => w.trigger.clone().upcast(),
            BuiltWidget::Audio(w) => w.trigger.clone().upcast(),
            BuiltWidget::Layout(w) => w.trigger.clone().upcast(),
            BuiltWidget::Network(w) => w.trigger.clone().upcast(),
            _ => gtk::Button::builder().label("err").build().upcast(),
        }
    }
}
