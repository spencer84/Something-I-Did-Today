// Set up Sqlite database if not already configured
use colored::Colorize;
use sqlite::Connection;
use std::fs::create_dir;
use crate::settings::read_settings;

fn get_connection() -> Result<Connection, sqlite::Error> {
    let db_path = read_settings().home_dir + "/.sidt/journal.db";
    let connection_result = sqlite::open(db_path);

    match connection_result {
        Ok(connection) => Ok(connection),
        Err(_) => {
            create_entry_table();
            let subsequent_connection = get_connection();
            subsequent_connection
        }
    }
}
pub fn create_entry_table() {
    let settings = read_settings();
    let db_dir = settings.home_dir + "/.sidt";
    let _create_sidt_dir_result = create_dir(&db_dir);
    let connection = sqlite::open(db_dir + "/journal.db").unwrap();

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
    let connection = get_connection().unwrap();
    // Read db to see if there is an existing entry

    // If existing entry, simply append to the original entry

    let query = format! {"SELECT * from entries where date == '{}';", date};

    let result = connection.prepare(query);
    match result {
        Ok(_) => (),
        Err(_) => create_entry_table(),
    }
    let any: Vec<Result<sqlite::Row, sqlite::Error>> = result.unwrap().iter().collect();

    if any.len() >= 1 {
        let update_statement = format!(
            "
    UPDATE entries SET entry = entry || ' ' || '{entry}' WHERE date == '{}';
    UPDATE entries SET last_updated = '{last_updated}' WHERE date == '{}';
    ",
            date, date
        );

        let _ = connection.execute(update_statement);
    } else {
        let insert_statement = format!(
            "
    INSERT INTO entries VALUES ('{date}','{entry}','{entry_date}','{last_updated}');
    "
        );

        let _ = connection.execute(insert_statement);

        // Handle failure to write to database due to it not existing
        // TODO: Re-write this to handle more specific error instances.
    }
}

