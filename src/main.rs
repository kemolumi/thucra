use gtk4::{
    gio::prelude::{ ApplicationExt, ApplicationExtManual },
    prelude::{ BoxExt, ButtonExt, GtkWindowExt },
};

use libadwaita::{ ActionRow, prelude::{ ActionRowExt, AdwApplicationWindowExt } };
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
    match lsl::init().await {
        Ok(_) => {}
        Err(_) => {
            tracing::error!("Can't launch server :(");
            return;
        }
    }

    let ctx = match pcsc::Context::establish(pcsc::Scope::User) {
        Ok(ctx) => ctx,
        Err(err) => {
            tracing::error!("Failed to establish context: {}", err);
            return;
        }
    };

    let mut readers_buf = [0; 2048];
    let mut readers = match ctx.list_readers(&mut readers_buf) {
        Ok(readers) => readers,
        Err(err) => {
            tracing::error!("Failed to list readers: {}", err);
            return;
        }
    };

    let reader = match readers.next() {
        Some(reader) => reader,
        None => {
            tracing::error!("No readers are connected.");
            return;
        }
    };
    tracing::info!("Using reader: {:?}", reader);

    let card = match ctx.connect(reader, pcsc::ShareMode::Shared, pcsc::Protocols::ANY) {
        Ok(card) => card,
        Err(pcsc::Error::NoSmartcard) => {
            tracing::error!("A smartcard is not present in the reader.");
            return;
        }
        Err(err) => {
            tracing::error!("Failed to connect to card: {}", err);
            return;
        }
    };

    let apdu = b"\x00\xa4\x04\x00\x0A\xA0\x00\x00\x00\x62\x03\x01\x0C\x06\x01";
    tracing::info!("Sending APDU: {:?}", apdu);
    let mut rapdu_buf = [0; pcsc::MAX_BUFFER_SIZE];
    let rapdu = match card.transmit(apdu, &mut rapdu_buf) {
        Ok(rapdu) => rapdu,
        Err(err) => {
            tracing::error!("Failed to transmit APDU command to card: {}", err);
            return;
        }
    };
    tracing::info!("APDU response: {:?}", rapdu);

    let application = libadwaita::Application
        ::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        // ActionRows are only available in Adwaita
        let row = ActionRow::builder().activatable(true).title("Click me").build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });

        let list = gtk4::ListBox
            ::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(gtk4::SelectionMode::None)
            // makes the list look nicer
            .css_classes(vec![String::from("boxed-list")])
            .build();
        list.append(&row);

        // Combine the content in a box
        let content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(&libadwaita::HeaderBar::new());
        content.append(&list);

        let window = libadwaita::ApplicationWindow
            ::builder()
            .application(app)
            .decorated(true)
            .title("")
            .default_width(350)
            .default_height(70)
            .content(&content)
            .build();

        window.present();
    });

    application.run();

    lsl::core().await;
}
