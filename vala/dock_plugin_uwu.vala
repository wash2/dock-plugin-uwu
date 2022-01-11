using Uwu;
namespace DockPlugin {
	public Gtk.Image image() {
		try {
			var f = Environment.get_user_data_dir() + "/com.cosmic.dock/plugins/uwu.gif";
	    	var uwu = new Gdk.PixbufAnimation.from_file(f);
	    	var uwu_iter = uwu.get_iter(new TimeVal ());
	    	var image = new Gtk.Image ();
	    	image.add_css_class("dock_plugin");
	    	image.set_pixel_size(64);
	    	image.set_from_pixbuf(uwu_iter.get_pixbuf());
	    	Timeout.add(uwu_iter.get_delay_time(), () => {
	    			uwu_iter.advance(new TimeVal ());
	    			image.set_from_pixbuf(uwu_iter.get_pixbuf());
	    			return true;
	    		});
	    	return image;
		} catch (Error e) {
			return new Gtk.Image();
		}
	}

	public Gtk.Box popover_menu() {
		var menu_box = new Gtk.Box(Gtk.Orientation.VERTICAL, 4);
		var uwu_entry = new Gtk.Entry();
		uwu_entry.set_placeholder_text(" Twext here UwU");
		menu_box.append(uwu_entry);
		uwu_entry.add_css_class("dock_plugin");
		var uwu_translation = new Gtk.Label("");
		uwu_translation.set_selectable(true);
		menu_box.append(uwu_translation);
		uwu_translation.add_css_class("dock_plugin");
		uwu_entry.changed.connect(() => {
				uint8[] buffer = new uint8[4096];
				var success = uwu(uwu_entry.get_text(), buffer, buffer.length);
				if (success) {
					var builder = new StringBuilder();
					for(int i=0; i<buffer.length; i++) {
						builder.append_c((char)buffer[i]);
					}
					uwu_translation.set_label(builder.str);
				} else {
					uwu_translation.set_label(uwu_entry.get_text());
				}
			});
		return menu_box;
	}

	public string name() {
		return "UwU!";
	}
}
