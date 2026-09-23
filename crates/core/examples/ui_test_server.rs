//! Test-only transport. Each request reopens the real database; never included in the desktop.
use scholar_core::{Command, Store};
use serde_json::{json, Value};
use tiny_http::{Header, Response, Server};
fn main() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("ui.sqlite");
    let server = Server::http("127.0.0.1:4319").unwrap();
    println!("UI test service ready");
    for mut request in server.incoming_requests() {
        let mut input = String::new();
        request
            .as_reader()
            .take(100_000_001)
            .read_to_string(&mut input)
            .unwrap();
        let result = (|| -> Result<Value, String> {
            let mut store = Store::open(&path)?;
            if request.method().as_str() == "GET" {
                return Ok(json!({"ready":true}));
            }
            let v: Value = serde_json::from_str(&input).map_err(|e| e.to_string())?;
            match v["method"].as_str().unwrap_or("") {
                "execute" => serde_json::to_value(
                    store.execute(
                        serde_json::from_value::<Command>(v["args"]["command"].clone())
                            .map_err(|e| e.to_string())?,
                    )?,
                )
                .map_err(|e| e.to_string()),
                "preview_backup" => Ok(json!(Store::preview(
                    v["args"]["json"].as_str().unwrap_or("")
                )?)),
                "restore_backup" => Ok(json!(store
                    .restore(v["args"]["json"].as_str().unwrap_or(""))?
                    .display()
                    .to_string())),
                "test_backup" => Ok(json!(store.backup()?)),
                _ => Err("Unknown test method".into()),
            }
        })();
        let payload = match result {
            Ok(value) => json!({"value":value}),
            Err(error) => json!({"error":error}),
        };
        let response = Response::from_string(payload.to_string())
            .with_header(Header::from_bytes("Content-Type", "application/json").unwrap());
        request.respond(response).unwrap();
    }
}
use std::io::Read;
