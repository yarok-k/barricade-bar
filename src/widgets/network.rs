use anyhow::Result;
use gtk::prelude::*;
use std::sync::{Mutex, Once};
use std::thread;
use std::time::Duration;

use crate::lib::core::widget::WidgetTrait;
use crate::lib::network::{ConnectionStates, ConnectionTypes, Device, Network as NetworkLib};

const POLL_INTERVAL: Duration = Duration::from_secs(2);
const OFFLINE_ICON: &str = "network-offline-symbolic";

// Последняя известная иконка, пустая строка = данных ещё нет
static LATEST_ICON: Mutex<String> = Mutex::new(String::new());
static POLLER: Once = Once::new();

#[derive(Clone)]
pub struct Network {
    pub trigger: gtk::Button,
    pub indicator: gtk::Image,
    shown_icon: String,
}

impl Network {
    /// Запускает единственный фоновый поток опроса NetworkManager.
    fn start_poller() {
        POLLER.call_once(|| {
            thread::Builder::new()
                .name("network-poller".into())
                .spawn(|| loop {
                    match NetworkLib::get_devices() {
                        Ok(devices) => {
                            let icon = Self::pick_icon(&devices);
                            if let Ok(mut guard) = LATEST_ICON.lock() {
                                *guard = icon;
                            }
                        }
                        Err(e) => eprintln!("network poll failed: {e:#}"),
                    }
                    thread::sleep(POLL_INTERVAL);
                })
                .expect("failed to spawn network poller thread");
        });
    }

    fn cached_icon() -> String {
        match LATEST_ICON.lock() {
            Ok(g) if !g.is_empty() => g.clone(),
            _ => OFFLINE_ICON.to_string(),
        }
    }

    /// Выбираем, какое устройство показывать в панели.
    fn pick_icon(devices: &[Device]) -> String {
        let active = devices
            .iter()
            .find(|d| {
                matches!(d.status, ConnectionStates::Connected)
                    && matches!(d.ctype, ConnectionTypes::Wired)
            })
            .or_else(|| {
                devices
                    .iter()
                    .find(|d| matches!(d.status, ConnectionStates::Connected))
            });
        if let Some(d) = active {
            return d.get_icon();
        }

        if let Some(d) = devices
            .iter()
            .find(|d| matches!(d.status, ConnectionStates::Connecting))
        {
            return d.get_icon();
        }

        OFFLINE_ICON.to_string()
    }
}

impl WidgetTrait for Network {
    fn new(_popup: Option<String>) -> Result<Self> {
        Self::start_poller();

        let internal_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let indicator = gtk::Image::builder().icon_name(OFFLINE_ICON).build();
        internal_container.pack_start(&indicator, false, false, 0);

        let trigger = gtk::Button::new();
        trigger.add(&internal_container);

        Ok(Self {
            trigger,
            indicator,
            shown_icon: OFFLINE_ICON.to_string(),
        })
    }

    fn update(&mut self) -> Result<()> {
        // Только чтение кэша: никаких обращений к D-Bus в главном потоке
        let icon = Self::cached_icon();
        if icon != self.shown_icon {
            self.indicator
                .set_from_icon_name(Some(&icon), gtk::IconSize::Button);
            self.shown_icon = icon;
        }
        Ok(())
    }
}
