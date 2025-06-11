// Set up Sqlite database if not already configured
pub mod db {
    use std::env;

    pub fn create_entry_table()  {

    let connection = sqlite::open("../journal.db").unwrap();

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
    let path = env::home_dir().unwrap();
    println!("Home dir: {:?}", path);
    let connection = sqlite::open("../journal.db").unwrap();

    // Read db to see if there is an existing entry

    // If existing entry, simply append to the original entry

    let query = format!{"SELECT * from entries where date == '{}';", date};

    let result = connection.prepare(query);
    match result {
        Ok(_) => (),
        Err(_) => create_entry_table()
    }

    println!("Checking records for date: {}",date);
    let any: Vec<Result<sqlite::Row, sqlite::Error>> = result.unwrap().iter().collect();

    println!("Records for date: {}",date);


    if any.len() >= 1 {
        let update_statement = format!("
        UPDATE entries SET entry = entry || ' ' || '{entry}' WHERE date == '{}';
        UPDATE entries SET last_updated = '{last_updated}' WHERE date == '{}';
        ", date, date);

        let _ = connection.execute(update_statement);

    }


    else {
        let insert_statement = format!("
        INSERT INTO entries VALUES ('{date}','{entry}','{entry_date}','{last_updated}');    
        ");
    
        let _ = connection.execute(insert_statement);
    
        // Handle failure to write to database due to it not existing
        // TODO: Re-write this to handle more specific error instances.

    }
    


}

pub fn update_entry(date: String, entry: String, last_updated: i64) {

    let connection = sqlite::open("../journal.db").unwrap();
    let query = format!("
        UPDATE entries SET entry = '{entry}',last_updated = '{last_updated}'  WHERE date == '{date}';
    ");

    let _ = connection.execute(query);

}

pub fn read_last_entry() {
   read_selected_entries(1);
}

pub fn read_selected_entries(rows: usize) -> (){
    let connection = sqlite::open("../journal.db").unwrap();

    let query = format!("
        SELECT * FROM entries ORDER BY entry_date DESC LIMIT {rows};
    ");

    let mut result = connection.prepare(query).unwrap();

    while let Ok(sqlite::State::Row) = result.next() {
        let date = result.read::<String, _>("date").unwrap();

        let entry = result.read::<String, _>("entry").unwrap();

       // let entry_date = result.read::<String, _>("entry_date").unwrap();

       // let last_updated = result.read::<String, _>("last_updated").unwrap();

        println!("{} {}", date, entry);
    }
}

pub fn read_entry(date: Option<String>) -> Result<String, String> {
    match date {
        Some(date) => {
            let connection = sqlite::open("../journal.db").unwrap();
            let query: String = format!("
            SELECT entry FROM entries WHERE date == '{}';", date);
            let result = connection.prepare(query);
            if result.is_ok(){
                let mut data = result.unwrap();
                println!("Column names: {:?}", data.column_names().to_vec());
                if let Ok(sqlite::State::Row) = data.next(){
                    Ok(data.read::<String, _>("entry").unwrap())
                }
                else {
                    Err(String::from("Entry not found"))
                }

            }
            else {
                Err(String::from("Entry not found"))
            }
        },
        None => {
            Err("No date specified.").expect("TODO: panic message")
        }
    }

}
    pub fn read_all_entries() {
    let connection = sqlite::open("../journal.db").unwrap();

    let query = "SELECT * FROM entries ORDER BY entry_date DESC;";

    let result = connection.prepare(query);

    if result.is_ok(){
        let mut data = result.unwrap();

        while let Ok(sqlite::State::Row) = data.next() {
            let date = data.read::<String, _>("date").unwrap();
    
            let entry = data.read::<String, _>("entry").unwrap();
    
           // let entry_date = result.read::<String, _>("entry_date").unwrap();
    
            //let last_updated = result.read::<String, _>("last_updated").unwrap();
    
            println!("{} {}", date, entry);
        }
    }

    
}

pub fn delete_selected_entry(date: String){
    let connection = sqlite::open("../journal.db").unwrap();

    let query = format!("
    DELETE * FROM entries WHERE date == '{}';
    ", date);

    connection.execute(query).unwrap();

}

// pub fn delete_date_range()

pub fn get_search_results(search_phrase: &String) {
    let connection: sqlite::Connection = sqlite::open("../journal.db").unwrap();

    let query = format!("
    SELECT date, entry FROM entries WHERE entry LIKE '%{}%';
    ", search_phrase);

    let result = connection.prepare(query);


    if result.is_ok(){
        let mut data = result.unwrap();

        while let Ok(sqlite::State::Row) = data.next() {
            let date = data.read::<String, _>("date").unwrap();
    
            let entry = data.read::<String, _>("entry").unwrap();
    
            println!("{} {}", date, entry);
        }
    }

}

pub fn change_date(old_date: &String, new_date: &String){
    let connection: sqlite::Connection = sqlite::open("../journal.db").unwrap();

    let query: String = format!("
    UPDATE entries SET date = '{new_date}' WHERE date == '{old_date}'
    ");

    connection.execute(query).unwrap();

}

}

