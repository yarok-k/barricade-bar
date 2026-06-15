use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;
use pulsectl::controllers::types::DeviceInfo;

#[derive(Clone)]
pub struct AudioLib {
    pub volume: u32,
    pub mute: bool,
}

impl AudioLib {
    pub fn new() -> Self {
        let mut audio = Self {
            volume: 100,
            mute: false,
        };
        audio.update();

        audio
    }

    /// Запрашивает свежие данные из системы и обновляет внутреннее состояние структуры.
    /// Так как мы меняем поля, функции нужен `&mut self`.
    pub fn update(&mut self) {
        // Вскрываем Option. Если там Some(device), работаем с девайсом напрямую
        if let Some(device) = self.get_system_device() {
            // Берем громкость первого канала напрямую из u32 (без косяков с парсингом строк!)
            if let Some(chan_vol) = device.volume.get().first() {
                let raw_value = chan_vol.0 as f32;
                self.volume = ((raw_value / 65536.0) * 100.0).round() as u32;
            }

            self.mute = device.mute;
        } else {
            // Фолбек на случай, если PulseAudio не ответил
            println!("Ошибка: не удалось получить аудиоустройство");
        }
    }
    /// ТЕПЕРЬ РАБОТАЕТ: Изменяет мут В СИСТЕМЕ
    pub fn set_mute(&mut self, mute: bool) {
        if let Some(mut handler) = SinkController::create().ok() {
            if let Some(device) = handler.get_default_device().ok() {
                // Метод называется set_device_mute_by_index
                handler.set_device_mute_by_index(device.index, mute);

                // Обновляем локальный кэш
                self.mute = mute;
            }
        }
    }

    /// ДОБАВИМ СЕТТЕР ГРОМКОСТИ: Изменяет громкость В СИСТЕМЕ (0-100)
    pub fn set_volume(&mut self, pct: u32) {
        if let Some(mut handler) = SinkController::create().ok() {
            if let Some(device) = handler.get_default_device().ok() {
                let raw_volume = ((pct as f32 / 100.0) * 65536.0).round() as u32;

                let mut new_volume = device.volume.clone();

                // .get_mut() возвращает слайс со всеми каналами, меняем их в цикле
                for channel_vol in new_volume.get_mut() {
                    channel_vol.0 = raw_volume; // .0 так как это Tuple Struct вокруг u32
                }

                handler.set_device_volume_by_index(device.index, &new_volume);
                self.volume = pct;
            }
        }
    }

    /// Возвращает название иконки в формате freedesktop.
    pub fn get_icon_name(&self) -> &'static str {
        if self.mute || self.volume <= 0 {
            return "audio-volume-muted";
        }
        match self.volume {
            1..=30 => "audio-volume-low",
            31..=70 => "audio-volume-medium",
            _ => "audio-volume-high",
        }
    }
    fn get_system_device(&self) -> Option<DeviceInfo> {
        let mut handler = SinkController::create().ok()?;
        let system_device = handler.get_default_device().ok()?;
        Some(system_device)
    }
}
