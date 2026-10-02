mod platform;
mod sense;

use platform::linux::arp::{parse_arp_table, read_arp_table};

fn main() {
    match read_arp_table() {
        Ok(content) => {
            let observations = parse_arp_table(&content);

            for observation in observations {
                println!("{observation:#?}");
            }
        }
        Err(error) => {
            eprintln!("Failed to read ARP table: {error}");
        }
    }
}