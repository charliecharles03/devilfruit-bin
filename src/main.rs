use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use awc::Client;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/getFollowerCount")]
async fn echo(req_body: String) -> impl Responder {
    let client = Client::default();


    // Perform a GET request to an example URL
    let url = "http://localhost:8080/hey";
    let response = client.get(url).send().await;

    match response {

        Ok(mut resp) => {
            if let Ok(body) = resp.body().await {
                let body_text = String::from_utf8_lossy(&body);
                format!("Response: {}", body_text)
            } else {
                "Failed to read the response body.".to_string()
            }
        }
        Err(e) => format!("Error occurred: {}", e),
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("0")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
