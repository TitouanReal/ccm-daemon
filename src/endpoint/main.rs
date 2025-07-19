// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use gio::{self, BusNameOwnerFlags, BusType, DBusConnection, bus_own_name, glib::MainLoop};
use tracker::{EndpointDBus, SparqlConnection, SparqlConnectionFlags};

fn read_bus_acquired(dbus_connection: DBusConnection, _str: &str) {
    let ontology_file = gio::File::for_path("./ontology");
    let db_file = gio::File::for_path("./data/data.db");

    let sparql_connection = SparqlConnection::new(
        SparqlConnectionFlags::NONE,
        Some(&db_file),
        Some(&ontology_file),
        None::<&gio::Cancellable>,
    )
    .unwrap();

    sparql_connection.update_async(
        "INSERT DATA {
            _:local_provider a ccm:Provider ;
                ccm:providerName \"Local\" .

            _:local_collection a ccm:Collection ;
                ccm:provider _:local_provider ;
                ccm:collectionName \"My Personal Local Collection\" .

            _:local_calendar a ccm:Calendar ;
                ccm:collection _:local_collection ;
                ccm:calendarName \"My Personal Local Calendar\" ;
                ccm:color \"#FF0000\" .

            _:event_1 a ccm:Event ;
                ccm:calendar _:local_calendar ;
                ccm:eventName \"Going out for a walk\" ;
                ccm:eventDescription \"Walking in the park\" ;
                ccm:eventStart \"2025-07-19T10:00:00[Europe/Paris]\" ;
                ccm:eventEnd \"2025-07-19T11:00:00[Europe/Paris]\" ;
                ccm:eventAllDay false .

            _:local_calendar_2 a ccm:Calendar ;
                ccm:collection _:local_collection ;
                ccm:calendarName \"Empty Calendar\" ;
                ccm:color \"#FF00FF\" .

            _:google_provider a ccm:Provider ;
                ccm:providerName \"Google\" .

            _:titouan_real a ccm:Collection ;
                ccm:provider _:google_provider ;
                ccm:collectionName \"My Personal Google Collection\" .

            _:google_calendar a ccm:Calendar ;
                ccm:collection _:titouan_real ;
                ccm:calendarName \"titouan.real@gmail.com\" ;
                ccm:color \"#00FF00\" .

            _:event_2 a ccm:Event ;
                ccm:calendar _:google_calendar ;
                ccm:eventName \"Synced up meeting\" ;
                ccm:eventDescription \"Meeting with Jeff\" ;
                ccm:eventStart \"2025-07-20T10:00:00[Europe/Paris]\" ;
                ccm:eventEnd \"2025-07-20T11:00:00[Europe/Paris]\" ;
                ccm:eventAllDay false .

            _:jeff a ccm:Collection ;
                ccm:provider _:google_provider ;
                ccm:collectionName \"Jeff's Collection\" .

            _:other_google_calendar a ccm:Calendar ;
                ccm:collection _:jeff ;
                ccm:calendarName \"jeff@gmail.com\" ;
                ccm:color \"#00FFFF\" .

            _:event_3 a ccm:Event ;
                ccm:calendar _:google_calendar ;
                ccm:eventName \"Going to New York!\" ;
                ccm:eventDescription \"Walking with Jeff\" ;
                ccm:eventStart \"2025-07-22T10:00:00[Europe/Paris]\" ;
                ccm:eventEnd \"2025-07-22T12:00:00[America/New_York]\" ;
                ccm:eventAllDay false .

            _:event_4 a ccm:Event ;
                ccm:calendar _:google_calendar ;
                ccm:eventName \"Some holidays in NY\" ;
                ccm:eventDescription \"I will visit the Statue of Liberty and Central Park\" ;
                ccm:eventStart \"2025-07-23\" ;
                ccm:eventEnd \"2025-07-26\" ;
                ccm:eventAllDay true .
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
