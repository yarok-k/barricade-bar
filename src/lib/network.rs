use dbus::blocking::Connection;
use networkmanager::devices::{Any, Device, Wired, Wireless};
use networkmanager::types::ActiveConnectionState;
use networkmanager::{Error, NetworkManager};

#[derive(Debug, Clone)]
pub struct WiredConnection {
    pub speed: u32,
}
#[derive(Debug, Clone)]
pub struct WiFiConnection {
    pub power: u8,
    pub speed: u32,
}
#[derive(Debug, Clone)]
pub enum ConnectionTypes {
    WiFi(WiFiConnection),
    Wired(WiredConnection),
    None,
}

#[derive(Debug, Clone)]
pub struct NetworkLib {
    pub connection_type: ConnectionTypes,
    pub speed: u32,
}

impl NetworkLib {
    pub fn new() -> Self {
        let mut net = Self {
            connection_type: ConnectionTypes::None,
            speed: 0,
        };
        net.update().ok();
        net
    }

    pub fn update(&mut self) -> Result<(), anyhow::Error> {
        let dbus_connection = Connection::new_system()?;
        let nm = NetworkManager::new(&dbus_connection);

        // Сбрасываем текущее состояние
        self.connection_type = ConnectionTypes::None;
        let mut found_connections = Vec::<ConnectionTypes>::new();

        for dev in nm.get_devices()? {
            match dev {
                Device::Ethernet(act_dev) => {
                    if let Ok(active_conn) = act_dev.active_connection() {
                        if let Ok(state) = active_conn.state() {
                            if let ActiveConnectionState::Activated = state {
                                let speed = act_dev.speed().unwrap_or(0);
                                found_connections
                                    .push(ConnectionTypes::Wired(WiredConnection { speed }));
                            }
                        }
                    }
                }
                Device::WiFi(act_dev) => {
                    if let Ok(active_conn) = act_dev.active_connection() {
                        if let Ok(state) = active_conn.state() {
                            if let ActiveConnectionState::Activated = state {
                                let speed = act_dev.bitrate().unwrap_or(0) / 1000;
                                let mut power = 0;
                                if let Ok(ap) = act_dev.active_access_point() {
                                    power = ap.strength().unwrap_or(0);
                                }
                                found_connections
                                    .push(ConnectionTypes::WiFi(WiFiConnection { power, speed }));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if found_connections.is_empty() {
            self.connection_type = ConnectionTypes::None;
        } else {
            for conn in found_connections {
                match conn {
                    ConnectionTypes::Wired(this_conn) => {
                        self.speed = this_conn.speed;
                        self.connection_type = ConnectionTypes::Wired(this_conn);
                        break;
                    }
                    ConnectionTypes::WiFi(this_conn) => {
                        self.speed = this_conn.speed;
                        self.connection_type = ConnectionTypes::WiFi(this_conn);
                        break;
                    }
                    ConnectionTypes::None => {}
                }
            }
        }

        Ok(())
    }

    pub fn get_icon_name(&self) -> &str {
        let icon_name = match &self.connection_type {
            ConnectionTypes::Wired(_) => "network-wired-symbolic",
            ConnectionTypes::WiFi(this_conn) => match this_conn.power {
                0..=20 => "network-wireless-signal-none-symbolic",
                21..=45 => "network-wireless-signal-weak-symbolic",
                46..=70 => "network-wireless-signal-ok-symbolic",
                71..=90 => "network-wireless-signal-good-symbolic",
                _ => "network-wireless-signal-excellent-symbolic",
            },
            ConnectionTypes::None => "network-offline-symbolic",
        };
        icon_name
    }
}
