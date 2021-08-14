use std::path::{Path, PathBuf};

use dotenv;
use serde_json::Value;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_files::NamedFile;

// use actix_web::client::Client;
use std::str;
use std::io::Read;
use curl::easy::Easy;

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
    let req_body_string = format!(
        "auth_key={}&text={}&source_lang={}&target_lang={}",
        translate_api_key,
        original_text,
        src_lang,
        dst_lang
    );
    let mut req_body = req_body_string.as_bytes();

    let mut resp = Vec::new();
    let mut easy = Easy::new();
    easy.url(TRANSLATE_API_URL).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(req_body.len() as u64).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.read_function(|req_buf| {
            Ok(req_body.read(req_buf).unwrap_or(0))
        }).unwrap();
        transfer.write_function(|resp_buf| {
            resp.extend_from_slice(resp_buf);
            Ok(resp_buf.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    
    let server_response_code = easy.response_code().unwrap();
    if server_response_code < 200 || 300 <= server_response_code {
        panic!("Server error.");
    }
    let resp_txt = match str::from_utf8(&resp) {
        Ok(x) => x,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let resp_body_json: Value = serde_json::from_str(resp_txt).unwrap();
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
