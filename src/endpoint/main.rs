// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use gio::{self, BusNameOwnerFlags, BusType, DBusConnection, bus_own_name, glib::MainLoop};
use tracker::{EndpointDBus, SparqlConnection, SparqlConnectionFlags};

fn read_bus_acquired(dbus_connection: DBusConnection, _str: &str) {
    let file = gio::File::for_path("./ontology");

    let sparql_connection = SparqlConnection::new(
        SparqlConnectionFlags::NONE,
        None::<&gio::File>,
        Some(&file),
        None::<&gio::Cancellable>,
    )
    .unwrap();

    sparql_connection.update_async(
        "INSERT DATA {
            _:local_provider a ccm:Provider ;
                rdfs:label \"Local\" .

            _:local_collection a ccm:Collection ;
                ccm:provider _:local_provider ;
                rdfs:label \"My Personal Local Collection\" .

            _:local_calendar a ccm:Calendar ;
                ccm:collection _:local_collection ;
                rdfs:label \"My Personal Local Calendar\" ;
                ccm:color \"#FF0000\" .

            _:event_1 a ccm:Event ;
                ccm:calendar _:local_calendar ;
                rdfs:label \"Going out for a walk\" .

            _:local_calendar_2 a ccm:Calendar ;
                ccm:collection _:local_collection ;
                rdfs:label \"Empty Calendar\" ;
                ccm:color \"#FF00FF\" .

            _:google_provider a ccm:Provider ;
                rdfs:label \"Google\" .

            _:titouan_real a ccm:Collection ;
                ccm:provider _:google_provider ;
                rdfs:label \"My Personal Google Collection\" .

            _:google_calendar a ccm:Calendar ;
                ccm:collection _:titouan_real ;
                rdfs:label \"titouan.real@gmail.com\" ;
                ccm:color \"#00FF00\" .

            _:event_2 a ccm:Event ;
                ccm:calendar _:google_calendar ;
                rdfs:label \"Synced up meeting\" .

            _:jeff a ccm:Collection ;
                ccm:provider _:google_provider ;
                rdfs:label \"Jeff's Collection\" .

            _:other_google_calendar a ccm:Calendar ;
                ccm:collection _:jeff ;
                rdfs:label \"jeff@gmail.com\" ;
                ccm:color \"#00FFFF\" .

            _:event_3 a ccm:Event ;
                ccm:calendar _:google_calendar ;
                rdfs:label \"Going out for a walk\" .
        }",
        None::<&gio::Cancellable>,
        |result| println!("{result:?}"),
    );

    let endpoint = Box::new(
        EndpointDBus::new(
            &sparql_connection,
            &dbus_connection,
            None,
            None::<&gio::Cancellable>,
        )
        .unwrap(),
    );

    Box::leak(endpoint);
}

fn read_name_acquired(_connection: DBusConnection, _str: &str) {}

fn read_name_lost(_option_connection: Option<DBusConnection>, _str: &str) {}

fn main() {
    let main_loop = MainLoop::new(None, false);

    let _read_owner_id = bus_own_name(
        BusType::Session,
        "io.gitlab.TitouanReal.CcmRead",
        BusNameOwnerFlags::DO_NOT_QUEUE,
        read_bus_acquired,
        read_name_acquired,
        read_name_lost,
    );

    main_loop.run();
}
