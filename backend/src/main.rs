use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use lingua::Language::{English, French, German, Spanish};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::summarization::SummarizationModel;
use rust_bert::pipelines::sentiment::{ SentimentModel, Sentiment };
use rust_bert::pipelines::translation::{
    TranslationConfig, TranslationModel, TranslationModelBuilder,
};
use rust_bert::resources::{RemoteResource, Resource};
use rust_bert::t5::{T5ConfigResources, T5ModelResources, T5VocabResources};
use serde::{Deserialize, Serialize, Serializer};
use std::sync::{Arc, Mutex};
use tch::Device;
use tokio::task::spawn_blocking;

#[derive(Debug, Serialize, Deserialize)]
pub struct TQuery {
    pub query: String,
    target: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SQuery {
    pub query: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SeQuery {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    pub service: String,
}

pub struct AppData {
    TModel: Option<TranslationModel>,
    SModel: Option<SummarizationModel>,
    SeModel: Option<SentimentModel>
}

pub async fn index() -> impl Responder {
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
    } else if lang == "Spanish" {
        rust_bert::pipelines::translation::Language::Spanish
    } else if lang == "German" {
        rust_bert::pipelines::translation::Language::German
    } else if lang == "French" {
        rust_bert::pipelines::translation::Language::French
    } else {
        rust_bert::pipelines::translation::Language::English
    }
}

async fn translate(q: web::Json<TQuery>, data: web::Data<Mutex<AppData>>) -> HttpResponse {
    let data = data.lock().unwrap();
    let query = format!("{}", q.query);
    let target = format!("{}", q.target);

    if query == "".to_string() {
        return HttpResponse::Ok().body("Enter phrase");
    }

    if query.len() < 3 {
        return HttpResponse::Ok().body("Enter phrase");
    }

    let inferred_language = infer_input(&query).await;
    let source_language = convert_lang(inferred_language).await;
    let target_language = convert_lang(target).await;

    if source_language == target_language {
        return HttpResponse::Ok().body(query);
    }

    match &data.TModel {
        Some(translation_model) => {
            let output = translation_model
                .translate(&[query], source_language, target_language)
                .unwrap();
            let res = &output[0];

            HttpResponse::Ok().body(res.to_string())
        }
        None => HttpResponse::Ok().body("CANNOT ACCESS TRANSLATIONAL MODEL".to_string()),
    }
}

async fn summarize(q: web::Json<SQuery>, data: web::Data<Mutex<AppData>>) -> HttpResponse {
    let data = data.lock().unwrap();
    let query = format!("{}", q.query);

    if query == "".to_string() {
        return HttpResponse::Ok().body("Enter text");
    }

    match &data.SModel {
        Some(summarization_model) => {
            let output = summarization_model.summarize(&[query]);
            let res = &output[0];
            HttpResponse::Ok().body(res.to_string())
        }
        None => HttpResponse::Ok().body("CANNOT ACCESS SUMMARIZATION MODEL".to_string())
    }
}

async fn sentiment(q: web::Json<SeQuery>, data: web::Data<Mutex<AppData>>) -> HttpResponse {
    let data = data.lock().unwrap();
    let query = format!("{}", q.query);
    
    if query == "".to_string() {
        return HttpResponse::Ok().body("Enter text");
    }
    
    match &data.SeModel {
        Some(sentiment_model) => {
            let output = sentiment_model.predict([query.as_ref()]);
            let res = &output[0];
            HttpResponse::Ok().body(serde_json::to_string(res).unwrap())
        }
        None => HttpResponse::Ok().body("CANNOT ACCESS SENTIMENT MODEL".to_string())
    }
}

pub fn get_translation_model() -> TranslationModel {
    let model_resource =
        Resource::Remote(RemoteResource::from_pretrained(T5ModelResources::T5_BASE));
    let config_resource =
        Resource::Remote(RemoteResource::from_pretrained(T5ConfigResources::T5_BASE));
    let vocab_resource =
        Resource::Remote(RemoteResource::from_pretrained(T5VocabResources::T5_BASE));
    let merges_resource =
        Resource::Remote(RemoteResource::from_pretrained(T5VocabResources::T5_BASE));

    let source_languages = [
        rust_bert::pipelines::translation::Language::English,
        rust_bert::pipelines::translation::Language::French,
        rust_bert::pipelines::translation::Language::German,
        rust_bert::pipelines::translation::Language::Spanish,
    ];
    let target_languages = [
        rust_bert::pipelines::translation::Language::English,
        rust_bert::pipelines::translation::Language::French,
        rust_bert::pipelines::translation::Language::German,
        rust_bert::pipelines::translation::Language::Spanish,
    ];

    let translation_config = TranslationConfig::new(
        ModelType::T5,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
        source_languages,
        target_languages,
        Device::cuda_if_available(),
    );
    TranslationModel::new(translation_config).unwrap()
}

pub fn get_summarization_model() -> SummarizationModel {
    SummarizationModel::new(Default::default()).unwrap()
}

pub fn get_sentiment_model() -> SentimentModel {
    SentimentModel::new(Default::default()).unwrap()
}

async fn model_service(t: web::Json<ServiceType>, data: web::Data<Mutex<AppData>>) -> HttpResponse {
    let mut data = data.lock().unwrap();
    let service = format!("{}", t.service);
    match service.as_str() {
        "Translation" => {
            // data.SModel = None;
            match &data.TModel {
                Some(_) => {}
                None => {
                    data.TModel = Some(
                        spawn_blocking(move || get_translation_model())
                            .await
                            .unwrap(),
                    );
                }
            }
            HttpResponse::Ok().body("Translation Model Created/Already Exists".to_string())
        }

        "Summarization" => {
            // data.TModel = None;
            match &data.SModel {
                Some(_) => {}
                None => {
                    data.SModel = Some(
                        spawn_blocking(move || get_summarization_model())
                            .await
                            .unwrap(),
                    );
                }
            }
            HttpResponse::Ok().body("OK".to_string())
        }

        "Sentiment" => {
            match &data.SeModel {
                Some(_) => {},
                None => {
                    data.SeModel = Some(
                        spawn_blocking(move || get_sentiment_model())
                            .await
                            .unwrap(),
                        );
                }
            }
            HttpResponse::Ok().body("OK".to_string())
        }

        "None" => {
            // data.TModel = None;
            HttpResponse::Ok().body("OK".to_string())
        }

        _ => HttpResponse::Ok().body("NO MODEL CREATED".to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let data = web::Data::new(Mutex::new(AppData {
        TModel: None,
        SModel: None,
        SeModel: None
    }));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(data.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/service").route(web::post().to(model_service)))
            .service(web::resource("/predict").route(web::post().to(translate)))
            .service(web::resource("/summarize").route(web::post().to(summarize)))
            .service(web::resource("/sentiment").route(web::post().to(sentiment)))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
