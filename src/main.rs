mod sense;
mod platform;


fn main() {

match platform::linux::arp::read_arp_table(){
    Ok(content)=>println!("{content}"),
    Err(error)=>eprintln!("Failed to read ARP table : {error}")
}


}
