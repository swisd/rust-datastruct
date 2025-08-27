use std::fmt;
use std::fmt::{Display, Formatter};
use rusqlite::{Connection, Result, params};
use chrono::{DateTime, Local};


#[derive(Debug)]
pub enum LedgerType {
    V,
    D,
    I,
    W,
    E,
    F,
}

impl Display for LedgerType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LedgerType::V => write!(f, "V"),
            LedgerType::D => write!(f, "D"),
            LedgerType::I => write!(f, "I"),
            LedgerType::W => write!(f, "W"),
            LedgerType::E => write!(f, "E"),
            LedgerType::F => write!(f, "F"),
        }
    }
}


pub enum PIDSet {
    Lower,
    Upper,
}

pub enum Ledger {
    Date {
        date: String,
    },
    Time {
        time: String,
    },
    PID0 {
        pid0: i16,
    },
    PID1 {
        pid1: i16,
    },
    Tag {
        tag: String,
    },
    Path {
        path: String,
    },
    Cls {
        cls: LedgerType,
    },
    Msg {
        msg: String,
    }
}


// Config operations

pub fn init_db() -> Result<()> { // create the database for the ledgers
    let db_path = "records.db"; // Specify the desired database file name
    let conn = Connection::open(db_path)?; // Open or create the database file

    conn.execute(
        "CREATE TABLE IF NOT EXISTS rcd (
                id INTEGER PRIMARY KEY,
                date TEXT NOT NULL,
                time TEXT NOT NULL,
                pid0 TEXT NOT NULL,
                pid1 TEXT NOT NULL,
                tag TEXT,
                path TEXT NOT NULL,
                cls TEXT NOT NULL,
                msg TEXT NOT NULL
            )",
        /*
            id      id
            date    mm/dd/yyyy
            time    hh:mm:ss (24hr)
            pid0    process id (lower)
            pid1    process id (upper)
            tag     tag (group)
            path    path to origination
            cls     class/ledger type (V/D/I/W/E)
        */


        (), // Empty tuple for parameters
    )?;
    println!("SQLite database created and table 'rcd' ensured.");
    Ok(())
}

pub fn connect_db(name: &str) -> Result<Connection> {
    if name.is_empty() {
        let name: &str = "records.db";
    }
    let db_path = name;
    let conn = Connection::open(db_path)?;
    Ok(conn)
}

pub fn close_db(conn: Connection) -> Result<()> {
    let _ = conn.close();
    Ok(())
}

pub fn drop_db(conn: Connection) -> Result<()> {
    let _ = conn.execute("DROP TABLE rcd", []);
    Ok(())
}

// Set operations


pub fn ledger(conn: Connection, pid0: i16, pid1: i16, tag: &str, path: &str, cls: LedgerType, msg: &str) -> Result<()> { // creata a ledger in the database
    // Get date, time, and make an id
    let current_local: DateTime<Local> = Local::now();
    let date = current_local.format("%Y-%m-%d").to_string();
    let time = current_local.format("%H:%M:%S").to_string();

    // Use sequential generating for an id
    let mut stmt = conn.prepare("SELECT id FROM rcd ORDER BY id DESC LIMIT 1")?;
    let mut rows = stmt.query([])?;
    let mut id = 0;
    if let Ok(Some(row)) = rows.next() {
        id = row.get(0)?;
    }
    id += 1;

    // Create the ledger inside the database
    let mut stmt = conn.prepare("INSERT INTO rcd (id, date, time, pid0, pid1, tag, path, cls, msg) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")?;

    // Execute the ledger
    stmt.execute(params![id, date, time, pid0.to_string(), pid1.to_string(), tag, path, cls.to_string(), msg])?;

    // Return the ledger
    println!("Ledger created. {} {} {} {} {} {} {} {:?} {}", id, date, time, pid0.to_string(), pid1.to_string(), tag, path, cls, msg);

    Ok(())
}



// Get operations


pub fn get_ledger_all(conn: Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM rcd")?;
    stmt.execute([])?;
    Ok(())
}

pub fn get_ledger(conn: Connection, id: i16) -> Result<()> {
    // First query and process results
    {
        let mut stmt = conn.prepare("SELECT * FROM rcd WHERE id = ?")?;
        let mut rows = stmt.query(params![id])?;
        let row = rows.next()?;
        // Process row here if needed
    }

    // Execute second operation with new statement
    let mut stmt = conn.prepare("SELECT * FROM rcd WHERE id = ?")?;
    stmt.execute(params![id])?;
    Ok(())
}

pub fn get_ledger_by_date(conn: Connection, date: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM rcd WHERE date = ?")?;
    stmt.execute(params![date])?;
    Ok(())
}

pub fn get_ledger_by_tag(conn: Connection, tag: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM rcd WHERE tag = ?")?;
    stmt.execute(params![tag])?;
    Ok(())
}

pub fn get_ledger_by_path(conn: Connection, path: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM rcd WHERE path = ?")?;
    stmt.execute(params![path])?;
    Ok(())
}

pub fn get_ledger_by_cls(conn: Connection, cls: char) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM rcd WHERE cls = ?")?;
    stmt.execute(params![cls.to_string()])?;
    Ok(())
}