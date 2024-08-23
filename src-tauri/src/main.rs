// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
use tauri::Emitter;
use tokio::runtime::Runtime;

use futures::StreamExt;
//use tokio::sync::mpsc;
use flume as mpsc;
use futures_util::task::SpawnExt;
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
        
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[tauri::command]
async fn disconnect(state: tauri::State<'_, BackendState>) -> Result<(), String> {
    state
        .cmd
        .send(CrazyflieBackendCommand::Disconnect)
        
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[tauri::command]
async fn start_scan(address: &str, state: tauri::State<'_, BackendState>) -> Result<(), String> {
    println!("Start Crazyflies on address {}", address);
    state
        .cmd
        .send(CrazyflieBackendCommand::StartScan(address.to_string()))
        
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[tauri::command]
async fn stop_scan(state: tauri::State<'_, BackendState>) -> Result<(), String> {
    println!("Stop scanning");
    state
        .cmd
        .send(CrazyflieBackendCommand::StopScan)
        
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
    // Scanning,
    // Connecting,
    // Connected,
}

#[derive(Debug, serde::Serialize, TS)]
#[ts(export, export_to = "../src/interface/scan_response.ts")]
struct ScanResponse {
    uris: Vec<String>,
    err: Option<String>,
}

#[derive(Debug, serde::Serialize, TS)]
#[ts(export, export_to = "../src/interface/connected_event.ts")]
struct ConnectedEvent {
    connected: bool,
    uri: String,
    err: Option<String>,
}

#[derive(Debug, serde::Serialize, TS)]
#[ts(export, export_to = "../src/interface/console_event.ts")]
struct ConsoleEvent {
    message: String,
}

async fn crazyflie_backend_scan(
    ui_to_cf_rx: mpsc::Receiver<CrazyflieBackendCommand>,
    address: String,
    t: tauri::AppHandle,
) -> mpsc::Receiver<CrazyflieBackendCommand> {
    let link_context = crazyflie_link::LinkContext::new(async_executors::AsyncStd);
    //let mut ui_to_cf_rx_inner = ui_to_cf_rx.unwrap();
    loop {
        println!("Looping around to scan again!");
        println!("Scanning for Crazyflies on address {}", address);
        match link_context.scan([0xE7; 5]).await {
          Ok(uris) => {
              let response = ScanResponse { uris, err: None };
              t.emit("scan", &response).unwrap();
          }
          Err(e) => {
              println!("Error scanning for Crazyflies: {:?}", e);
              let response = ScanResponse { uris: vec![], err: Some(e.to_string()) };
              t.emit("scan", &response).unwrap();
              break;
          }
      }

      match ui_to_cf_rx.try_recv() {
        Ok(input) => {
            match input {
                CrazyflieBackendCommand::StopScan => {
                  println!("Will be exiting scan mode!");
                    break;
                }
                _ => {
                    println!("Error: Received CF command: {:?} in scan state!", input);
                }
            }
        }
        _ => {}
      }
    }
    println!("Exiting scan mode!");

    ui_to_cf_rx
}

async fn crazyflie_backend_connected(
    ui_to_cf_rx: mpsc::Receiver<CrazyflieBackendCommand>,
    uri: String,
    t: tauri::AppHandle,
) -> mpsc::Receiver<CrazyflieBackendCommand> {
    let link_context = crazyflie_link::LinkContext::new(async_executors::AsyncStd);

    println!("Calling connect now on {}!", uri);

    let cf = async_executors::AsyncStd::block_on(async {
      crazyflie_lib::Crazyflie::connect_from_uri(async_executors::AsyncStd, &link_context, uri.as_str())
          .await
  });

    match cf {
        Ok(cf) => {
            println!("Connected to Crazyflie on {}", uri);
            let response = ConnectedEvent {
                connected: true,
                uri,
                err: None,
            };
            t.emit("connected", &response).unwrap();

            let (tx, rx) = flume::unbounded::<String>();

            std::thread::spawn( move || {
                while let Ok(line) = rx.recv() {
                  let response = ConsoleEvent { message: line.clone()};
                  t.emit("console", &response).unwrap();
                    println!("Received: {}", line);
                }
            });

            println!("Starting console");
    
            //async_executors::AsyncStd::block_on(async {
                let mut stream = cf.console.line_stream().await;
    
                async_executors::AsyncStd.spawn(async move {
                    while let Some(line) = stream.next().await {
                        tx.send_async(line).await.expect("Failed to send line");
                    }
                    println!("Exiting console stream loop");
                }).unwrap();
    
            //}).unwrap();

            println!("Starting to sleep a bit");

            async_std::task::sleep(Duration::from_secs(10)).await;

            println!("Have Slept, waiting for disconnect");

            cf.wait_disconnect().await;

            println!("Exiting main connected loop");
    
           
        }
        Err(e) => {
            println!("Error connecting to Crazyflie on {}: {:?}", uri, e);
            let response = ConnectedEvent {
                connected: false,
                uri,
                err: Some(e.to_string()),
            };
            t.emit("connected", &response).unwrap();
        }
    }

    ui_to_cf_rx
}

async fn crazyflie_backend(
    mut ui_to_cf_rx: mpsc::Receiver<CrazyflieBackendCommand>,
    t: tauri::AppHandle,
) {
    let _state = CrazyflieBackendState::Idle;

    loop {
        // Idle -> Scanning or Connecting
        // Scanning -> Idle
        // Connecting -> Connected
        // Connected -> Idle
        // if state

        let cmd = ui_to_cf_rx.recv();

        match cmd {
            Ok(output) => match output {
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
            Err(e) => {
                println!("Error receiving CF command: {:?}", e);
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
    let (ui_to_cf_tx, ui_to_cf_rx) = mpsc::unbounded();

    tauri::Builder::default()
        .manage(BackendState::new(ui_to_cf_tx))
        .invoke_handler(tauri::generate_handler![
            scan, start_scan, stop_scan, connect, disconnect
        ])
        .setup(|app| {
            // This thread runs the Crazyflie backend and receives commands from the
            // UI via Tauri commands
            let t = app.handle().clone();
            std::thread::spawn(move || {

                let t = t.to_owned();

                let rt = Runtime::new().unwrap();

                let _guard = rt.enter();
                rt.block_on(crazyflie_backend(ui_to_cf_rx, t));
                
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
