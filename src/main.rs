use chrono::prelude::*;
use clap::{App, Arg};
use log::{info, trace, warn};
use rusqlite::{params, Connection, Result, Row};

#[derive(Debug)]
struct Ticket {
    id: i32,
    name: String,
}

#[derive(Debug)]
struct Note {
    id: i32,
    ticket_id: i32,
    content: String,
    date: String,
}

fn main() {
    trace!("Application intialization");
    let matches = App::new("Coppernotes")
        .version("0.1.0")
        .author("RayGervais")
        .about("Teaches about argument parsing")
        .arg(
            Arg::with_name("ticket")
                .short("t")
                .long("ticket")
                .required(true)
                .takes_value(true)
                .help("Related Kanban ticket"),
        )
        .arg(
            Arg::with_name("note")
                .short("n")
                .long("note")
                .takes_value(true)
                .required(false)
                .help("Note to add for the ticket provided"),
        )
        .get_matches();

    // Initialize Application
    let mut conn = Connection::open("tickets.db").expect("db connection failed");

    initialize_db(&mut conn).expect("failed to initialize the applications database");

    let ticket = matches.value_of("ticket").unwrap();
    match create_new_ticket(&mut conn, ticket.to_string()) {
        Ok(_) => info!("created new entry for {}", ticket),
        Err(e) => warn!("error creating {}: {}", ticket, e),
    }

    let ticket = get_ticket(&mut conn, ticket.to_string()).expect("Failed to deserialize row");

    if let Some(content) = matches.value_of("note") {
        match create_new_note(&mut conn, ticket.id, content) {
            Ok(_) => info!("Successfully created new note"),
            Err(_) => warn!("Failed to create new note"),
        }
    }

    let notes = get_notes_from_ticket(&mut conn, ticket.id);
    for note in notes {
        println!("{}", note.content);
    }
}

// Create the database schemas
fn initialize_db(conn: &mut rusqlite::Connection) -> Result<()> {
    let tx = conn.transaction().unwrap();

    tx.execute(
        "CREATE TABLE IF NOT EXISTS TICKETS (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    tx.execute(
        "CREATE TABLE IF NOT EXISTS NOTES (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            ticket_id   INTEGER NOT NULL,
            content     TEXT NOT NULL UNIQUE,
            date        TEXT NOT NULL,
            FOREIGN KEY (ticket_id) REFERENCES TICKETS(row_id)

        )",
        [],
    )?;

    tx.commit()
}

fn create_new_ticket(conn: &mut rusqlite::Connection, ticket: String) -> Result<usize> {
    conn.execute("INSERT INTO TICKETS (name) VALUES (?1)", [ticket])
}

fn create_new_note(
    conn: &mut rusqlite::Connection,
    ticket_id: i32,
    content: &str,
) -> Result<usize> {
    let date = Utc::now();

    conn.execute(
        "INSERT INTO NOTES (ticket_id, date, content) VALUES (?1, ?2, ?3)",
        params![ticket_id, date.timestamp(), content],
    )
}

fn get_ticket(conn: &mut rusqlite::Connection, ticket: String) -> Result<Ticket, rusqlite::Error> {
    info!("Querying database for ticket {}", ticket);

    conn.prepare("SELECT id, name FROM TICKETS WHERE name = ?;")
        .expect("Failed to create query string")
        .query_row([ticket], |row| {
            Ok(Ticket {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
            })
        })
}

fn get_notes_from_ticket(conn: &mut rusqlite::Connection, ticket_id: i32) -> Vec<Note> {
    info!(
        "Querying database for notes relating to ticket_id {}",
        ticket_id
    );

    conn.prepare("SELECT id, ticket_id, date, content FROM NOTES WHERE ticket_id = ?;")
        .expect("Failed to create query string")
        .query_map([ticket_id], |row: &Row| {
            Ok(Note {
                id: row.get(0).unwrap(),
                ticket_id: row.get(1).unwrap(),
                date: row.get(2).unwrap(),
                content: row.get(3).unwrap(),
            })
        })
        .expect("Failed to deserialize rows")
        .flatten()
        .collect()
}
