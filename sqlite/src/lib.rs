use csv::ReaderBuilder; //for loading from csv
use reqwest::blocking;
use rusqlite::ToSql;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::Path;

// Extract a url to a file path
pub fn extract() -> Result<(), Box<dyn Error>> {
    let url = "https://raw.githubusercontent.com/nogibjj/Xianjing_Huang_Mini_Proj_test/refs/heads/main/customer_new.csv";
    let directory = "data";
    let file_path = "data/customer_new.csv";

    // Create the directory if it doesn't exist
    if !Path::new(directory).exists() {
        create_dir_all(directory)?;
    }

    let response = blocking::get(url)?;
    let mut dest = File::create(file_path)?;
    let content = response.bytes()?;
    copy(&mut content.as_ref(), &mut dest)?;

    println!("File has been downloaded to {}", file_path);
    Ok(())
}

// Load data from a file path to a table
pub fn load_data_from_csv(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    //Box<dyn Error> is a trait object that can represent any error type
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let insert_query = format!(
        "INSERT INTO {} (id, name, gender, city) VALUES (?, ?, ?, ?)",
        table_name
    );

    for result in rdr.records() {
        let record = result?;
        let id: i32 = record[0].parse()?; //.parse() is a method that converts a string into a number
        let name: &str = &record[1];
        let gender: &str = &record[2];
        let city: &str = &record[3];

        conn.execute(&insert_query, params![id, name, gender, city])?;
    }

    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}

// Create a table
pub fn create_table(conn: &Connection, table_name: &str) -> Result<(), Box<dyn Error>> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            gender TEXT NOT NULL,
            city TEXT NOT NULL
        )",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(()) //returns nothing except an error if it occurs
}

// Drop a table
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<(), Box<dyn Error>> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

// Read records in table
pub fn read_exec(conn: &Connection, table_name: &str) -> Result<(), Box<dyn Error>> {
    // Prepare the query and iterate over the rows returned
    let query_string = format!("SELECT * FROM {}", table_name);
    let mut stmt = conn.prepare(&query_string)?;

    // Use query_map to handle multiple rows
    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let gender: String = row.get(2)?;
        let city: String = row.get(3)?;
        Ok((id, name, gender, city))
    })?;

    // Iterate over the rows and print the results
    for row in rows {
        let (id, name, gender, city) = row?;
        println!(
            "ID: {}, Name: {}, Gender: {}, City: {}",
            id, name, gender, city
        );
    }

    Ok(())
}

// Update a record in the table
pub fn update_exec(
    conn: &Connection,
    table_name: &str,
    id: i32,
    new_name: Option<&str>,
    new_gender: Option<&str>,
    new_city: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let mut updates = Vec::new();
    let mut params: Vec<Box<dyn ToSql>> = Vec::new(); // Vector to hold owned params

    if let Some(name) = new_name {
        updates.push("name = ?");
        params.push(Box::new(name.to_string())); // Convert to String and box it
    }
    if let Some(gender) = new_gender {
        updates.push("gender = ?");
        params.push(Box::new(gender.to_string())); // Convert to String and box it
    }
    if let Some(city) = new_city {
        updates.push("city = ?");
        params.push(Box::new(city.to_string())); // Convert to String and box it
    }

    if updates.is_empty() {
        println!("No fields to update for ID: {}", id);
        return Ok(());
    }

    // Append the ID at the end, since it's used in the WHERE clause.
    let update_query = format!(
        "UPDATE {} SET {} WHERE id = ?",
        table_name,
        updates.join(", ")
    );
    params.push(Box::new(id)); // Box the ID to match the param type

    // Execute the query with the params. Use params slice.
    conn.execute(
        &update_query,
        params
            .iter()
            .map(|b| &**b)
            .collect::<Vec<&dyn ToSql>>()
            .as_slice(),
    )?;

    println!(
        "Record with ID '{}' updated successfully in table '{}'.",
        id, table_name
    );
    Ok(())
}

// Insert a record in the table
pub fn create_exec(
    conn: &Connection,
    table_name: &str,
    id: i32,
    name: &str,
    gender: &str,
    city: &str,
) -> Result<(), Box<dyn Error>> {
    let insert_query = format!(
        "INSERT INTO {} (id, name, gender, city) VALUES (?, ?, ?, ?)",
        table_name
    );

    conn.execute(&insert_query, params![id, name, gender, city])?;
    println!(
        "Inserted person with ID '{}' into the '{}' table successfully!",
        id, table_name
    );
    Ok(())
}

// Delete a record in the table
pub fn delete_exec(conn: &Connection, table_name: &str, id: i32) -> Result<(), Box<dyn Error>> {
    let delete_query = format!("DELETE FROM {} WHERE id = ?", table_name);
    conn.execute(&delete_query, params![id])?;
    println!(
        "Deleted person with ID '{}' from the '{}' table successfully!",
        id, table_name
    );
    Ok(())
}
