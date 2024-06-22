mod libp2p;
mod http_proxy;
mod war_conversion;
mod config;

use std::env;
use crate::config::Config;

#[tokio::main]
async fn main() {
    // Load configuration
    let config = Config::load().expect("Failed to load configuration");

    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [options]", args[0]);
        return;
    }

    match args[1].as_str() {
        "start" => {
            // Start the node/client/proxy
            start_node(config).await;
        }
        "config" => {
            // Display or modify configuration
            Config::handle_config_command(&args[2..]);
        }
        "announce" => {
            if args.len() < 3 {
                eprintln!("Usage: {} announce <content_id>", args[0]);
                return;
            }
            libp2p::announce_content(&args[2]).await;  // Call the announce_content function
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}

async fn start_node(config: Config) {
    // Initialize libp2p, HTTP proxy, and other components
    libp2p::start_network(&config).await;
    http_proxy::start_proxy(&config).await;
    // Further initialization as needed
}
