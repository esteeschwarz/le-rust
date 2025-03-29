use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

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
        println!("user: {},self: {}",user,&self);
        let query = format!("SELECT * FROM {} ORDER BY timestamp DESC", user);
        let mut stmt = conn.prepare(&query)?;
        let column_count = stmt.column_count(); // Get the column count
        println!("nr of columns: {}",column_count);
        let rows = stmt.query_map([], |row| {
            let mut row_data = Vec::new();
            for i in 0..column_count {
                let field_value: String = row.get(i)?;
                println!("fetchdata:{}",field_value);
                row_data.push(field_value);
                
            }
            Ok(row_data.join("\n"))
        })?;
        
        let result: Vec<String> = rows.collect::<Result<_, _>>()?;
        Ok(result.join("\n"))
    }
}

#[post("/hello")]
async fn hello(req: web::Json<UserRequest>, db: web::Data<Arc<Database>>) -> impl Responder {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT pwkey FROM meta WHERE user = ?1").unwrap();
    let mut rows = stmt.query(params![req.name.clone()]).unwrap();
    
    if let Some(row) = rows.next().unwrap() {
        let stored_pw: String = row.get(0).unwrap();
        println!("pw-st:{}",stored_pw);
        
        if let Some(provided_pw) = &req.password {
            println!("pw-pr:{}",provided_pw);
            if provided_pw == &stored_pw {
                println!("passwords match...");
                match db.fetch_user_data(&req.name) {
                    Ok(user_data) => {
                        println!("data: {}",user_data);
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
