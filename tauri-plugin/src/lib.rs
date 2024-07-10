use std::{collections::HashMap, sync::Mutex};

use tauri::{
    http::uri,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use url::Url;

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::TauriPluginModuleFederation;
#[cfg(mobile)]
use mobile::TauriPluginModuleFederation;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the tauri-plugin-module-federation APIs.
pub trait TauriPluginModuleFederationExt<R: Runtime> {
    fn tauri_plugin_module_federation(&self) -> &TauriPluginModuleFederation<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TauriPluginModuleFederationExt<R> for T {
    fn tauri_plugin_module_federation(&self) -> &TauriPluginModuleFederation<R> {
        self.state::<TauriPluginModuleFederation<R>>().inner()
    }
}

#[derive(Default)]
struct Schemes(pub Mutex<HashMap<(String, Option<u16>), String>>);

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("tauri-plugin-module-federation")
        .register_asynchronous_uri_scheme_protocol(
            "module-federation",
            move |app, request, responder| {
                let app = app.clone();
                app.manage(Schemes::default());

                tauri::async_runtime::spawn(async move {
                    let schemes = app.state::<Schemes>();

                    let client = reqwest::Client::new();
                    let url = request.uri().clone();

                    let url: Url = url
                        .query()
                        .and_then(|query| {
                            let query_pairs: HashMap<_, _> =
                                form_urlencoded::parse(query.as_bytes())
                                    .into_iter()
                                    .collect();

                            query_pairs.get("fullUrl").map(|v| {
                                let url = Url::parse(v).unwrap();

                                let mut schemes = schemes.0.lock().unwrap();

                                let key = (url.host().unwrap().to_string(), url.port());

                                schemes.entry(key).or_insert(url.scheme().to_string());

                                url
                            })
                        })
                        .unwrap_or_else(|| {
                            let url = url.clone();

                            let host = url.host().unwrap().to_string();
                            let schemes = schemes.0.lock().unwrap();
                            let scheme = schemes
                                .get(&(host.clone(), url.port().map(|p| p.as_u16())))
                                .unwrap_or_else(|| {
                                    dbg!(&schemes);
                                    panic!("Unknown scheme for host '{host}:{:?}'", url.port())
                                });

                            let builder = uri::Builder::from(url).scheme(scheme.as_str());
                            Url::parse(&builder.build().unwrap().to_string()).unwrap()
                        });

                    let request_builder = client.request(request.method().clone(), url);

                    let req = request_builder.build().unwrap();
                    let mut resp = client.execute(req).await.unwrap();

                    let mut builder = tauri::http::Response::builder();

                    if let Some(h) = builder.headers_mut() {
                        *h = std::mem::take(resp.headers_mut());
                    }

                    responder.respond(
                        builder
                            .body::<Vec<u8>>(resp.bytes().await.unwrap().into())
                            .unwrap(),
                    );
                });
            },
        )
        .setup(|app, api| {
            #[cfg(mobile)]
            let tauri_plugin_module_federation = mobile::init(app, api)?;
            #[cfg(desktop)]
            let tauri_plugin_module_federation = desktop::init(app, api)?;
            app.manage(tauri_plugin_module_federation);
            Ok(())
        })
        .build()
}
