use gtk4::prelude::*;
use libadwaita::prelude::*;
use relm4::prelude::*;

pub struct SmartcardItem {
    name: String,
}

#[relm4::factory(pub)]
impl FactoryComponent for SmartcardItem {
    type Init = String;
    type Input = String;
    type Output = HomeMessage;
    type CommandOutput = ();
    type ParentWidget = libadwaita::PreferencesGroup;

    view! {
        libadwaita::ActionRow {
            set_title: &self.name,

            add_prefix: &gtk4::Image::from_icon_name("usb-stick-symbolic"),
            add_suffix = &gtk4::Button {
                add_css_class: "dangerous",
                set_label: "Khởi động lại",
                set_valign: gtk4::Align::Center,
                set_halign: gtk4::Align::Center,
            },
        },
    }

    fn init_model(name: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { name }
    }
}

pub struct HomeUi {
    pub smartcards: FactoryVecDeque<SmartcardItem>,
}

#[derive(Debug)]
pub enum HomeMessage {
    MoreDetails(u16),
}

#[relm4::component(pub)]
impl SimpleComponent for HomeUi {
    type Init = bool;
    type Input = HomeMessage;
    type Output = ();

    view! {
        #[root]
        libadwaita::ApplicationWindow {
            set_title: Some("Thực ra :3"),
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
                    #[local_ref]
                    counter_box -> libadwaita::PreferencesGroup {
                        set_title: "Danh sách thiết bị",
                    }
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>
    ) -> ComponentParts<Self> {
        let mut model = HomeUi {
            smartcards: FactoryVecDeque::builder()
                .launch(libadwaita::PreferencesGroup::default())
                .forward(sender.input_sender(), |output| {
                    match output {
                        HomeMessage::MoreDetails(so) => HomeMessage::MoreDetails(so),
                    }
                }),
        };

        let counter_box = model.smartcards.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            HomeMessage::MoreDetails(so) => {
                tracing::info!(so);
            }
        }
    }
}
