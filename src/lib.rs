use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            //ドキュメントルート及びアプリが受信するリクエストパスを設定
            .route("/demo", web::get().to(greet))
            //動的パス
            .route("/demo/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    // ソケットにバインドするip:portを設定
    .bind("127.0.0.1:8000")?
    // サーバを起動する。
    .run();
    Ok(server)
}
