use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result, Row};
use std::sync::{Arc, Mutex};
use chrono::{NaiveDateTime, Utc, TimeZone};
use regex::Regex;

#[derive(Deserialize)]
struct UserRequest {
    name: String,
    password: Option<String>,
    master_password: Option<String>,
    new_password: Option<String>,
}

#[derive(Serialize)]
struct Response {
    message: String,
    request_password: bool,
    allow_creation: bool,
}

struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    fn new(db_path: &str) -> Self {
        let conn = Connection::open(db_path).expect("Failed to open database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS meta (
                user TEXT PRIMARY KEY,
                pwkey TEXT NOT NULL
            )",
            [],
        ).expect("Failed to create table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    fn fetch_user_data(&self, user: &str) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        let query = format!("SELECT * FROM {} ORDER BY timestamp DESC", user);
        let mut stmt = conn.prepare(&query)?;
        let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
        let rows = stmt.query_map([], |row| {
            let mut row_data = String::new();
            let regex = Regex::new(r"#\\w+").unwrap();
            for (i, col) in column_names.iter().enumerate() {
                if col == "timestamp" {
                    let timestamp: String = row.get(i)?;
                    let timestamp = NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S").unwrap();
                    let cet_time = Utc.from_utc_datetime(&timestamp).with_timezone(&chrono::FixedOffset::east(3600));
                    row_data.push_str(&format!("<span style='color: rgba(0,0,0,0.5);'>{}</span>\n", cet_time.format("%Y-%m-%d %H:%M:%S")));
                } else {
                    let field_value: String = row.get(i)?;
                    let formatted_value = regex.replace_all(&field_value, |caps: &regex::Captures| {
                        format!("<span style='color: blue;'>{}</span>", &caps[0])
                    });
                    row_data.push_str(&format!("<span style='font-family: Courier;'>{}</span>\n", formatted_value));
                }
            }
            Ok(row_data)
        })?;
        
        let output: Vec<String> = rows.filter_map(|row| row.ok()).collect();
        Ok(output.join("\n"))
    }
}

#[post("/hello")]
async fn hello(req: web::Json<UserRequest>, db: web::Data<Arc<Database>>) -> impl Responder {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT pwkey FROM meta WHERE user = ?1").unwrap();
    let mut rows = stmt.query(params![req.name.clone()]).unwrap();
    
    if let Some(row) = rows.next().unwrap() {
        let stored_pw: String = row.get(0).unwrap();
        if let Some(provided_pw) = &req.password {
            if provided_pw == &stored_pw {
                match db.fetch_user_data(&req.name) {
                    Ok(user_data) => {
                        return HttpResponse::Ok().json(Response {
                            message: user_data,
                            request_password: false,
                            allow_creation: false,
                        });
                    }
                    Err(_) => {
                        return HttpResponse::Ok().json(Response {
                            message: "Failed to fetch user data".to_string(),
                            request_password: false,
                            allow_creation: false,
                        });
                    }
                }
            }
        }
        return HttpResponse::Ok().json(Response {
            message: "Incorrect password".to_string(),
            request_password: true,
            allow_creation: false,
        });
    }
    
    HttpResponse::Ok().json(Response {
        message: "User not found".to_string(),
        request_password: false,
        allow_creation: true,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Arc::new(Database::new("../../../../idsdb.db"));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .service(hello)
    })
    .bind(("0.0.0.0", 4174))?
    .run()
    .await
}
