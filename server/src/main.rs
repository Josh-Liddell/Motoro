mod routes;
mod tasks;

use actix_web::{App, HttpServer, web};
use jlrs::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting julia runtime...");
    let (async_handle, _thread_handle) = Builder::new()
        .n_threads(4)
        .async_runtime(Tokio::<3>::new(false))
        .spawn()
        .expect("cannot init Julia");

    let handle = web::Data::new(async_handle);

    println!("Running server on port 8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(handle.clone())
            .service(routes::test)
            .service(routes::test2)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    // thread_handle.join().expect("runtime thread panicked"); // ???
    // idk figure out the shutting down of it !!!!!!!!!!
}
