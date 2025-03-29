use warp::Filter;
use serde::{Deserialize, Serialize};
use warp::http::Method;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use chrono::{NaiveDateTime, Utc, TimeZone};
use regex::Regex;

#[derive(Deserialize)]
struct UserRequest {
    name: String,
    password: Option<String>,
    master_password: Option<String>,
}

#[derive(Serialize)]
struct Response {
    message: String,
    request_password: bool,
    allow_creation: bool,
}

#[derive(Clone)]
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

    fn check_user(&self, name: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT pwkey FROM meta WHERE user = ?1")?;
        let mut rows = stmt.query(params![name])?;
        if let Some(row) = rows.next()? {
            let pw: String = row.get(0)?;
            Ok(Some(pw))
        } else {
            Ok(None)
        }
    }

    fn create_user(&self, name: &str, password: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("INSERT INTO meta (user, pwkey) VALUES (?1, ?2)", params![name, password])?;
        Ok(())
    }
}

fn fetch_user_data(&self, name: &str) -> Result<Option<String>> {
    let conn = self.conn.lock().unwrap();
    let table_name = format!("{}", name);
    let query = format!("SELECT * FROM {} ORDER BY timestamp DESC", table_name);
    let mut stmt = match conn.prepare(&query) {
        Ok(stmt) => stmt,
        Err(_) => return Ok(None),
    };

    let mut rows = stmt.query([])?;
    let mut output = String::new();
    let regex = Regex::new(r"#\\w+").unwrap();

    while let Some(row) = rows.next()? {
        let timestamp: String = row.get("timestamp")?;
        let timestamp = NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S").unwrap();
        let cet_time = Utc.from_utc_datetime(&timestamp).with_timezone(&chrono::FixedOffset::east(3600));
        output.push_str(&format!("<span style='color: rgba(0,0,0,0.5);'>{}</span>\n", cet_time.format("%Y-%m-%d %H:%M:%S")));
        for i in 0..stmt.column_count() {
            let field_value: String = row.get(i)?;
            let formatted_value = regex.replace_all(&field_value, |caps: &regex::Captures| {
                format!("<span style='color: blue;'>{}</span>", &caps[0])
            });
            output.push_str(&format!("<span style='font-family: Courier;'>{}</span>\n", formatted_value));
        }
        output.push_str("\n");
    }

    Ok(Some(output))
}


#[tokio::main]
async fn main() {
    let db = Database::new("../../../../idsdb.db");
    let db = Arc::new(db);
    let db_filter = warp::any().map(move || Arc::clone(&db));

    let hello = warp::path("hello")
        .and(warp::post())
        .and(warp::body::json())
        .and(db_filter)
        .map(
            |req: UserRequest, db: Arc<Database>| {
            match db.check_user(&req.name) {
                Ok(Some(stored_pw)) => {
                    if let Some(provided_pw) = req.password {
                        // if provided_pw == stored_pw {
                        //     warp::reply::json(&Response {
                        //         message: format!("Hello, {}!", req.name),
                        //         request_password: false,
                        //         allow_creation: false,
                        //     })
                        // } 
                        if provided_pw == stored_pw {
                            match db.fetch_user_data(&req.name) {
                                Ok(Some(user_data)) => warp::reply::json(&Response {
                                    message: user_data,
                                    request_password: false,
                                    allow_creation: false,
                                });
                              }  
                              else {
                            warp::reply::json(&Response {
                                message: "Incorrect password".to_string(),
                                request_password: true,
                                allow_creation: false,
                            })
                        }
                    }} else {
                        warp::reply::json(&Response {
                            message: "Password required".to_string(),
                            request_password: true,
                            allow_creation: false,
                        })
                    }
                }
                Ok(None) => {
                    if let Some(master_pw) = req.master_password {
                        if db.check_user("master").unwrap() == Some(master_pw) {
                            if let Some(new_password) = req.password {
                                db.create_user(&req.name, &new_password).unwrap();
                                warp::reply::json(&Response {
                                    message: "User created successfully".to_string(),
                                    request_password: false,
                                    allow_creation: false,
                                });
                            } else {
                                warp::reply::json(&Response {
                                    message: "Enter a password to create user".to_string(),
                                    request_password: true,
                                    allow_creation: true,
                                });
                            
                        } else {
                            warp::reply::json(&Response {
                                message: "Incorrect master password".to_string(),
                                request_password: false,
                                allow_creation: false,
                            })
                        }
                    } else {
                        warp::reply::json(&Response {
                            message: "User not existing, create?".to_string(),
                            request_password: false,
                            allow_creation: true,
                        });
                    }
                }
                Err(_) => warp::reply::json(&Response {
                    message: "Database error".to_string(),
                    request_password: false,
                    allow_creation: false,
                }),
            }
        }
        
    
});
    
    let routes = hello.with(warp::cors().allow_any_origin().allow_methods(&[Method::POST]).allow_headers(vec!["Content-Type"]));
    warp::serve(routes).run(([0, 0, 0, 0], 4174)).await;
}