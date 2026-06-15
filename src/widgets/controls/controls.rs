use crate::widgets::controls::audio::AudioWidget;
use crate::widgets::controls::battery::BatteryWidget;
use crate::widgets::controls::network::NetworkWidget;
use crate::widgets::controls::power::PowerWidget;

//use crate::widgets::controls::tray::TrayWidget;
use gtk::prelude::*;

#[derive(Clone)]
pub enum ControlsIndicators {
    Audio(AudioWidget),
    Network(NetworkWidget),
    Battery(BatteryWidget),
    Power(PowerWidget),
    //Tray(TrayWidget),
}

pub struct Controls {
    pub container: gtk::Box,
    pub indicators: Vec<ControlsIndicators>,
}

impl Controls {
    pub fn new() -> Result<Self, anyhow::Error> {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let audio = AudioWidget::new();
        let network = NetworkWidget::new();
        let battery = BatteryWidget::new();
        let power = PowerWidget::new();
        //let tray = TrayWidget::new();

        container.add(&audio.trigger);
        container.add(&network.trigger);
        container.add(&battery.trigger);
        container.add(&power.trigger);
        // container.add(&tray.trigger);

        Ok(Self {
            container: container,
            indicators: vec![
                ControlsIndicators::Audio(audio),
                ControlsIndicators::Network(network),
                ControlsIndicators::Battery(battery),
                ControlsIndicators::Power(power),
            ],
        })
    }
    pub fn update(&mut self) -> Result<(), anyhow::Error> {
        for indicator in self.indicators.clone() {
            match indicator {
                ControlsIndicators::Audio(audio) => audio.clone().update(),
                ControlsIndicators::Network(network) => network.clone().update()?,
                ControlsIndicators::Battery(battery) => battery.clone().update()?,
                //ControlsIndicators::Tray(tray) => tray.clone().update(),
                _ => {}
            }
        }
        Ok(())
    }
}
