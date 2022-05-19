use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use lingua::Language::{English, French, German, Spanish};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::translation::{ TranslationConfig, TranslationModel, TranslationModelBuilder };
use rust_bert::pipelines::summarization:: SummarizationModel;
use rust_bert::resources::{ Resource, RemoteResource };
use rust_bert::t5::{T5ConfigResources, T5ModelResources, T5VocabResources};
use serde::{Deserialize, Serialize};
use std::sync:: { Arc, Mutex };
use tokio::task::spawn_blocking;
use tch::Device;

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    pub service: String,
}

pub struct AppData {
    TModel: Option<TranslationModel>,
    SModel: Option<SummarizationModel>
}


pub async fn index(data: web::Data<Mutex<AppData>>) -> impl Responder {
    let data = data.lock().unwrap();
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

async fn pred_from_query(q: web::Json<Query>, data: web::Data<Mutex<AppData>>) -> HttpResponse {
    let data = data.lock().unwrap();
    let query = format!("{}", q.query);

    if query == "".to_string() {
        return HttpResponse::Ok().body("Enter phrase");
    }

    if query.len() < 5 {
        return HttpResponse::Ok().body("Enter phrase");
    }

    let inferred_language = infer_input(&query).await;
    let source_language = convert_lang(inferred_language).await;
    
    // let model = data.TModel;
    match &data.TModel {
        Some(translation_model) => {
                        let output = translation_model.translate(
                            &[query],
                            source_language,
                            rust_bert::pipelines::translation::Language::French,
                        )
                        .unwrap();
                        let res = &output[0];

                        HttpResponse::Ok().body(res.to_string())
        }
        None => HttpResponse::Ok().body("ERROR".to_string())
    }
    
}

pub fn get_translation_model() -> TranslationModel {
    let model_resource = Resource::Remote(RemoteResource::from_pretrained(T5ModelResources::T5_BASE));
    let config_resource = Resource::Remote(RemoteResource::from_pretrained(T5ConfigResources::T5_BASE));
    let vocab_resource = Resource::Remote(RemoteResource::from_pretrained(T5VocabResources::T5_BASE));
    let merges_resource = Resource::Remote(RemoteResource::from_pretrained(T5VocabResources::T5_BASE));

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

    // TranslationModelBuilder::new()
    //     .with_model_type(ModelType::M2M100)
    //     .with_source_languages(vec![
    //         rust_bert::pipelines::translation::Language::English,
    //         rust_bert::pipelines::translation::Language::Spanish,
    //         rust_bert::pipelines::translation::Language::French,
    //         rust_bert::pipelines::translation::Language::German,
    //     ])
    //     .with_target_languages(vec![
    //         rust_bert::pipelines::translation::Language::Spanish,
    //         rust_bert::pipelines::translation::Language::French,
    //         rust_bert::pipelines::translation::Language::German,
    //     ])
    //     .create_model()
    //     .unwrap()

    // TranslationModelBuilder::new()
    // .with_medium_model()
    // .with_source_languages(vec![rust_bert::pipelines::translation::Language::English, rust_bert::pipelines::translation::Language::Spanish, rust_bert::pipelines::translation::Language::French, rust_bert::pipelines::translation::Language::German])
    // .with_target_languages(vec![rust_bert::pipelines::translation::Language::English, rust_bert::pipelines::translation::Language::Spanish, rust_bert::pipelines::translation::Language::French, rust_bert::pipelines::translation::Language::German])
    // .create_model().unwrap()
}

async fn model_service(t: web::Json<ServiceType>, data: web::Data<Mutex<AppData>>) -> HttpResponse {
    let mut data = data.lock().unwrap();
    let service = format!("{}", t.service);
    match service.as_str() {
        "Translation" => {
            data.SModel = None;
            match &data.TModel {
                Some(_) => {},
                None => {
                    data.TModel = Some(spawn_blocking(move || get_translation_model()).await.unwrap());
                }
            }
            // data.TModel = Some(spawn_blocking(move || get_translation_model()).await.unwrap());
            // drop(data);
            HttpResponse::Ok().body("Translation Model Created/Already Exists".to_string())
        },
        "Summarization" => {
            data.TModel = None;
            HttpResponse::Ok().body("OK".to_string())
        },
        "None" => {
            data.TModel = None;
            HttpResponse::Ok().body("OK".to_string())
        }
        _ => HttpResponse::Ok().body("NO MODEL CREATED".to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    // let model = spawn_blocking(move || get_translation_model()).await;
    // let data = web::Data::new(Mutex::new(AppData {
    //     Model: model.unwrap(),
    // }));
    let data = web::Data::new(Mutex::new(AppData {
        TModel: None,
        SModel: None
    }));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(data.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/service").route(web::post().to(model_service)))
            .service(web::resource("/predict").route(web::post().to(pred_from_query)))
            // .service(web::resource("/summarize").route(web::post().to(summarize_from_query)))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
