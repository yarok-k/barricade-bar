use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, SystemTime};

use gtk::prelude::*;
use gtk::{CssProvider, StyleContext, Application, ApplicationWindow, glib, gdk};

use ui::Ui;
use lib::core::{parcer::{modified_time, Parser},flags::parse_flags};

mod lib;
mod ui;
mod widgets;

// Следит за конфигом и CSS-файлом и перезагружает стили на лету.
// Провайдер создаётся один раз и только перезаполняется,
// поэтому провайдеры не накапливаются.
struct StyleWatcher {
    provider: CssProvider,
    css_path: Option<PathBuf>,
    config_mtime: Option<SystemTime>,
    css_mtime: Option<SystemTime>,
}

impl StyleWatcher {
    fn new() -> Self {
        let provider = CssProvider::new();
        match gdk::Screen::default() {
            Some(screen) => StyleContext::add_provider_for_screen(
                &screen,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            ),
            None => eprintln!("stylesheet disabled: no default screen"),
        }

        let mut watcher = Self {
            provider,
            css_path: None,
            config_mtime: None,
            css_mtime: None,
        };
        watcher.reload_config();
        watcher
    }

    // Перечитывает конфиг (путь к CSS мог поменяться) и заново грузит стили
    fn reload_config(&mut self) {
        let parser = Parser::new();
        // mtime берём до парсинга, чтобы ошибка в конфиге не повторялась каждый тик
        self.config_mtime = modified_time(parser.config_path());

        match parser.parse() {
            Ok(cfg) => self.css_path = cfg.stylesheet,
            Err(e) => eprintln!("config reload failed: {e:#}"),
        }
        self.reload_css();
    }

    fn reload_css(&mut self) {
        self.css_mtime = self.css_path.as_deref().and_then(modified_time);

        let result = match &self.css_path {
            Some(path) => self.provider.load_from_path(&path.to_string_lossy()),
            // стиль убрали из конфига — очищаем провайдер
            None => self.provider.load_from_data(b""),
        };
        if let Err(e) = result {
            eprintln!("stylesheet load failed: {e}");
        }
    }

    fn tick(&mut self) {
        let parser = Parser::new();
        if modified_time(parser.config_path()) != self.config_mtime {
            self.reload_config();
        } else if self.css_path.as_deref().and_then(modified_time) != self.css_mtime {
            self.reload_css();
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = parse_flags();

    let app = Application::builder()
        .application_id("ru.bst.Barricade.bar")
        .build();

    let rebuild_flag = Rc::new(RefCell::new(args.update));

    app.connect_startup(|_| {
        let watcher = Rc::new(RefCell::new(StyleWatcher::new()));
        glib::timeout_add_local(Duration::from_millis(500), move || {
            watcher.borrow_mut().tick();
            glib::ControlFlow::Continue
        });
    });

    let flag_for_activate = Rc::clone(&rebuild_flag);
    app.connect_activate(move |app| {
        rebuild_windows(app);

        // Подписываемся на добавление мониторов только один раз при первичном старте
        let once_guard = Rc::new(RefCell::new(false));
        if !*once_guard.borrow() {
            *once_guard.borrow_mut() = true;
            if let Some(display) = gtk::gdk::Display::default() {
                let app_clone = app.clone();
                display.connect_monitor_added(move |_, monitor| {
                    create_window_for_monitor(&app_clone, monitor);
                });
            }
        }

        *flag_for_activate.borrow_mut() = false;
    });

    // Если запущен с -u/--update и уже есть активный процесс с этим application_id:
    // регистрируем приложение и активируем его, чтобы сработал connect_activate в основном процессе.
    if args.update {
        app.register(None::<&gio::Cancellable>)?;
        if app.is_remote() {
            app.activate();
            return Ok(());
        }
    }

    let exit_code = app.run();
    if exit_code.value() != 0 {}
    Ok(())
}

fn create_window_for_monitor(app: &Application, monitor: &gdk::Monitor) {
    let win = ApplicationWindow::builder()
        .application(app)
        .decorated(false)
        .build();

    gtk_layer_shell::init_for_window(&win);
    gtk_layer_shell::set_monitor(&win, monitor);
    gtk_layer_shell::set_layer(&win, gtk_layer_shell::Layer::Top);
    gtk_layer_shell::set_anchor(&win, gtk_layer_shell::Edge::Top, true);
    gtk_layer_shell::set_anchor(&win, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&win, gtk_layer_shell::Edge::Right, true);
    gtk_layer_shell::set_margin(&win, gtk_layer_shell::Edge::Bottom, 10);
    gtk_layer_shell::set_exclusive_zone(&win, 35);
    let _ui = Ui::new(&win, app);

    win.show_all();
}


fn rebuild_windows(app: &Application) {
    // Закрываем существующие окна бара перед пересборкой
    for win in app.windows() {
        win.close();
    }

    let Some(display) = gtk::gdk::Display::default() else {
        eprintln!("Could not get default display");
        return;
    };

    let monitor_count = display.n_monitors();
    for i in 0..monitor_count {
        if let Some(monitor) = display.monitor(i) {
            create_window_for_monitor(app, &monitor);
        }
    }
}
