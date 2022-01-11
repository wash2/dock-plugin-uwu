valac --library=shared -H dock_plugin_uwu.h libuwuifier.vapi dock_plugin_uwu.vala -X ~/.local/share/com.cosmic.dock/plugins/libuwuifier.so -X -fPIC -X -I. -X -shared -o dock_plugin_uwu.so --pkg gtk4
cp dock_plugin_uwu.* ~/.local/share/com.cosmic.dock/plugins/
cp libuwuifier.* ~/.local/share/com.cosmic.dock/plugins/
