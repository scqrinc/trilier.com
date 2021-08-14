use std::path::{Path, PathBuf};

use dotenv;
use serde_json::Value;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_files::NamedFile;

use actix_web::client::Client;

mod env;

const DOCUMENT_ROOT_DIR: &str = "./app-web/dist";

const TRANSLATE_API_URL: &str = "https://api-free.deepl.com/v2/translate";

async fn search(params: web::Json<env::SearchParams>) -> Result<HttpResponse> {
    let translate_api_key =
        dotenv::var("TRANSLATE_API_KEY").expect("TRANSLATE_API_KEY must be set.");
    // todo: sanitize src_lang / original_text ?

    let mut results = Vec::new();
    for dst_lang in &params.dst_langs {
        let result = query_translate(&params.original_text, &params.src_lang, dst_lang, translate_api_key.as_ref()).await;
        results.push(result);
    }
    let search_results = env::SearchResults { results: results };
    Ok(HttpResponse::Ok().json(search_results))
}
async fn query_translate(original_text: &str, src_lang: &str, dst_lang: &str, translate_api_key: &str) -> env::SearchResult {
    // let params = [
    //     ("auth_key", translate_api_key),
    //     ("text", original_text),
    //     ("source_lang", src_lang),
    //     ("target_lang", dst_lang)
    // ];
    let params = format!("auth_key={}&text={}&source_lang={}&target_lang={}",
        translate_api_key,
        original_text,
        src_lang,
        dst_lang
    );
    // let params = serde_json::json!({
    //     "auth_key": translate_api_key,
    //     "text": original_text,
    //     "source_lang": src_lang,
    //     "target_lang": dst_lang,
    // });
    let client = Client::default();
    let req = client
        .post(TRANSLATE_API_URL)
        // .set(actix_web::http::header::ContentType::form_url_encoded())
        // .set(actix_web::http::header::ContentType::json())
        ;
    println!("{},{},{},{},{},{:?},{:?}",
        translate_api_key, original_text, src_lang, dst_lang, TRANSLATE_API_URL,
        req, params);
    // let resp = req.send_form(&params).await;
    // let resp = req.send_json(&params).await;
    let resp = req.send_body(&params).await;

    match resp {
        Ok(_) => {},
        Err(error) => {
            panic!("Server error: {}", error);
        },
    }

    let resp_body_bytes = resp.unwrap().body().await.unwrap();
    let resp_body_str = std::str::from_utf8(resp_body_bytes.as_ref()).unwrap();
    let resp_body_json: Value = serde_json::from_str(resp_body_str).unwrap();

    match resp_body_json.get("error") {
        Some(x) => {
            return env::SearchResult {
                src_lang: src_lang.to_string(),
                dst_lang: dst_lang.to_string(),
                translated_text: format!("(Failed: {})", x.get("message").unwrap()),
            }
        }
        None => {}
    }

    let translation = resp_body_json
        .get("translations")
        .unwrap()
        .get(0)
        .unwrap();
    let translated_text = translation.get("text").unwrap().as_str().unwrap();
    let detected_src_lang = match translation.get("detected_source_language") {
        Some(x) => x.as_str().unwrap(),
        None => src_lang,
    };
    env::SearchResult {
        src_lang: detected_src_lang.to_string(),
        dst_lang: dst_lang.to_string(),
        translated_text: translated_text.to_string(),
    }

    // env::SearchResult {
    //     src_lang: "".to_string(),
    //     dst_lang: "".to_string(),
    //     translated_text: "".to_string(),
    // }
}

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = Path::new(DOCUMENT_ROOT_DIR).join("index.html");
    Ok(NamedFile::open(path)?)
}
async fn static_file(req: HttpRequest) -> Result<NamedFile> {
    let rel_path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let path: PathBuf = Path::new(DOCUMENT_ROOT_DIR).join(rel_path);
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    // handles requests for `GET /api/ ...`
                    .route("/search", web::post().to(search)),
            )
            .service(
                web::scope("")
                    .route("/", web::get().to(index))
                    .route("/{filename:.*}", web::get().to(static_file)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
