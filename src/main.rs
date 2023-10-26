use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // Connect to the database
    let conn = Connection::open("my_database.db")?;

    // Create a new table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS people (
            id INTEGER PRIMARY KEY,
            name TEXT,
            age INTEGER
         )",
        [],
    )?;

    // Insert two records
    conn.execute(
        "INSERT INTO people (name, age) VALUES (?, ?)",
        params!["Alice", 30],
    )?;
    conn.execute(
        "INSERT INTO people (name, age) VALUES (?, ?)",
        params!["Bob", 25],
    )?;

    // Update Alice's age
    conn.execute(
        "UPDATE people SET age = ? WHERE name = ?",
        params![31, "Alice"],
    )?;

    // Print all records
    let mut stmt = conn.prepare("SELECT id, name, age FROM people")?;
    let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;

    for row in rows {
        let (id, name, age): (i32, String, i32) = row?;
        println!("id = {}, name = {}, age = {}", id, name, age);
    }

    // Delete Bob's record
    conn.execute("DELETE FROM people WHERE name = ?", params!["Bob"])?;

    Ok(())
}
