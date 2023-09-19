//COMMENTS
// 1. We are using the actix_web crate to build our web server
// 2. We are using the chrono crate to generate the current timestamp
// 3. We are using the serde_json crate to serialize the JSON response
// 4. We are using the uuid crate to generate a unique identifier for each note
// 5. We are using the sqlx crate to connect to the database and execute SQL queries
// 6. We are using the dotenv crate to load environment variables from the .env file
// 7. We are using the env_logger crate to log the HTTP requests
// 8. We are using the actix_cors crate to enable CORS

mod api;
mod repository;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// AppState is a struct that holds the database connection pool
pub struct AppState {
    db: Pool<Postgres>,
}


#[actix_web::main]
async fn main()-> std::io::Result<()> {
    // HTTP Request Logger
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    //load environment variables
    dotenv().ok();
    //initialize logger
    env_logger::init();

    //connect to the database
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //create a connection pool
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        //if the connection is successful, the pool will be returned
        Ok(pool) => {
            println! ("Connection to the database is successful");
            pool
        }

        //if the connection fails, the error will be printed to the console and the application will exit
        Err(err) => {
            println! ("Failed to connect to the database: {:?}", err);
            std::process::exit(1)
        }
    };

    println! ("Server started successfully");

    //start the HTTP server
    HttpServer::new(move || {

        //enable CORS
        let cors = Cors::default()
            //allow only requests from http://localhost:3000
            .allowed_origin("http://localhost:3000")
            //allow the following HTTP methods: GET, POST, PATCH, DELETE
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            // allow the following HTTP headers: Content-Type, Authorization, Accept
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT
            ])
            .supports_credentials();
        // create the application instance
        App::new()
            .app_data(web::Data::new(AppState {db: pool.clone()}))
            .configure(api::handler::config)
            .wrap(cors)
            .wrap(Logger::default())

    })
    .bind(("127.0.0.1", 8000))?// bubble up the error since we are using a result
    .run()
    .await
}