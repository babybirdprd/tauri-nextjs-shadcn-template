// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::time::{SystemTime, UNIX_EPOCH};

mod mcp_client;
mod mcp_server;

#[tauri::command]
fn greet() -> String {
  let now = SystemTime::now();
  let epoch_ms = now.duration_since(UNIX_EPOCH).unwrap().as_millis();
  format!("Hello world from Rust! Current epoch: {}", epoch_ms)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // Initialize MCP modules (this is mostly to ensure code is compiled and linked)
  mcp_client::init_client();
  mcp_server::init_server();

  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
        greet, 
        mcp_client::main::start_mcp_client, 
        mcp_server::main::start_mcp_server
    ])
    .setup(|app| {
        // Example of starting the MCP server in a background thread.
        // The client is started on demand via its tauri command.
        // You might want to manage these differently based on your app's lifecycle.
        let app_handle = app.handle().clone();
        tokio::spawn(async move {
            println!("Attempting to start MCP server in background...");
            if let Err(e) = mcp_server::main::run_server().await {
                eprintln!("Failed to start MCP server in background: {}", e);
            } else {
                println!("MCP server background task completed (or server exited).");
            }
        });
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
