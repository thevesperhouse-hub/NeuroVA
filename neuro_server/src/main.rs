use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Json,
        State,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::broadcast;
use futures_util::{stream::StreamExt, SinkExt};
use agi_core::{Core, performance_monitor::{PerformanceMonitor, Metrics}};
use std::env;
use std::sync::atomic::Ordering;

// Define the structure for the request body
#[derive(Debug, Deserialize)]
struct Query {
    prompt: String,
}

// Define the structure for the response body
#[derive(Serialize)]
struct PromptResponse {
    response: String,
}

// Define the application state to be shared across handlers
struct AppState {
    agi_core: Arc<Mutex<Core>>,
    perf_monitor: Arc<Mutex<PerformanceMonitor>>,
    metrics_tx: broadcast::Sender<Metrics>,
}

#[tokio::main]
async fn main() {
    // --- AGI Core Initialization ---
    println!("--- Initializing NeuroVA AGI Core ---");

    // Set the working directory to the project root to ensure correct pathing for knowledge files.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let project_root = std::path::Path::new(manifest_dir).parent().unwrap();
    env::set_current_dir(&project_root).expect("Failed to set working directory");
    println!("Working directory set to: {:?}", project_root);

    let knowledge_path = project_root.join("knowledge.txt");
    let identity_path = project_root.join("identity.txt");

    // 1. Create a new, empty AGI Core.
    let mut core = Core::new(None);

    // 2. Load the identity first to establish the semantic baseline.
    println!("--- Loading identity file... ---");
    if let Err(e) = core.learn_from_file(identity_path.to_str().unwrap()) {
        eprintln!("ERROR: Failed to load identity file: {}", e);
    }

    // 3. Load the general knowledge base.
    println!("--- Loading knowledge base... ---");
    if let Err(e) = core.learn_from_file(knowledge_path.to_str().unwrap()) {
        eprintln!("ERROR: Failed to load knowledge file: {}", e);
    }

        let agi_core = Arc::new(Mutex::new(core));
        let perf_monitor = Arc::new(Mutex::new(PerformanceMonitor::new()));
    let (metrics_tx, _) = broadcast::channel(100);
    println!("--- AGI Core Initialized ---");

    // --- Background Thread for AGI Ticking ---
    // --- Metrics Broadcasting Task ---
    let metrics_tx_clone = metrics_tx.clone();
    let core_for_metrics = Arc::clone(&agi_core);
    let monitor_for_metrics = Arc::clone(&perf_monitor);

    tokio::spawn(async move {
        loop {
            let (concepts_in_memory, power_draw_w) = {
                // Lock, read data, and unlock immediately by ending the scope.
                let core_guard = core_for_metrics.lock().unwrap();
                let concepts = core_guard.hippocampus.holographic_memory.len();
                let power = core_guard.power_draw.load(Ordering::Relaxed);
                (concepts, power)
            };

            let metrics = monitor_for_metrics.lock().unwrap().get_metrics(concepts_in_memory, power_draw_w);
            
            if let Err(_) = metrics_tx_clone.send(metrics) {
                // This can happen if there are no receivers, which is fine.
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    // --- Background Thread for AGI Ticking & Performance Monitoring ---
    let core_for_tick = Arc::clone(&agi_core);
    let monitor_for_tick = Arc::clone(&perf_monitor);
    thread::spawn(move || {
        loop {
            {
                let mut core_guard = core_for_tick.lock().unwrap();
                core_guard.tick();
                let mut monitor_guard = monitor_for_tick.lock().unwrap();
                monitor_guard.tick();
            } // Locks are released here
            
            // Tick at a reasonable rate (e.g., 20 Hz)
            thread::sleep(Duration::from_millis(50));
        }
    });
    println!("--- AGI Core Ticking Thread Started ---");

    // --- Axum Server Setup ---
        let app_state = Arc::new(AppState { agi_core, perf_monitor, metrics_tx });

    let app = Router::new()
        .route("/api/stimulate", post(prompt_handler))
        .route("/api/status", get(status_handler))
                .route("/ws/metrics", get(websocket_handler))
                .route("/agi-load-test", get(agi_load_test_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("NeuroVA Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn agi_load_test_handler(State(state): State<Arc<AppState>>) -> &'static str {
    println!("--- AGI Load Test Started ---");

    tokio::spawn(async move {
        let prompts = vec![
            "What is the nature of consciousness?",
            "Explain the theory of relativity.",
            "Who was Leonardo da Vinci and what were his main contributions?",
            "What is a neural network?",
            "Summarize the plot of the book Dune."
        ];
        let num_requests = 200;

        for i in 0..num_requests {
            let prompt = prompts[i % prompts.len()].to_string();
            
            // Lock, stimulate, and unlock in a tight scope
            let _response = {
                let mut core_guard = state.agi_core.lock().unwrap();
                core_guard.get_response_for_prompt(&prompt)
            };
            
            // Small delay to allow other tasks to run and not completely block everything.
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        println!("--- AGI Load Test Finished ---");
    });

    "AGI load test initiated in the background. Observe the metrics."
}

async fn status_handler() -> axum::Json<serde_json::Value> {
    axum::Json(json!({ "status": "ok" }))
}

#[axum::debug_handler]
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    let mut rx = state.metrics_tx.subscribe();
    let (mut sender, _) = stream.split();

    while let Ok(metrics) = rx.recv().await {
        let payload = serde_json::to_string(&metrics).unwrap();
        if sender.send(Message::Text(payload)).await.is_err() {
            // Client disconnected
            break;
        }
    }
}

#[axum::debug_handler]
async fn prompt_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Query>,
) -> axum::Json<PromptResponse> {
    let prompt = payload.prompt;
    println!("Received prompt: {}", prompt);

    // Lock the AGI core to process the prompt
    let mut agi_core_guard = state.agi_core.lock().unwrap();
    
    // Get the response from the AGI core
    let response_tuple = agi_core_guard.get_response_for_prompt(&prompt);

    // Drop the guard as soon as we're done with the core
    drop(agi_core_guard);

    if let Some((response, _query_type)) = response_tuple {
        axum::Json(PromptResponse {
            response,
        })
    } else {
        axum::Json(PromptResponse {
            response: "The AGI did not produce a response for this prompt.".to_string(),
        })
    }
}
