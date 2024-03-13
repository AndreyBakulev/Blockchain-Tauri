// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//thisd looks for blockchain in the directory
mod blockchain;
mod block;

use blockchain::Blockchain;
use block::Block;
fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  println!("Hello Tauri!");
  let blockchain = Blockchain::new();   
}
