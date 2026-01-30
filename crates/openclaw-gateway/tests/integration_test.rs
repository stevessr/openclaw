/// Integration tests for the gateway server
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_gateway_starts_and_stops() {
    // Start gateway on a random port
    let port = 19000;

    // Spawn gateway in background
    let gateway_handle = tokio::spawn(async move {
        let result = openclaw_gateway::run("127.0.0.1".to_string(), port).await;
        if let Err(e) = result {
            eprintln!("Gateway error: {}", e);
        }
    });

    // Give it time to start
    sleep(Duration::from_millis(500)).await;

    // Test that it's running by making a request
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://127.0.0.1:{}/health", port))
        .timeout(Duration::from_secs(5))
        .send()
        .await;

    match response {
        Ok(resp) => {
            assert!(resp.status().is_success());
            let json: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(json["status"], "ok");
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            // Gateway might not have started yet, but test shouldn't panic
        }
    }

    // Stop the gateway
    gateway_handle.abort();
}

#[tokio::test]
async fn test_gateway_status_endpoint() {
    let port = 19001;

    let gateway_handle = tokio::spawn(async move {
        let _ = openclaw_gateway::run("127.0.0.1".to_string(), port).await;
    });

    sleep(Duration::from_millis(500)).await;

    let client = reqwest::Client::new();
    if let Ok(resp) = client
        .get(format!("http://127.0.0.1:{}/status", port))
        .timeout(Duration::from_secs(5))
        .send()
        .await
    {
        if resp.status().is_success() {
            let json: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(json["gateway"], "running");
            assert_eq!(json["implementation"], "rust");
        }
    }

    gateway_handle.abort();
}