pub fn update_entry(date: String, entry: String, last_updated: i64) {
    let connection = get_connection().unwrap();
    let query = format!("
    UPDATE entries SET entry = '{entry}',last_updated = '{last_updated}'  WHERE date == '{date}';
");

    let _ = connection.execute(query);
}

pub fn read_last_entry() {
    read_selected_entries(1);
}

pub fn read_selected_entries(rows: usize) -> () {
    let connection = get_connection().unwrap();

    let query = format!(
        "
    SELECT * FROM entries ORDER BY entry_date DESC LIMIT {rows};
"
    );

    let mut result = connection.prepare(query).unwrap();

    while let Ok(sqlite::State::Row) = result.next() {
        let date = result.read::<String, _>("date").unwrap();

        let entry = result.read::<String, _>("entry").unwrap();

        // let entry_date = result.read::<String, _>("entry_date").unwrap();

        // let last_updated = result.read::<String, _>("last_updated").unwrap();

        println!("{} {}", date.blue().bold(), entry);
    }
}

pub fn read_entry(date: Option<String>) -> Result<String, String> {
    match date {
        Some(date) => {
            let connection = get_connection().unwrap();
            let query: String = format!(
                "
        SELECT entry FROM entries WHERE date == '{}';",
                date
            );
            let result = connection.prepare(query);
            if result.is_ok() {
                let mut data = result.unwrap();
                if let Ok(sqlite::State::Row) = data.next() {
                    Ok(data.read::<String, _>("entry").unwrap())
                } else {
                    Err(String::from("Entry not found"))
                }
            } else {
                Err(String::from("Entry not found"))
            }
        }
        None => Err("No date specified.").expect("TODO: panic message"),
    }
}
pub fn read_all_entries() {
    let connection = get_connection().unwrap();

    let query = "SELECT * FROM entries ORDER BY entry_date DESC;";

    let result = connection.prepare(query);

    if result.is_ok() {
        let mut data = result.unwrap();

        while let Ok(sqlite::State::Row) = data.next() {
            let date = data.read::<String, _>("date").unwrap();

            let entry = data.read::<String, _>("entry").unwrap();

            // let entry_date = result.read::<String, _>("entry_date").unwrap();

            //let last_updated = result.read::<String, _>("last_updated").unwrap();
            // TODO: Provide ability to customise colour choice
            println!("{} {}", date.blue().bold(), entry);
        }
    }
}

pub fn delete_selected_entry(date: String) {
    let connection = get_connection().unwrap();

    let query = format!(
        "
DELETE FROM entries WHERE date == '{}';
",
        date
    );

    connection.execute(query).unwrap();
}

// pub fn delete_date_range()

pub fn get_search_results(search_phrase: &String) {
    let connection = get_connection().unwrap();

    let query = format!(
        "
SELECT date, entry FROM entries WHERE entry LIKE '%{}%';
",
        search_phrase
    );

    let result = connection.prepare(query);

    if result.is_ok() {
        let mut data = result.unwrap();

        while let Ok(sqlite::State::Row) = data.next() {
            let date = data.read::<String, _>("date").unwrap();

            let entry = data.read::<String, _>("entry").unwrap();

            println!("{} {}", date, entry);
        }
    }
}

pub fn change_date(old_date: &String, new_date: &String) {
    let connection = get_connection().unwrap();

    let query: String = format!(
        "
UPDATE entries SET date = '{new_date}' WHERE date == '{old_date}'
"
    );

    connection.execute(query).unwrap();
}

fn create_tag_tables() {
    let connection = get_connection().unwrap();

    // Similar to entry table; log of all tag usage and content
    let query_tag_content = "\
    CREATE TABLE tag_content\
    (date TEXT, tag TEXT, tag_content TEXT, entry_date INTEGER, last_updated INTEGER);";
    connection.execute(query_tag_content).unwrap();

    // Summary table about each tag
    let query_tags = "\
    CREATE TABLE tags\
    (tag TEXT, long_form_tag TEXT, short_form_tag TEXT, description TEXT);";
    connection.execute(query_tags).unwrap();
}

pub fn create_tag(tag: &String) {
    let connection = get_connection().unwrap();
    let query = format!("INSERT INTO tags VALUES ('{}','','','');", tag);
    let result = connection.execute(query);
    if result.is_err() {
        let error_message = result.err().unwrap();
        if error_message.to_string() == "no such table: tags" {
            create_tag_tables();
        }
    }
}
pub fn write_tag(
    date: String,
    tag: &String,
    tag_content: &String,
    entry_date: i64,
    last_updated: i64,
) {
    let connection = get_connection().unwrap();

    let query = format!("INSERT INTO tag_content VALUES ('{date}','{tag}','{tag_content}','{entry_date}','{last_updated}');");
    connection.execute(query).unwrap();
}

pub fn read_selected_tags(tag: &String, number: usize) -> () {
    let connection = get_connection().unwrap();
    let query: String = format!(
        "SELECT date, tag_content FROM tag_content WHERE tag == '{tag}' ORDER BY entry_date DESC LIMIT {number};",
    );
    let mut result = connection.prepare(query).unwrap();

    while let Ok(sqlite::State::Row) = result.next() {
        let date = result.read::<String, _>("date").unwrap();

        let tag_content = result.read::<String, _>("tag_content").unwrap();

        println!("{} {}", date.blue().bold(), tag_content);
    }
}

pub fn get_tags() -> Vec<String> {
    let connection = get_connection().unwrap();

    let query = "SELECT tag,short_form_tag FROM tags";
    let mut result_raw = connection.prepare(query);
    let mut result = match result_raw {
        Ok(result) => result,
        Err(err) => {
            create_tag_tables();
            return get_tags();
        }
    };

    let mut short_form: Vec<String> = Vec::new();
    let mut tags: Vec<String> = Vec::new();
    while let Ok(sqlite::State::Row) = result.next() {
        short_form.push(result.read::<String, _>("short_form_tag").unwrap());
        tags.push(result.read::<String, _>("tag").unwrap());
    }
    let all_tags = vec![short_form, tags].concat();
    all_tags
}

