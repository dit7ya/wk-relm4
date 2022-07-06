use gtk::prelude::{BoxExt, ButtonExt, EntryExt, GtkWindowExt, OrientableExt};
use relm4::gtk::prelude::EditableExt;
use relm4::gtk::prelude::EntryBufferExtManual;
use relm4::{gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

#[derive(Default)]
struct AppModel {
    counter: u32,
    name: String,
    matches: Vec<String>,
}

enum AppMsg {
    Increment,
    Decrement,
    Double,
    SetName(String),
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }

            AppMsg::Double => {
                self.counter = self.counter.wrapping_mul(2);
            }

            AppMsg::SetName(name) => {
                self.name = name.clone();
                self.matches = vec![name.clone(), name.clone(), name.clone()];
            }
        }
        true
    }
}
fn get_list(items: &Vec<String>) -> gtk::ListBox {
    let list = gtk::ListBox::new();

    for item in items {
        let label = gtk::Label::new(Some(&format!("{}", item)));

        list.append(&label);
    }
    list
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        gtk::ApplicationWindow {
            set_title: Some("Nice App"),
            set_default_width: 300,
            set_default_height: 100,
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                append = &gtk::Button {
                    set_label: "Increment",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::Increment);
                    },
                },
                append = &gtk::Button::with_label("Decrement") {
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::Decrement);
                    },
                },

                append = &gtk::Button::with_label("Double!") {
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::Double);
                    },
                },

                append = &gtk::Entry{
                    connect_changed(sender) => move |entry| {
                        let buffer = entry.buffer().text() ;
                        send!(sender, AppMsg::SetName(buffer));
                    }
                },

                append = &gtk::Label {
                    set_label: watch! { &format!("Counter: {}", model.counter) },
                },
                append = &gtk::Label {
                    set_label: watch! { &format!("Hello: {}", model.name) },
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 5,
                    set_spacing: 5,
                },
                append: watch! { &get_list(&model.matches)}
            },
        }
    }
}

fn main() {
    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run();
}
