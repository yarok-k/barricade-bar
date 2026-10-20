use anyhow::Result;
use nmrs::{DeviceState, NetworkManager};
use std::sync::OnceLock;
use tokio::runtime::{Builder, Runtime};

pub struct Device {
    pub name: String,
    pub power: Option<u8>, // None для wired
    pub speed: u32,        // Мбит/с, 0 если неизвестно
    pub status: ConnectionStates,
    pub ctype: ConnectionTypes,
}

impl Device {
    pub fn new(
        name: String,
        power: Option<u8>,
        speed: u32,
        status: ConnectionStates,
        ctype: ConnectionTypes,
    ) -> Self {
        Self { name, power, speed, status, ctype }
    }

    pub fn get_icon(&self) -> String {
        let name = match (&self.ctype, &self.status) {
            (ConnectionTypes::WiFi, ConnectionStates::Connected) => match self.power.unwrap_or(0) {
                80..=100 => "network-wireless-signal-excellent-symbolic",
                55..=79 => "network-wireless-signal-good-symbolic",
                30..=54 => "network-wireless-signal-ok-symbolic",
                1..=29 => "network-wireless-signal-weak-symbolic",
                _ => "network-wireless-signal-none-symbolic",
            },
            (ConnectionTypes::WiFi, ConnectionStates::Connecting) => "network-wireless-acquiring-symbolic",
            (ConnectionTypes::WiFi, ConnectionStates::NotConnected) => "network-wireless-offline-symbolic",
            (ConnectionTypes::Wired, ConnectionStates::Connected) => "network-wired-symbolic",
            (ConnectionTypes::Wired, ConnectionStates::Connecting) => "network-wired-acquiring-symbolic",
            (ConnectionTypes::Wired, ConnectionStates::NotConnected) => "network-wired-disconnected-symbolic",
        };
        name.to_string()
    }
}

pub enum ConnectionTypes {
    WiFi,
    Wired,
}

pub enum ConnectionStates {
    Connected,
    NotConnected,
    Connecting,
}

pub struct Network {}

impl Network {
    fn runtime() -> &'static Runtime {
        static RT: OnceLock<Runtime> = OnceLock::new();
        RT.get_or_init(|| {
            Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to build tokio runtime")
        })
    }

    fn map_state(state: &DeviceState) -> ConnectionStates {
        if *state == DeviceState::Activated {
            ConnectionStates::Connected
        } else if state.is_transitional() {
            ConnectionStates::Connecting
        } else {
            ConnectionStates::NotConnected
        }
    }

    fn wired_speed(iface: &str) -> u32 {
        std::fs::read_to_string(format!("/sys/class/net/{iface}/speed"))
            .ok()
            .and_then(|s| s.trim().parse::<i32>().ok())
            .filter(|v| *v > 0)
            .map(|v| v as u32)
            .unwrap_or(0)
    }

    pub fn get_devices() -> Result<Vec<Device>> {
        Self::runtime().block_on(async {
            let nm = NetworkManager::new().await?;
            let raw = nm.list_devices().await?;
            let current_ssid = nm.current_ssid().await;

            let mut out = Vec::new();
            for d in raw {
                let status = Self::map_state(&d.state);

                if d.is_wireless() {
                    // Уровень сигнала ищем среди сетей именно этого интерфейса
                    let power = match (&status, &current_ssid) {
                        (ConnectionStates::Connected, Some(ssid)) => nm
                            .list_networks(Some(&d.interface))
                            .await?
                            .into_iter()
                            .find(|n| &n.ssid == ssid)
                            .and_then(|n| n.strength)
                            .unwrap_or(0),
                        _ => 0,
                    };
                    out.push(Device::new(
                        d.interface,
                        Some(power),
                        0,
                        status,
                        ConnectionTypes::WiFi,
                    ));
                } else if d.is_wired() {
                    let speed = Self::wired_speed(&d.interface);
                    out.push(Device::new(
                        d.interface,
                        None,
                        speed,
                        status,
                        ConnectionTypes::Wired,
                    ));
                }
            }
            Ok(out)
        })
    }
}
