use anyhow::Result;
use gtk::prelude::*;


pub trait WidgetTrait: Sized {
    fn new(popup: Option<String>) -> Result<Self>;
    fn update(&mut self) -> Result<()>;
    fn make_trigger() -> (gtk::Box, gtk::Button) {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let trigger = gtk::Button::new();
        trigger.style_context().add_class("barricade_statusbar_button");
        trigger.add(&container);
        (container, trigger)
    }
    fn attach_click_command(trigger: &gtk::Button, command: Option<String>) {
        let Some(cmd) = command else { return };
        trigger.connect_clicked(move |_| {
            let mut parts = cmd.split_whitespace();
            let Some(program) = parts.next() else { return };

            if let Err(e) = std::process::Command::new(program).args(parts).spawn() {
                eprintln!("Не удалось запустить {}: {}", cmd, e);
            }
        });
    }
}
