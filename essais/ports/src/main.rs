use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;

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

fn fetch_user_data(conn: &Connection, user: &str) -> Result<String> {
    let query = format!("SELECT * FROM {} ORDER BY timestamp DESC", user);

    let mut stmt = conn.prepare(&query)?;
    let column_count = stmt.column_count(); // Get the column count
    let column_count_int: i32 = column_count as i32;
    println!("nr of columns: {}",column_count);
    let rows = stmt.query_map([], |row| {
        let mut row_data = Vec::new();
        for i in 0..column_count {
            // let field_value: String = row.get(i)?;
            let field_value: String = row.get::<_, String>(10)?;
            println!("field: {}",field_value);
            row_data.push(field_value);
        }
        Ok(row_data.join("\n"))
    })?;
    
    let result: Vec<String> = rows.collect::<Result<_, _>>()?;
    Ok(result.join("\n"))
}

#[post("/hello")]
async fn hello(req: web::Json<UserRequest>, db: web::Data<Mutex<Connection>>) -> impl Responder {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT pwkey FROM meta WHERE user = ?1").unwrap();
    let mut rows = stmt.query(params![req.name.clone()]).unwrap();
    
    if let Some(row) = rows.next().unwrap() {
        let stored_pw: String = row.get(0).unwrap();
        if let Some(provided_pw) = &req.password {
            if provided_pw == &stored_pw {
                println!("pw-pr:{}",provided_pw);
        

                match fetch_user_data(&conn, &req.name) {
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
    let conn = Connection::open("../../../../idsdb.db").expect("Failed to open database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS meta (
            user TEXT PRIMARY KEY,
            pwkey TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create table");
    
    let db = web::Data::new(Mutex::new(conn));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        
        App::new()
            .wrap(cors)
            .app_data(db.clone())
            .service(hello)
    })
    .bind(("0.0.0.0", 4174))?
    .run()
    .await
}
