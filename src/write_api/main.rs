// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{error::Error, future::pending};

use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use tracker::SparqlConnection;
use zbus::{connection, interface};

struct ProviderObject {
    endpoint: SparqlConnection,
}

#[interface(name = "io.gitlab.TitouanReal.CcmWrite.Provider")]
impl ProviderObject {
    async fn create_collection(&mut self, provider_uri: &str, name: &str) {
        let endpoint = self.endpoint.clone();
        let statement = endpoint
            .update_statement(
                "INSERT DATA {
                    _:collection a ccm:Collection ;
                        ccm:collectionName ~name ;
                        ccm:provider ~provider_uri .
                }",
                None::<&gio::Cancellable>,
            )
            .expect("SPARQL should be valid")
            .expect("SPARQL should be valid");
        statement.bind_string("provider_uri", provider_uri);
        statement.bind_string("name", name);

        match statement.update(None::<&gio::Cancellable>) {
            Ok(()) => info!("Collection \"{name}\" created"),
            Err(err) => error!("Failed to create collection: {err:?}"),
        }
    }

    async fn create_calendar(&mut self, collection_uri: &str, name: &str, color: &str) {
        let endpoint = self.endpoint.clone();
        let statement = endpoint
            .update_statement(
                "INSERT DATA {
                    _:calendar a ccm:Calendar ;
                        ccm:collection ~collection_uri ;
                        ccm:calendarName ~name ;
                        ccm:color ~color .
                }",
                None::<&gio::Cancellable>,
            )
            .expect("SPARQL should be valid")
            .expect("SPARQL should be valid");
        statement.bind_string("collection_uri", collection_uri);
        statement.bind_string("name", name);
        statement.bind_string("color", color);

        match statement.update(None::<&gio::Cancellable>) {
            Ok(()) => info!("Calendar \"{name}\" created"),
            Err(err) => error!("Failed to create calendar: {err:?}"),
        }
    }

    async fn update_calendar_name(&mut self, uri: &str, name: &str) {
        let endpoint = self.endpoint.clone();
        let statement = endpoint
            .update_statement(
                "DELETE {
                    ~uri ccm:calendarName ?old_name
                } INSERT {
                    ~uri ccm:calendarName ~name
                } WHERE {
                    ~uri a ccm:Calendar ;
                        ccm:calendarName ?old_name .
                }",
                None::<&gio::Cancellable>,
            )
            .expect("SPARQL syntax should be valid")
            .expect("SPARQL syntax should be valid");
        statement.bind_string("uri", uri);
        statement.bind_string("name", name);

        match statement.update(None::<&gio::Cancellable>) {
            Ok(()) => info!("Calendar \"{uri}\" updated to name \"{name}\""),
            Err(err) => error!("Failed to update calendar \"{uri}\" to name \"{name}\": {err:?}"),
        }
    }

    async fn update_calendar_color(&mut self, uri: &str, color: &str) {
        let endpoint = self.endpoint.clone();
        let statement = endpoint
            .update_statement(
                "DELETE {
                    ~uri ccm:color ?old_color
                } INSERT {
                    ~uri ccm:color ~color
                } WHERE {
                    ~uri a ccm:Calendar ;
                        ccm:color ?old_color .
                }",
                None::<&gio::Cancellable>,
            )
            .expect("SPARQL should be valid")
            .expect("SPARQL should be valid");
        statement.bind_string("uri", uri);
        statement.bind_string("color", color);

        match statement.update(None::<&gio::Cancellable>) {
            Ok(()) => info!("Calendar \"{uri}\" updated to color {color}"),
            Err(err) => error!("Failed to update calendar \"{uri}\" to color {color}: {err:?}"),
        }
    }

    async fn delete_calendar(&mut self, uri: &str) {
        let endpoint = self.endpoint.clone();
        let statement = endpoint
            .update_statement(
                "DELETE DATA {
                    ~uri a ccm:Calendar .
                }",
                None::<&gio::Cancellable>,
            )
            .expect("SPARQL should be valid")
            .expect("SPARQL should be valid");
        statement.bind_string("uri", uri);

        match statement.update(None::<&gio::Cancellable>) {
            Ok(()) => info!("Calendar \"{uri}\" deleted"),
            Err(err) => error!("Failed to delete calendar \"{uri}\": {err:?}"),
        }
    }

    async fn create_event(&mut self, calendar_uri: &str, name: &str, description: &str) {
        let endpoint = self.endpoint.clone();
        let statement = endpoint
            .update_statement(
                "INSERT DATA {
                    _:event a ccm:Event ;
                        ccm:calendar ~calendar_uri ;
                        ccm:eventName ~name ;
                        ccm:eventDescription ~description .
                }",
                None::<&gio::Cancellable>,
            )
            .expect("SPARQL should be valid")
            .expect("SPARQL should be valid");
        statement.bind_string("calendar_uri", calendar_uri);
        statement.bind_string("name", name);
        statement.bind_string("description", description);

        match statement.update(None::<&gio::Cancellable>) {
            Ok(()) => info!("Event \"{name}\" created"),
            Err(err) => error!("Failed to create event: {err:?}"),
        }
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
        endpoint: sparql_connection,
    };
    let _conn = connection::Builder::session()?
        .name("io.gitlab.TitouanReal.CcmWrite")?
        .serve_at("/io/gitlab/TitouanReal/CcmWrite/Provider", provider)?
        .build()
        .await?;

    pending::<()>().await;

    Ok(())
}
