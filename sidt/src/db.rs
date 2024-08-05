// Set up Sqlite database if not already configured

pub mod db {

    use sqlite::Statement;



pub fn create_entry_table()  {

    let connection = sqlite::open("journal.db").unwrap();

    /* Notes on Schema
    date - TEXT - Formatted date string for the entry date. Acts as Primary Key.
    entry - TEXT - The user's entry for the given day.
    entry_date - INTEGER - When the first entry for the date was created. Unix timestamp value (SQLite doesn't support date/datetime types)
    last_updated - INTEGER - The date/time of last update to this record. Unix timestamp value. 

    */
     
    let query = "
        CREATE TABLE entries (date TEXT, entry TEXT, entry_date INTEGER, last_updated INTEGER);
    ";
    let _ = connection.execute(query).unwrap();
}

pub fn write_entry(date: String, entry: String, entry_date: i64, last_updated: i64) {

    let connection = sqlite::open("journal.db").unwrap();

    let query = format!("
        INSERT INTO entries VALUES ('{date}','{entry}','{entry_date}','{last_updated}');
    ");

    let result = connection.execute(query);

    // Handle failure to write to database due to it not existing
    // TODO: Re-write this to handle more specific error instances.

    match result {
        Ok(_) => (),
        Err(_) => create_entry_table()
    }
}


pub fn read_entry() {
    let connection = sqlite::open("journal.db").unwrap();

    let query = format!("
        SELECT * FROM entries;
    ");

    let mut result = connection.prepare(query).unwrap();

    use sqlite::State;

    //result.bind((1, 50)).unwrap();

    while let Ok(State::Row) = result.next() {
        let date = result.read::<String, _>("date").unwrap();

        let entry = result.read::<String, _>("entry").unwrap();

        let entry_date = result.read::<String, _>("entry_date").unwrap();

        let last_updated = result.read::<String, _>("last_updated").unwrap();

        println!("{} {} {} {}", date, entry, entry_date, last_updated);
    }
}

pub fn read_selected_entries(rows: usize) {
    let connection = sqlite::open("journal.db").unwrap();

    let query = format!("
        SELECT * FROM entries ORDER BY entry_date DESC LIMIT {rows};
    ");

    println!("Query string:{}",query);

    let mut result = connection.prepare(query).unwrap();

    use sqlite::State;

    //result.bind((1, 50)).unwrap();

    while let Ok(State::Row) = result.next() {
        let date = result.read::<String, _>("date").unwrap();

        let entry = result.read::<String, _>("entry").unwrap();

        let entry_date = result.read::<String, _>("entry_date").unwrap();

        let last_updated = result.read::<String, _>("last_updated").unwrap();

        println!("{} {} {} {}", date, entry, entry_date, last_updated);
    }
}

pub fn read_all_entries() {
    let connection = sqlite::open("journal.db").unwrap();

    let query = format!("
        SELECT * FROM entries ORDER BY entry_date DESC;
    ");

    let mut result = connection.prepare(query).unwrap();

    use sqlite::State;

    //result.bind((1, 50)).unwrap();

    while let Ok(State::Row) = result.next() {
        let date = result.read::<String, _>("date").unwrap();

        let entry = result.read::<String, _>("entry").unwrap();

        let entry_date = result.read::<String, _>("entry_date").unwrap();

        let last_updated = result.read::<String, _>("last_updated").unwrap();

        println!("{} {} {} {}", date, entry, entry_date, last_updated);
    }
}

}

