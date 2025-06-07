// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{error::Error, future::pending, sync::Mutex};

use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use tracker::{SparqlConnection, gio};
use zbus::{connection, interface};

struct ProviderObject {
    endpoint: Mutex<SparqlConnection>,
}

#[interface(name = "io.gitlab.TitouanReal.CcmWrite.Provider")]
impl ProviderObject {
    async fn add_collection(&mut self, provider_uri: &str, collection_name: &str) {
        let endpoint = self.endpoint.lock().unwrap();
        endpoint.update_async(
            &format!(
                "INSERT {{
                    GRAPH ccm:Calendar {{
                        _:collection a ccm:Collection ;
                            rdfs:label \"{collection_name}\" ;
                            ccm:provider \"{provider_uri}\".
                    }}
                }}",
            ),
            None::<&gio::Cancellable>,
            |_| {},
        );
    }

    async fn add_calendar(&mut self, collection_uri: &str, name: &str, color: &str) {
        let endpoint = self.endpoint.lock().unwrap();
        info!(
            "Creating calendar {} of color {} to collection {}...",
            name, color, collection_uri
        );
        endpoint.update_async(
            &format!(
                "INSERT {{
                    GRAPH ccm:Calendar {{
                        _:calendar a ccm:Calendar ;
                            ccm:collection \"{collection_uri}\" ;
                            rdfs:label \"{name}\" ;
                            ccm:color \"{color}\" .
                    }}
                }}",
            ),
            None::<&gio::Cancellable>,
            |_| {},
        );
        info!("Calendar {} created", name);
    }

    async fn add_event(&mut self, calendar_uri: &str, event_name: &str) {
        let endpoint = self.endpoint.lock().unwrap();
        endpoint.update_async(
            &format!(
                "INSERT {{
                    GRAPH ccm:Calendar {{
                        _:event a ccm:Event ;
                            rdfs:label \"{event_name}\" ;
                            ccm:calendar \"{calendar_uri}\".
                    }}
                }}",
            ),
            None::<&gio::Cancellable>,
            |_| {},
        );
    }

    async fn delete_calendar(&mut self, uri: &str) {
        let endpoint = self.endpoint.lock().unwrap();
        info!("Deleting calendar {}", uri);
        endpoint.update_async(
            &format!(
                "DELETE {{
                    GRAPH ccm:Calendar {{
                        {uri} a ccm:Calendar.
                    }}
                }}",
            ),
            None::<&gio::Cancellable>,
            |_| {},
        );
        info!("Calendar {} deleted", uri);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(env_filter))
        .init();

    let sparql_connection =
        SparqlConnection::bus_new("io.gitlab.TitouanReal.CcmRead", None, None).unwrap();

    let provider = ProviderObject {
        endpoint: Mutex::new(sparql_connection),
    };
    let _conn = connection::Builder::session()?
        .name("io.gitlab.TitouanReal.CcmWrite")?
        .serve_at("/io/gitlab/TitouanReal/CcmWrite/Provider", provider)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}
