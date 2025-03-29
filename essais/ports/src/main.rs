use warp::Filter;
use serde::{Deserialize, Serialize};
use warp::http::Method;
use rusqlite::{params, Connection, Result};
// use std::sync::Mutex;
use std::sync::{Arc, Mutex}; // Import Arc


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

// struct Database {
//     conn: Mutex<Connection>,
// }
#[derive(Clone)]
struct Database {
    conn: Arc<Mutex<Connection>>, // Use Arc<Mutex<Connection>>
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
            // conn: Mutex::new(conn),
            conn: Arc::new(Mutex::new(conn)), // Wrap in Arc

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

#[tokio::main]
async fn main() {
    // let db = Database::new("../../../../idsdb.db");
    // let db_filter = warp::any().map(move || db.clone());

    let db = Database::new("../../../../idsdb.db");
    let db = Arc::new(db); // Wrap Database in Arc
    let db_filter = warp::any().map(move || Arc::clone(&db)); // Clone Arc
    

    let hello = warp::path("hello")
        .and(warp::post())
        .and(warp::body::json())
        .and(db_filter)
        .map(|req: UserRequest, db: Arc<Database>| {
            match db.check_user(&req.name) {
                Ok(Some(stored_pw)) => {
                    if let Some(provided_pw) = req.password {
                        if provided_pw == stored_pw {
                            warp::reply::json(&Response {
                                message: format!("Hello, {}!", req.name),
                                request_password: false,
                                allow_creation: false,
                            })
                        } else {
                            warp::reply::json(&Response {
                                message: "Incorrect password".to_string(),
                                request_password: true,
                                allow_creation: false,
                            })
                        }
                    } else {
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
                            db.create_user(&req.name, &req.password.unwrap_or_default()).unwrap();
                            warp::reply::json(&Response {
                                message: "User created successfully".to_string(),
                                request_password: false,
                                allow_creation: false,
                            })
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
                        })
                    }
                }
                Err(_) => warp::reply::json(&Response {
                    message: "Database error".to_string(),
                    request_password: false,
                    allow_creation: false,
                }),
            }
        });

    let routes = hello.with(warp::cors().allow_any_origin().allow_methods(&[Method::POST]).allow_headers(vec!["Content-Type"]));
    warp::serve(routes).run(([0, 0, 0, 0], 4174)).await;
}