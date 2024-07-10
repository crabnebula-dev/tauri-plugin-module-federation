# tauri-plugin-module-federation

A Tauri plugin and [Federation Runtime](https://module-federation.io/guide/basic/runtime.html) plugin that cache remote modules for offline use.

The Federation Runtime plugin rewrites remote module requests to use the `module-federation://` URI scheme, which is then handled by the Tauri plugin.
Right now the URI scheme acts as a simple proxy, but soon it will cache remote assets locally and serve those if loading the remote assets fails.
