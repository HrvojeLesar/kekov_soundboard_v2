use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

use actix_multipart::Multipart;
use actix_web::{
    post,
    web::{self, scope, Data, ServiceConfig},
    HttpResponse,
};
use futures_util::TryStreamExt;
use log::error;
use snowflake::SnowflakeIdGenerator;

use crate::{
    error::errors::KekServerError,
    middleware::auth_middleware::{self, AuthService},
    models::sound_file::SoundFile,
};
use lazy_static::lazy_static;

lazy_static! {
    // TODO: Make nicer
    static ref MAX_FILE_SIZE: usize = dotenv::var("MAX_FILE_SIZE")
        .unwrap_or(10_000_000.to_string())
        .parse()
        .unwrap_or(10_000_000);
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("files").wrap(AuthService).service(upload_file));
}

async fn delete_file(sound_file: Arc<SoundFile>) -> Result<(), KekServerError> {
    web::block(move || std::fs::remove_file(sound_file.get_file_name())).await??;
    return Ok(());
}

async fn validate_audio_mime(sound_file: Arc<SoundFile>) -> Result<(), KekServerError> {
    let mime = web::block(move || infer::get_from_path(sound_file.get_file_name())).await??;

    let mime = match mime {
        Some(m) => m,
        None => return Err(KekServerError::UnableToGetMimeError),
    };

    if mime.matcher_type() != infer::MatcherType::Audio {
        return Err(KekServerError::WrongMimeTypeError);
    }

    return Ok(());
}

#[post("/upload")]
pub async fn upload_file(
    mut payload: Multipart,
    snowflake: Data<Mutex<SnowflakeIdGenerator>>,
) -> Result<HttpResponse, KekServerError> {
    let mut uploaded_files_size = 0;
    let mut max_file_size_exceeded = false;
    let mut files: Vec<Arc<SoundFile>> = Vec::new();

    while let Some(mut field) = payload.try_next().await? {
        if mime::AUDIO != field.content_type().type_() {
            continue;
        }

        let id;
        {
            let mut lock = snowflake.lock().unwrap();
            id = lock.generate();
        }

        let sound_file = Arc::new(SoundFile::new(id.to_string(), field.name().to_string()));
        files.push(Arc::clone(&sound_file));

        let moved_file = Arc::clone(&sound_file);
        let mut file_handle =
            web::block(move || File::create(&*moved_file.get_file_name())).await??;

        while let Some(chunk) = field.try_next().await? {
            uploaded_files_size += chunk.len();
            if uploaded_files_size > *MAX_FILE_SIZE {
                max_file_size_exceeded = true;
                break;
            }
            file_handle =
                web::block(move || file_handle.write_all(&chunk).map(|_| file_handle)).await??;
        }
    }

    if max_file_size_exceeded {
        for file in files {
            delete_file(file).await?;
        }
        return Err(KekServerError::FileTooLargeError);
    }

    let mut successfully_uploaded = Vec::with_capacity(files.len());
    for file in files {
        match validate_audio_mime(Arc::clone(&file)).await {
            Ok(_) => successfully_uploaded.push(file),
            Err(e) => {
                error!("{}", e);
                delete_file(file).await?;
            }
        }
    }

    if successfully_uploaded.len() == 0 {
        return Err(KekServerError::NoFilesUploadedError);
    }

    // TODO: return nicer response (an actual response)
    // maybe with more info on failed files
    // or only successfully uploded ones
    return Ok(HttpResponse::Ok().json(successfully_uploaded));
}
