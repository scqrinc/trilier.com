use std::path::{Path, PathBuf};

use dotenv;

use serde_json::Value;

use actix_files::NamedFile;
use actix_web::client::Client;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};

mod env;

const DOCUMENT_ROOT_DIR: &str = "./app-web/dist";

async fn search(params: web::Json<env::SearchParams>) -> Result<HttpResponse> {
    let translate_api_key =
        dotenv::var("TRANSLATE_API_KEY").expect("TRANSLATE_API_KEY must be set.");
    // todo: sanitize src_lang / keywords ?
    let url_tmpl = format!(
        "https://www.googleapis.com/language/translate/v2?key={}&q={}&source={}&target=",
        translate_api_key, params.keywords, params.src_lang
    );

    let mut results = Vec::new();
    for dst_lang in &params.dst_langs {
        // todo: sanitize dst_lang ?
        let url = format!("{}{}", url_tmpl, dst_lang);
        let result = query_translate(&params.src_lang, dst_lang, &url).await;
        results.push(result);
    }
    let search_results = env::SearchResults { results: results };
    Ok(HttpResponse::Ok().json(search_results))
}
async fn query_translate(src_lang: &str, dst_lang: &str, url: &str) -> env::SearchResult {
    let client = Client::default();
    let mut resp = client.get(url).send().await.unwrap();

    let resp_body_bytes = resp.body().await.unwrap();
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
        .get("data")
        .unwrap()
        .get("translations")
        .unwrap()
        .get(0)
        .unwrap();
    let translated_text = translation.get("translatedText").unwrap().as_str().unwrap();
    let detected_src_lang = match translation.get("detectedSourceLanguage") {
        Some(x) => x.as_str().unwrap(),
        None => src_lang,
    };
    env::SearchResult {
        src_lang: detected_src_lang.to_string(),
        dst_lang: dst_lang.to_string(),
        translated_text: translated_text.to_string(),
    }
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
