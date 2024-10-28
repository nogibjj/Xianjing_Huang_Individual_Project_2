//this will be the CLI portion of the project where we accept
//user defined arguments and call lib.rs logic to handle them
use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use sqlite::{
    create_exec, create_table, delete_exec, drop_table, extract, load_data_from_csv, read_exec,
    update_exec,
};

//Here we define a struct (or object) to hold our CLI arguments
//for #[STUFF HERE] syntax, these are called attributes.
//for now, they define behavior for elements in rust.

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
//Think of a struct as a class which makes objects in python
//This is designed to generate an object out of the CLI inputs
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

//An enum is a type in rust that can have multiple exauhstive and mutually exclusive options
//Extract, Create, Load, Query, Drop, Insert, Update, Delete

#[derive(Debug, Subcommand)]
//By separating out the commands as enum types we can easily match what the user is
//trying to do in main
enum Commands {
    ///Extract a url to a file path
    /// "sqlite -e"
    #[command(alias = "e", short_flag = 'e')]
    Extract {},
    ///Pass a table name to create a table
    /// "sqlite -c table1"
    #[command(alias = "c", short_flag = 'c')]
    Create { table_name: String },
    ///Pass a table name and a file path to load data from csv
    /// "sqlite -l table1 data/customer_new.csv"
    #[command(alias = "l", short_flag = 'l')]
    Load {
        table_name: String,
        file_path: String,
    },
    ///Pass a table name to read data
    /// "sqlite -q table1"
    #[command(alias = "q", short_flag = 'q')]
    Query { table_name: String },
    ///Pass a table name to drop
    /// "sqlite -d table1"
    #[command(alias = "d", short_flag = 'd')]
    Drop { table_name: String },
    ///Pass a new record to insert
    /// "sqlite -i table1 11 Remi female Durham"
    #[command(alias = "i", short_flag = 'i')]
    Insert {
        table_name: String,
        id: i32,
        name: String,
        gender: String,
        city: String,
    },
    ///Pass a new record to update
    /// "sqlite -u table1 11 Remi female 'Los Angeles'"
    #[command(alias = "u", short_flag = 'u')]
    Update {
        table_name: String,
        id: i32,
        new_name: Option<String>,
        new_gender: Option<String>,
        new_city: Option<String>,
    },
    ///Delete a record by id
    /// "sqlite -x table1 11"
    #[command(alias = "x", short_flag = 'x')]
    Delete { table_name: String, id: i32 },
}

fn main() -> Result<()> {
    //Here we parse the CLI arguments and store them in the args object
    let args = Cli::parse();
    //generate connection
    let conn = Connection::open("my_database.db")?;

    //Here we can match the behavior on the subcommand and call our lib logic
    match args.command {
        Commands::Extract {} => {
            println!("Extract a url to a file path");
            extract().expect("Failed to extract");
        }
        Commands::Create { table_name } => {
            println!("Creating Table {}", table_name);
            create_table(&conn, &table_name).expect("Failed to create table");
        }
        Commands::Query { table_name } => {
            println!("Read Table: {}", table_name);
            read_exec(&conn, &table_name).expect("Failed to execute query");
        }
        Commands::Drop { table_name } => {
            println!("Deleting: {}", table_name);
            drop_table(&conn, &table_name).expect("Failed to drop table");
        }
        Commands::Load {
            table_name,
            file_path,
        } => {
            println!(
                "Loading data into table '{}' from '{}'",
                table_name, file_path
            );
            load_data_from_csv(&conn, &table_name, &file_path)
                .expect("Failed to load data from csv");
        }
        Commands::Insert {
            table_name,
            id,
            name,
            gender,
            city,
        } => {
            println!(
                "Insert record in table '{}' with ID {}, name {:?}, gender {:?}, city {:?}",
                table_name, id, name, gender, city
            );
            create_exec(&conn, &table_name, id, &name, &gender, &city)
                .expect("Failed to insert record");
        }
        Commands::Update {
            table_name,
            id,
            new_name,
            new_gender,
            new_city,
        } => {
            println!("Updating record in table '{}' with ID {}", table_name, id);
            update_exec(
                &conn,
                &table_name,
                id,
                new_name.as_deref(),
                new_gender.as_deref(),
                new_city.as_deref(),
            )
            .expect("Failed to update record");
        }
        Commands::Delete { table_name, id } => {
            println!("Delete record in table '{}' with ID {}", table_name, id);
            delete_exec(&conn, &table_name, id).expect("Failed to delete record");
        }
    }
    Ok(())
}
