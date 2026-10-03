use gtk::prelude::*;
use crate::widgets::clock::Clock;

// Enum готовых GTK-объектов
#[derive(Clone)]
pub enum BuiltWidget {
    Clock(Clock),
    // Battery(Battery),
}

impl BuiltWidget {
    pub fn gtk_widget(&self) -> &gtk::Widget {
        match self {
            BuiltWidget::Clock(clock) => clock.trigger.upcast_ref(),
        }
    }
}
