use axum::{
    extract::{Form, State},
    response::Redirect,
    http::{Response, StatusCode}
    };

use std::{
    fs,
    convert::Infallible,
    env
    };

use bb8_postgres::{
    tokio_postgres::{
        types::ToSql,
        NoTls,
        Config,
        Error
        },
    PostgresConnectionManager,
    bb8::Pool
};

use dotenv::dotenv;

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    nome: String,
    cognome: String,
    email: String,
}



#[derive(Clone)]
pub struct AppState {
    db_pool: Pool<PostgresConnectionManager<NoTls>>
}

impl  AppState {


    pub async fn make_pool() -> Self {
        
        dotenv().ok();

        let mut config = Config::new();
        
        config.user(env::var("DB_USER").unwrap().as_str());
        config.password(env::var("DB_PASSWORD").unwrap().as_str());
        config.dbname(env::var("DB_NAME").unwrap().as_str());
        config.host(env::var("DB_HOST").unwrap().as_str());
        config.port(env::var("DB_PORT").unwrap().parse().unwrap());
        
        let manager = PostgresConnectionManager::new(config, NoTls);
    
        let pool = Pool::builder()
            .max_size(5)
            .build(manager)
            .await.unwrap();
    
        AppState {
            db_pool: pool,
        }
    }
}

pub async fn handle_form(State(client): State<AppState>, Form(data): Form<FormData>) -> Result<Redirect, StatusCode> {
    println!("->>\thandle_form__handler called");

    //let conn = client.db_pool.get().await.unwrap();
    let conn = match client.db_pool.get().await {
        Ok(connection) => connection,
        Err(error) => {
            eprintln!("Failed to get a database connection: {}", error);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    

    let values: &[&(dyn ToSql + Sync)] = &[&data.nome, &data.cognome, &data.email];

    let query = "INSERT INTO users (nome, cognome, email) VALUES ($1, $2, $3)";
    if let Err(e) = conn.query(query, values)
        .await.map_err(|e| Error::from(e)) {   
        eprintln!("error executing query: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Redirect::to("/form/success/"))
}

async fn read_file_handler(filename: &str) -> Result<Response<String>, Infallible> {
    match fs::read_to_string(filename) {
        Ok(contents) => Ok(Response::new(contents)),
        Err(e) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(e.to_string())
            .unwrap()),
    }
}

pub async fn form_handler() -> Result<Response<String>, Infallible> {
    println!("->>\tform_handler called");
    read_file_handler("html/form.html").await
}

pub async fn success_handler() -> Result<Response<String>, Infallible> {
    println!("->>\tsuccess_handler called");
    read_file_handler("html/success.html").await
}

pub async fn index_handler() -> Result<Response<String>, Infallible> {
    println!("->>\tindex_handler called");
    read_file_handler("html/home.html").await
}

pub async fn about_handler() -> Result<Response<String>, Infallible> {
    println!("->>\tabout_handler called");
    read_file_handler("html/about.html").await
}

