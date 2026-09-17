mod config;
mod wizard;
mod port;
mod terminal;

use wizard::SerialWizard;
use port::PortManager;
use terminal::SerialTerminal;

fn main() {
    let config = SerialWizard::run();

    println!("Attempting to connect to {} at {}...", config.port_name, config.baud_rate);
    
    match PortManager::initialize(&config) {
        Ok(port) => {
            println!("Connection established successfully!\n");
            SerialTerminal::start(config, port);
        }
        Err(err) => {
            eprintln!("Critical Hardware Error: Could not open target port: {}", err);
            std::process::exit(1);
        }
    }
}
