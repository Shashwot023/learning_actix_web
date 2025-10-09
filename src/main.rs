use actix_web::{App, get, web, Responder, HttpResponse, HttpServer};

struct AppState{
    app_name: String,
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Welcome to {app_name}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        app_name: String::from("Math Helper"),
    });

    HttpServer::new(move || {
                    App::new()
                    .app_data(app_state.clone())
                    .route("/", web::get().to(index))
                    })
    .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
