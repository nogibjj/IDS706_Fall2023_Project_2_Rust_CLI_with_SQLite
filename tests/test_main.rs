use rusqlite::{params, Connection, Result};

#[test]
fn test_main() -> Result<()> {
    // Connect to the database
    let conn = Connection::open("my_database.db")?;

    // Drop the people table if it exists
    conn.execute("DROP TABLE IF EXISTS people", [])?;

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

    let mut stmt = conn.prepare("SELECT id, name, age FROM people")?;

    let rows: Vec<(i32, String, i32)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
        .map(|r| r.unwrap())
        .collect();
    assert_eq!(rows.len(), 2);
    Ok(())
}
