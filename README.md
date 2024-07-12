# tauri-plugin-module-federation

A Tauri plugin and [Federation Runtime](https://module-federation.io/guide/basic/runtime.html) plugin that cache remote modules for offline use.

The Federation Runtime plugin rewrites remote module requests to use the `module-federation://` URI scheme, which is then handled by the Tauri plugin.
Files loaded over this URI scheme are cached for serving when fetching from the network fails.
