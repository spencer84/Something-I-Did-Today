// Set up Sqlite database if not already configured

fn create_entry_table() {

    let connection = sqlite::open(":memory:").unwrap();

    let query = "
        CREATE TABLE entries (date DATE, entry TEXT, entry_date DATETIME, last_updated DATETIME);
    ";
    connection.execute(query).unwrap();

}

