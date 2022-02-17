#![feature(trace_macros)]
use cascade::cascade;
use cosmic_plugin::*;
use gdk_pixbuf::glib::SourceId;
use gtk4::{glib, prelude::*, Align, Box, Orientation, Popover, PositionType};
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use uwuifier::uwuify_str_sse;

#[derive(Debug, Default)]
pub struct Uwu {
    handle: OnceCell<SourceId>,
    applet: OnceCell<gtk4::Box>,
    popover: OnceCell<Popover>,
}

const ID: &str = "com.cosmic.uwu";

impl Plugin for Uwu {
    fn css_provider(&self) -> gtk4::CssProvider {
        // Load the css file and add it to the provider
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));
        provider
    }

    fn on_plugin_unload(&mut self) {
        // XXX handle must not be 0
        // unsafe { glib::translate::from_glib::<_, SourceId>(self.handle).remove() };
        self.handle.take().unwrap().remove();
        drop(self.applet.take().unwrap());
        drop(self.popover.take().unwrap());
    }

    fn applet(&self) -> gtk4::Box {
        self.applet.get().unwrap().clone()
    }

    fn set_position(&self, position: Position) {
        let p = self.popover.get().unwrap();
        p.set_position(match position {
            Position::Start => PositionType::Right,
            Position::End => PositionType::Left,
            Position::Top => PositionType::Bottom,
            Position::Bottom => PositionType::Top,
        });
    }

    fn set_size(&self, _size: Size) {
        // currently not used
    }
    fn on_plugin_load(&mut self) {
        let b = cascade! {
            Box::new(Orientation::Vertical, 0);
            ..set_valign(Align::Center);
            ..set_height_request(64);
            ..set_width_request(64);
            ..add_css_class("dock_item");
        };

        // load image
        let mut image_f = PathBuf::from(ID);
        std::fs::create_dir_all(&image_f).unwrap();
        image_f.push("uwu.gif");
        if let Some(image_f) = cosmic_plugin::get_path_to_xdg_data(image_f) {
            if let Ok(pixbuf) = gdk_pixbuf::PixbufAnimation::from_file(image_f) {
                let pixbuf_iter = pixbuf.iter(Some(SystemTime::now()));
                let image = cascade! {
                    gtk4::Image::from_pixbuf(Some(&pixbuf_iter.pixbuf()));
                    ..set_pixel_size(64);
                    ..add_css_class("dock");
                    ..set_tooltip_text(Some("UWU!"));
                };
                let id = glib::timeout_add_local(
                    Duration::from_millis(pixbuf_iter.delay_time().try_into().unwrap_or(100)),
                    glib::clone!(@strong pixbuf_iter, @weak image => @default-return Continue(true), move || {
                        pixbuf_iter.advance(SystemTime::now());
                        image.set_from_pixbuf(Some(&pixbuf_iter.pixbuf()));
                        Continue(true)
                    }),
                );
                // XXX if handle isn't saved and removed, then when the plugin is unloaded it will crash bc of the strong reference.
                self.handle.set(id).unwrap();
                b.append(&image);
            }
        }
        // setup popover
        let uwu_translation = cascade! {
            gtk4::Label::new(Some(""));
            ..set_selectable(true);
            ..add_css_class("dock_popover_title");
        };
        let uwu_entry = cascade! {
            gtk4::Entry::new();
            ..set_placeholder_text(Some(" entew twext hewe pwease <3"));
            ..add_css_class("dock_plugin_uwu");
            ..connect_changed(glib::clone!(@weak uwu_translation => move |entry| {
                let new_translation = uwuify_str_sse(&entry.buffer().text());
                uwu_translation.set_text(new_translation.as_str());
            }));
        };
        let popover_menu = cascade! {
            Box::new(gtk4::Orientation::Vertical, 4);
            ..append(&uwu_entry);
            ..append(&uwu_translation);
            ..add_css_class("popover_menu");
        };
        let popover = cascade! {
            gtk4::Popover::new();
            ..set_autohide(true);
            ..set_child(Some(&popover_menu));
            ..add_css_class("dock");
        };
        b.append(&popover);

        // setup click events
        let click_controller = gtk4::GestureClick::builder()
            .button(0)
            .propagation_limit(gtk4::PropagationLimit::None)
            .propagation_phase(gtk4::PropagationPhase::Capture)
            .build();
        b.add_controller(&click_controller);

        click_controller.connect_released(
            glib::clone!(@weak popover, @weak b => move |_controller, _, x, y| {
                if x < 0.0 || y < 0.0 || x > b.width().into() || y > b.height().into() {
                    popover.popdown();
                } else {
                    popover.popup();
                }
            }),
        );
        self.popover.set(popover).unwrap();
        self.applet.set(b).unwrap();
    }
}

trace_macros!(true);
declare_plugin!(Uwu);
