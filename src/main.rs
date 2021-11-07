extern crate pricing_daemon;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(pricing_daemon::sync::run()) {
        Ok(_) => println!("Done"),
        Err(e) => eprintln!("An error ocurred: {}", e),
    };
}
