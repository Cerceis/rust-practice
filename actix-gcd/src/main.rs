use actix_web::{get, post, App, HttpResponse, web, HttpServer, Responder};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(get_index)
            .service(post_gcd)
    });

    print!("Serving on port 3000"); 

    server
        .bind(("127.0.0.1", 3000)).expect("Error when binding server to port")
        .run()
        .await
}

#[derive(Deserialize)]
struct GCDParameters{
    n: i64,
    m: i64,
}

#[post("/gcd")]
async fn post_gcd(form: web::Form<GCDParameters>) -> impl Responder {

    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text_html")
            .body("0 ? really ...? ")
    }

    let response = format!(r#"
            You know what ? I don't know how to calculate lol.
            Do it yourself !!!! {}, {}
        "#, form.n, form.m);

    HttpResponse::Ok()
        .content_type("text_html")
        .body(response)
}

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type=text name=n />
                    <input type=text name=m />
                    <button type=submit>Compute GCD</button>
                </form>
            "#
        )
}
