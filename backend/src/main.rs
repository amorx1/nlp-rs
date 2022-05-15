use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use rust_bert::pipelines::translation::{TranslationModel, TranslationModelBuilder, TranslationConfig};
use serde::{Deserialize, Serialize};
use tch::Device;
use rust_bert::pipelines::common::ModelType;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use lingua::Language::{English, French, German, Spanish};
use actix_cors::Cors;
use std::sync::Mutex;
use log::info;
use tokio::task::spawn_blocking;

// use rust_bert::pipelines::translation

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub query: String,
}

pub struct MyData {
    Model: TranslationModel,
}

// #[get("/")]
pub async fn index(data: web::Data<Mutex<MyData>>) -> impl Responder {
    // let data = data.lock().unwrap();
    web::Bytes::from_static(b"Hello World")
}

async fn infer_input(query: &String) -> String {
    let languages = vec![English, French, German, Spanish];
    let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();
    let detected_language: Option<Language> = detector.detect_language_of(query);
    match detected_language {
        Some(l) => l.to_string(),
        None => "FAILED INFERENCE".to_string(),
    }
}

async fn convert_lang(lang: String) -> rust_bert::pipelines::translation::Language {
    if lang == "English" {
        rust_bert::pipelines::translation::Language::English
    }
    else if lang == "Spanish" {
        rust_bert::pipelines::translation::Language::Spanish
    }
    else if lang == "German" {
        rust_bert::pipelines::translation::Language::German
    }
    else if lang == "French" {
        rust_bert::pipelines::translation::Language::French
    }
    else {
        rust_bert::pipelines::translation::Language::English
    }
}

// , data: web::Data<Mutex<MyData>>
// #[post("/")]
async fn pred_from_query(q: web::Json<Query>, data: web::Data<Mutex<MyData>>) -> HttpResponse {
    let data = data.lock().unwrap();
    let query = format!("{}", q.query);

    if query == "".to_string() {
        return HttpResponse::Ok().body("Enter phrase")
    }

    if query.len() < 5 {
        return HttpResponse::Ok().body("Enter phrase")  
    }

    let inferred_language = infer_input(&query).await;
    let source_language = convert_lang(inferred_language).await;

    let model = &data.Model;
    let output = model.translate(&[query], source_language, rust_bert::pipelines::translation::Language::Spanish).unwrap();
    let res = &output[0];

    HttpResponse::Ok().body(res.to_string())
    // HttpResponse::Ok().body("Ok")

}

pub fn get_model() -> TranslationModel {
    TranslationModelBuilder::new()
    .with_model_type(ModelType::M2M100)
        .with_source_languages(vec![rust_bert::pipelines::translation::Language::English, rust_bert::pipelines::translation::Language::Spanish, rust_bert::pipelines::translation::Language::French, rust_bert::pipelines::translation::Language::German])
        .with_target_languages(vec![rust_bert::pipelines::translation::Language::Spanish, rust_bert::pipelines::translation::Language::French, rust_bert::pipelines::translation::Language::German])
        .create_model().unwrap()
    // TranslationModelBuilder::new()
    // .with_medium_model()
    // .with_source_languages(vec![rust_bert::pipelines::translation::Language::English, rust_bert::pipelines::translation::Language::Spanish, rust_bert::pipelines::translation::Language::French, rust_bert::pipelines::translation::Language::German])
    // .with_target_languages(vec![rust_bert::pipelines::translation::Language::English, rust_bert::pipelines::translation::Language::Spanish, rust_bert::pipelines::translation::Language::French, rust_bert::pipelines::translation::Language::German])
    // .create_model().unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let model = spawn_blocking(move || {get_model()}).await;
    let data = web::Data::new(Mutex::new(MyData{ Model: model.unwrap()}));
    // let data = web::Data::new(Mutex::new(MyData{ Model: model }));
    HttpServer::new(move || {
        App::new()
        .wrap(
            Cors::permissive()
        )
        .app_data(data.clone())
        .service(
            web::resource("/")
            .route(
                web::get().to(index)
            )
        )
        .service(
            web::resource("/predict")
            .route(
                web::post().to(pred_from_query)
            )
        )
        // .route("/", web::get().to(index))
        // .service(index)
        // .service(pred_from_query)
        // .route("/predict", web::post().to(pred_from_query))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
