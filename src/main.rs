use gtk4::prelude::*;
use relm4::prelude::*;
use thucra::{ app::home::HomeUi, vnpt::VnptCa };

fn main() {
    let _guard = match thucra::init() {
        Ok(guard) => guard,
        Err(_) => {
            tracing::error!("Application initialize failed.");
            return;
        }
    };

    let app = RelmApp::new("com.kemolumi.thucra");

    let settings = gtk::Settings::default().unwrap();
    apply_css(settings.is_gtk_application_prefer_dark_theme());

    settings.connect_gtk_application_prefer_dark_theme_notify(|settings| {
        apply_css(settings.is_gtk_application_prefer_dark_theme());
    });

    relm4::main_application().connect_startup(move |_| {
        std::thread::spawn(move || {
            tokio_runtime(async {
                let mut vnpt_ca = VnptCa::new().await;
                vnpt_ca.launch().await;
            });
        });
    });

    relm4::main_application().connect_activate(move |app| {
        let windows = app.windows();

        for window in &windows {
            if window.is_visible() {
                window.present();
                tracing::info!(
                    "Already have an active window, sending user to the closest window."
                );
                return;
            }
        }
    });

    app.run::<HomeUi>(HomeUi {});
}

fn apply_css(is_dark: bool) {
    if is_dark {
        relm4::set_global_css(include_str!("./app/style.dark.css"));
    } else {
        relm4::set_global_css(include_str!("./app/style.light.css"));
    }
}

fn tokio_runtime<T: Future>(future: T) {
    tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(future);
}
