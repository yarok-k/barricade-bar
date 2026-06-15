use std::fs;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
    Unknown,
    NotCharging,
    NoBattery, // Добавили явный статус для удобства
}

const CAPACITY_FILE_PATH: &str = "/sys/class/power_supply/BAT0/capacity";
const BATTERY_STATUS_FILE_PATH: &str = "/sys/class/power_supply/BAT0/status";

#[derive(Clone, Copy)]
pub struct BatteryLib {
    pub capacity: i16, // Изменили u8 на i16, чтобы хранить -1
    pub status: BatteryStatus,
}

impl BatteryLib {
    pub fn new() -> Self {
        let mut battery = Self {
            capacity: -1, // По умолчанию считаем, что батареи нет
            status: BatteryStatus::NoBattery,
        };
        // Игнорируем ошибку чтения файлов, так как метод сам выставит -1 внутри update
        let _ = battery.update();
        battery
    }

    pub fn update(&mut self) -> Result<(), anyhow::Error> {
        let power_supply_path = "/sys/class/power_supply";

        // 1. Проверяем, существует ли директория и есть ли в ней файлы
        let mut has_devices = false;
        if Path::new(power_supply_path).exists() {
            if let Ok(mut entries) = fs::read_dir(power_supply_path) {
                if entries.next().is_some() {
                    has_devices = true;
                }
            }
        }

        // Если папка пуста или её нет — выставляем -1 и выходим без ошибок
        if !has_devices {
            self.capacity = -1;
            self.status = BatteryStatus::NoBattery;
            return Ok(());
        }

        // 2. Дополнительно проверяем, существует ли конкретно BAT0
        // (на ПК в power_supply может лежать "AC", а батареи BAT0 не будет)
        if !Path::new(CAPACITY_FILE_PATH).exists() {
            self.capacity = -1;
            self.status = BatteryStatus::NoBattery;
            return Ok(());
        }

        // 3. Читаем данные батареи
        let capacity_raw = fs::read_to_string(CAPACITY_FILE_PATH)?;
        let capacity = capacity_raw.trim().parse().unwrap_or(-1);
        self.capacity = capacity;

        let status_raw = fs::read_to_string(BATTERY_STATUS_FILE_PATH)?;
        let status = match status_raw.trim() {
            "Charging" => BatteryStatus::Charging,
            "Discharging" => BatteryStatus::Discharging,
            "Full" => BatteryStatus::Full,
            "Not charging" | "Not Charging" => BatteryStatus::NotCharging,
            _ => BatteryStatus::Unknown,
        };
        self.status = status;

        Ok(())
    }

    pub fn get_icon_name(&self) -> &str {
        use BatteryStatus::*;

        // Если батареи нет, иконку вообще не выводим (или пустую строку)
        if self.capacity == -1 || self.status == NoBattery {
            return "";
        }

        match (self.status, self.capacity) {
            (Full, _) => "battery-level-100",

            // Заряжается (Charging)
            (Charging, 0..=5) => "battery-level-0-charging",
            (Charging, 6..=10) => "battery-level-10-charging",
            (Charging, 11..=20) => "battery-level-20-charging",
            (Charging, 21..=30) => "battery-level-30-charging",
            (Charging, 31..=40) => "battery-level-40-charging",
            (Charging, 41..=50) => "battery-level-50-charging",
            (Charging, 51..=60) => "battery-level-60-charging",
            (Charging, 61..=70) => "battery-level-70-charging",
            (Charging, 71..=80) => "battery-level-80-charging",
            (Charging, 81..=100) => "battery-level-100-charging",

            // Разряжается (Discharging) или любые другие статусы
            (_, 0..=5) => "battery-level-0",
            (_, 6..=10) => "battery-level-10",
            (_, 11..=20) => "battery-level-20",
            (_, 21..=30) => "battery-level-30",
            (_, 31..=40) => "battery-level-40",
            (_, 41..=50) => "battery-level-50",
            (_, 51..=60) => "battery-level-60",
            (_, 61..=70) => "battery-level-70",
            (_, 71..=80) => "battery-level-80",
            (_, 81..=100) => "battery-level-100",

            // Все остальные непредвиденные случаи
            _ => "battery-level-0",
        }
    }
}
