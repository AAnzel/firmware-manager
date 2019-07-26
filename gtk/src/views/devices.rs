use crate::{traits::DynamicGtkResize, widgets::DeviceWidget};
use firmware_manager::FirmwareInfo;
use gtk::prelude::*;
use std::num::NonZeroU8;

#[derive(Shrinkwrap)]
pub struct DevicesView {
    #[shrinkwrap(main_field)]
    container: gtk::Container,
    device_firmware: gtk::ListBox,
    system_firmware: gtk::ListBox,
}

impl DevicesView {
    pub fn new() -> Self {
        let system_firmware = cascade! {
            gtk::ListBox::new();
            ..set_margin_bottom(12);
            ..set_selection_mode(gtk::SelectionMode::None);
            ..connect_row_activated(move |_, row| {
                if let Some(widget) = row.get_child() {
                    let _ = widget.emit("button_press_event", &[&gdk::Event::new(gdk::EventType::ButtonPress)]);
                }
            });
        };

        let upper = system_firmware.downgrade();
        let device_firmware = cascade! {
            gtk::ListBox::new();
            ..set_selection_mode(gtk::SelectionMode::None);
            ..connect_row_activated(move |_, row| {
                if let Some(widget) = row.get_child() {
                    let _ = widget.emit("button_press_event", &[&gdk::Event::new(gdk::EventType::ButtonPress)]);
                }
            });
            ..connect_key_press_event(move |listbox, event| {
                gtk::Inhibit(
                    if event.get_keyval() == gdk::enums::key::Up {
                        listbox.get_children()
                            .into_iter()
                            .filter_map(|widget| widget.downcast::<gtk::ListBoxRow>().ok())
                            .next()
                            .and_then(|row| if row.has_focus() { upper.upgrade() } else { None })
                            .and_then(|upper| upper.get_children().into_iter().last())
                            .map_or(false, |child| {
                                child.grab_focus();
                                true
                            })
                    } else {
                        false
                    }
                )
            });
        };

        let lower = device_firmware.downgrade();
        system_firmware.connect_key_press_event(move |listbox, event| {
            gtk::Inhibit(if event.get_keyval() == gdk::enums::key::Down {
                listbox
                    .get_children()
                    .into_iter()
                    .filter_map(|widget| widget.downcast::<gtk::ListBoxRow>().ok())
                    .last()
                    .and_then(|row| if row.has_focus() { lower.upgrade() } else { None })
                    .and_then(|lower| lower.get_children().into_iter().next())
                    .map_or(false, |child| {
                        child.grab_focus();
                        true
                    })
            } else {
                false
            })
        });

        let layout: gtk::Box = cascade! {
            gtk::Box::new(gtk::Orientation::Vertical, 12);
            ..set_halign(gtk::Align::Center);
            ..set_margin_top(24);
            ..add(&cascade! {
                gtk::Label::new("<b>System Firmware</b>".into());
                ..set_use_markup(true);
                ..set_xalign(0.0);
            });
            ..add(&system_firmware);
            ..add(&cascade! {
                gtk::Label::new("<b>Device Firmware</b>".into());
                ..set_use_markup(true);
                ..set_xalign(0.0);
            });
            ..add(&device_firmware);
        };

        cascade! {
            gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
            ..add_widget(&system_firmware);
            ..add_widget(&device_firmware);
        };

        device_firmware.set_header_func(Some(Box::new(separator_header)));
        system_firmware.set_header_func(Some(Box::new(separator_header)));

        let container = cascade! {
            gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
            ..add(&layout);
            ..show_all();
            ..dynamic_resize(layout, NonZeroU8::new(66), None);
        };

        Self { container: container.upcast(), device_firmware, system_firmware }
    }

    pub fn clear(&self) {
        self.system_firmware.foreach(WidgetExt::destroy);
        self.device_firmware.foreach(WidgetExt::destroy);
    }

    pub fn device(&self, info: &FirmwareInfo) -> DeviceWidget {
        Self::append(&self.device_firmware, info)
    }

    pub fn system(&self, info: &FirmwareInfo) -> DeviceWidget {
        Self::append(&self.system_firmware, info)
    }

    fn append(parent: &impl gtk::ContainerExt, info: &FirmwareInfo) -> DeviceWidget {
        let widget = DeviceWidget::new(info);
        parent.add(widget.as_ref());
        widget
    }
}

fn separator_header(current: &gtk::ListBoxRow, before: Option<&gtk::ListBoxRow>) {
    if before.is_some() {
        current.set_header(Some(&gtk::Separator::new(gtk::Orientation::Horizontal)));
    }
}
