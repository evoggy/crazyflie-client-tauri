// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::{thread, time};

use crazyflie_lib::Crazyflie;
use futures::StreamExt;
use tauri::Manager;
use tokio::sync::mpsc;

use ts_rs::TS;

#[tauri::command]
async fn scan(address: &str) -> Result<Vec<String>, String> {
    println!("Scanning for Crazyflies on address {}", address);
    let link_context = crazyflie_link::LinkContext::new(async_executors::AsyncStd);
    let uris = link_context
        .scan([0xE7; 5])
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(uris)
}

#[tauri::command]
async fn connect(uri: &str, state: tauri::State<'_, BackendState>) -> Result<(), String> {
    println!("Connect to Crazyflie on {}", uri);
    state
        .cmd
        .send(CrazyflieBackendCommand::Connect(uri.to_string()))
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[tauri::command]
async fn disconnect(state: tauri::State<'_, BackendState>) -> Result<(), String> {
    state
        .cmd
        .send(CrazyflieBackendCommand::Disconnect)
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[tauri::command]
async fn start_scan(address: &str, state: tauri::State<'_, BackendState>) -> Result<(), String> {
    println!("Start Crazyflies on address {}", address);
    state
        .cmd
        .send(CrazyflieBackendCommand::StartScan(address.to_string()))
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[tauri::command]
async fn stop_scan(state: tauri::State<'_, BackendState>) -> Result<(), String> {
    println!("Stop scanning");
    state
        .cmd
        .send(CrazyflieBackendCommand::StopScan)
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

struct BackendState {
    cmd: mpsc::Sender<CrazyflieBackendCommand>,
}

impl BackendState {
    fn new(ui_to_cf_tx: mpsc::Sender<CrazyflieBackendCommand>) -> Self {
        BackendState { cmd: ui_to_cf_tx }
    }
}

#[derive(Debug, Clone)]
enum CrazyflieBackendCommand {
    StartScan(String),
    StopScan,
    Connect(String),
    Disconnect,
}

enum CrazyflieBackendState {
    Idle,
    Scanning,
    Connecting,
    Connected,
}

#[derive(Debug, serde::Serialize, TS)]
#[ts(export, export_to = "../src/backend-interface.ts")]
struct ScanResponse {
    uris: Vec<String>,
    err: Option<String>,
}

#[derive(Debug, serde::Serialize, TS)]
#[ts(export, export_to = "../src/backend-interface.ts")]
struct ConnectedEvent {
    connected: bool,
    uri: String,
    err: Option<String>,
}

async fn crazyflie_backend_scan(
    mut ui_to_cf_rx: mpsc::Receiver<CrazyflieBackendCommand>,
    address: String,
    t: tauri::AppHandle,
) -> mpsc::Receiver<CrazyflieBackendCommand> {
    let link_context = crazyflie_link::LinkContext::new(async_executors::AsyncStd);
    //let mut ui_to_cf_rx_inner = ui_to_cf_rx.unwrap();
    loop {
        println!("Looping around to scan again!");
        tokio::select! {
          found = link_context.scan([0xE7; 5]) => {
              println!("Scanning for Crazyflies on address {}", address);
              match found {
                  Ok(uris) => {
                      let response = ScanResponse { uris, err: None };
                      t.emit_all("scan", &response).unwrap();
                  }
                  Err(e) => {
                      println!("Error scanning for Crazyflies: {:?}", e);
                      let response = ScanResponse { uris: vec![], err: Some(e.to_string()) };
                      t.emit_all("scan", &response).unwrap();
                      break;
                  }
              }
          }
          Some(input) = ui_to_cf_rx.recv() => {
              match input {
                  CrazyflieBackendCommand::StopScan => {
                      break;
                  }
                  _ => {
                      println!("Error: Received CF command: {:?} in scan state!", input);
                  }
              }
          }
        }
    }
    println!("Exiting scan mode!");

    ui_to_cf_rx
}

async fn crazyflie_backend_connected(
    mut ui_to_cf_rx: mpsc::Receiver<CrazyflieBackendCommand>,
    uri: String,
    t: tauri::AppHandle,
) -> mpsc::Receiver<CrazyflieBackendCommand> {
    let link_context = crazyflie_link::LinkContext::new(async_executors::AsyncStd);

    // let cf = crazyflie_lib::Crazyflie::connect_from_uri(
    //     async_executors::AsyncStd,
    //     &link_context,
    //     uri.as_str(),
    // )
    // .await.unwrap();

    // match cf {
    //     Ok(cf) => {
    //         println!("Connected to Crazyflie on {}", uri);
    //         let response = ConnectedEvent {
    //             connected: true,
    //             uri,
    //             err: None,
    //         };
    //         t.emit_all("connected", &response).unwrap();

    //         let mut console_stream = cf.console.line_stream().await;
           
    //     }
    //     Err(e) => {
    //         println!("Error connecting to Crazyflie on {}: {:?}", uri, e);
    //         let response = ConnectedEvent {
    //             connected: false,
    //             uri,
    //             err: Some(e.to_string()),
    //         };
    //         t.emit_all("connected", &response).unwrap();
    //     }
    // }

    //let mut console_stream = cf.console.line_stream().await;

    // tokio::select! {
    //   Some(line) = console_stream.next() => {
    //       println!("Console line: {:?}", line);
    //   }
    // }

    ui_to_cf_rx
}

async fn crazyflie_backend(
    mut ui_to_cf_rx: mpsc::Receiver<CrazyflieBackendCommand>,
    t: tauri::AppHandle,
) {
    let state = CrazyflieBackendState::Idle;

    loop {
        // Idle -> Scanning or Connecting
        // Scanning -> Idle
        // Connecting -> Connected
        // Connected -> Idle
        // if state

        let cmd = ui_to_cf_rx.recv().await;

        match cmd {
            Some(output) => match output {
                CrazyflieBackendCommand::StartScan(address) => {
                    println!("Start scanning for Crazyflies on address {}", address);
                    ui_to_cf_rx =
                        crazyflie_backend_scan(ui_to_cf_rx, address.to_string(), t.clone()).await;
                }
                CrazyflieBackendCommand::Connect(uri) => {
                    println!("Connect to Crazyflie on {}", uri);
                    ui_to_cf_rx =
                        crazyflie_backend_connected(ui_to_cf_rx, uri.to_string(), t.clone()).await;
                }
                _ => {
                    println!("Error: Received CF command: {:?} in main state!", output);
                }
            },
            None => {
                println!("Error: Received None from ui_to_cf_rx in main state!");
            }
        }
        // tokio::select! {
        //     Some(ui_msg) = cf_to_ui_rx.recv() => {
        //         println!("Received UI message: {}", ui_msg.data);
        //     }
        //     Some(cf_cmd) = ui_to_cf_rx.recv() => {
        //         println!("Received CF command: {}", cf_cmd.cmd);
        //     }
        // }
    }
}

fn main() {
    let (ui_to_cf_tx, ui_to_cf_rx) = mpsc::channel::<CrazyflieBackendCommand>(1);

    tauri::Builder::default()
        .manage(BackendState::new(ui_to_cf_tx))
        .invoke_handler(tauri::generate_handler![
            scan, start_scan, stop_scan, connect, disconnect
        ])
        .setup(|app| {
            // This thread runs the Crazyflie backend and receives commands from the
            // UI via Tauri commands
            let t = app.handle();
            tauri::async_runtime::spawn(async move {
                crazyflie_backend(ui_to_cf_rx, t).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
