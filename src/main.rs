use std::env;
use std::io;
use rusqlite::{Connection, NO_PARAMS};

fn main() {
    // Change to the home directory
    let home_dir = env::var("HOME").unwrap();
    env::set_current_dir(home_dir).unwrap();

    // Prompt the user for the path to the places file
    let mut places_path = String::new();
    println!("Enter the path to the places file, or press enter to search for it automatically:");
    io::stdin().read_line(&mut places_path).unwrap();
    places_path = places_path.trim().to_string();

    // If the user didn't enter a path, search for the places file in the default Firefox profile directory
    if places_path.is_empty() {
        let mut firefox_profile = String::new();
        let firefox_profiles_dir = format!("{}/.mozilla/firefox", env::var("HOME").unwrap());
        for entry in std::fs::read_dir(firefox_profiles_dir).unwrap() {
            if let Ok(entry) = entry {
                if entry.file_type().unwrap().is_dir() {
                    let entry_path = entry.path();
                    if entry_path.to_str().unwrap().contains(".default") {
                        firefox_profile = entry_path.to_str().unwrap().to_string();
                        break;
                    }
                }
            }
        }
        places_path = format!("{}/places.sqlite", firefox_profile);
    }

    // Open the database connection
    let conn = Connection::open(places_path).unwrap();

    // Get a list of all the tables in the database
    let tables: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .unwrap()
        .query_map(NO_PARAMS, |row| row.get(0))
        .unwrap()
        .map(|result| result.unwrap())
        .collect();

    // Set the output mode to XML
    println!(".mode xml");

    // Loop over each table and output its contents to an XML file
    for table in tables {
        let xml_file = format!("{}.xml", table);
        println!(".output {}", xml_file);
        let query = format!("SELECT * FROM {}", table);
        conn.execute(query.as_str(), NO_PARAMS).unwrap();
    }
}
