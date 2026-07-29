use gtk4::prelude::*;
use libadwaita::prelude::*;
use relm4::prelude::*;
use tokio::sync::mpsc;

pub struct HomeUi {
    pub vnpt_ca_restart_trip: mpsc::Sender<()>,
    pub signinghub_restart_trip: mpsc::Sender<()>,
}

#[derive(Debug)]
pub enum HomeMessage {
    RestartVnpt,
    RestartSigningHub,
}

#[relm4::component(pub)]
impl SimpleComponent for HomeUi {
    type Init = HomeUi;
    type Input = HomeMessage;
    type Output = ();

    view! {
        libadwaita::ApplicationWindow {
            set_decorated: true,
            set_title: Some("Thực ra :3"),
            set_resizable: false,
            set_default_size: (520, 480),

            connect_close_request => move |window| {
                window.hide();
                gtk::glib::Propagation::Stop 
            },

            gtk4::Box {
                set_orientation: gtk4::Orientation::Vertical,
                set_spacing: 0,

                libadwaita::HeaderBar {
                    set_title_widget: Some(
                        &gtk4::Label::builder().label("Thực ra :3").css_classes(vec!["title"]).build()
                    ),
                    pack_start: &gtk4::Button::from_icon_name("open-menu-symbolic"),
                },

                libadwaita::PreferencesPage {
                    libadwaita::PreferencesGroup {
                        set_title: "VNPT CA Plugin",

                        libadwaita::ActionRow {
                            set_title: "Dịch vụ",

                            add_prefix: &gtk4::Image::from_icon_name("network-server-symbolic"),
                            add_suffix = &gtk4::Button {
                                add_css_class: "dangerous",
                                set_label: "Khởi động lại",
                                set_valign: gtk4::Align::Center,
                                set_halign: gtk4::Align::Center,
                                connect_clicked => HomeMessage::RestartVnpt
                            },
                        }
                    },

                    libadwaita::PreferencesGroup {
                        set_title: "CTSigningHub",

                        libadwaita::ActionRow {
                            set_title: "Dịch vụ",

                            add_prefix: &gtk4::Image::from_icon_name("network-server-symbolic"),
                            add_suffix = &gtk4::Button {
                                add_css_class: "dangerous",
                                set_label: "Khởi động lại",
                                set_valign: gtk4::Align::Center,
                                set_halign: gtk4::Align::Center,
                                connect_clicked => HomeMessage::RestartSigningHub
                            },
                        }
                    },
                }
            }
        }
    }

    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>
    ) -> ComponentParts<Self> {
        let model = counter;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            HomeMessage::RestartVnpt => {
                self.vnpt_ca_restart_trip.blocking_send(());
            }
            HomeMessage::RestartSigningHub => {
                tracing::info!("Hub");
            }
        }
    }
}
