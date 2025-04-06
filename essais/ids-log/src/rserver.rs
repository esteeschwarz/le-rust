use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use base64::{decode, encode};
use std::env;
use std::path::Path;

#[derive(Deserialize)]
struct RScriptRequest {
    script_b64: String,
}

#[derive(Serialize)]
struct RScriptResponse {
    response_b64: String,
}

#[post("/rserver")]
async fn run_rscript(req: web::Json<RScriptRequest>) -> impl Responder {
    // Decode the R script
    let decoded_script = match decode(&req.script_b64) {
        Ok(bytes) => String::from_utf8(bytes).unwrap_or_else(|_| "Invalid UTF-8".to_string()),
        Err(_) => return web::Json(RScriptResponse { response_b64: encode("Invalid Base64 input") }),
    };

    // Define script file path
    let script_path = "/tmp/script.R";

    // Save the script to a file
    if let Err(e) = fs::write(script_path, &decoded_script) {
        return web::Json(RScriptResponse { response_b64: encode(format!("Error writing script: {}", e)) });
    }

    // Run the R script using `Rscript script.R`
    let output = Command::new("Rscript")
        .arg(script_path)
        .output();

    let response_text = match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("stdout:\n{}\nstderr:\n{}", stdout, stderr)
        }
        Err(e) => format!("Error running Rscript: {}", e),
    };

    // Cleanup: Remove the temporary script
    let _ = fs::remove_file(script_path);

    // Send Base64-encoded response back
    web::Json(RScriptResponse {
        response_b64: encode(response_text),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "0.0.0.0:8080";
    println!("Server running at {}", server_address);

    HttpServer::new(|| App::new().service(run_rscript))
        .bind(server_address)?
        .run()
        .await
}
