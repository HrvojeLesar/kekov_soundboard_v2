use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

use actix_multipart::{Field, Multipart};
use actix_web::{
    post,
    web::{self, scope, Data, ServiceConfig},
    HttpResponse,
};
use futures_util::TryStreamExt;
use log::error;
use snowflake::SnowflakeIdGenerator;
use sqlx::PgPool;

use crate::{
    error::errors::KekServerError, middleware::auth_middleware::AuthService,
    models::sound_file::SoundFile, utils::auth::AuthorizedUser,
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
    web::block(move || std::fs::remove_file(sound_file.get_id().to_string())).await??;
    return Ok(());
}

async fn validate_audio_mime(sound_file: Arc<SoundFile>) -> Result<(), KekServerError> {
    let mime = web::block(move || infer::get_from_path(sound_file.get_id().to_string())).await??;

    let mime = match mime {
        Some(m) => m,
        None => return Err(KekServerError::UnableToGetMimeError),
    };

    if mime.matcher_type() != infer::MatcherType::Audio {
        return Err(KekServerError::WrongMimeTypeError);
    }

    return Ok(());
}

fn parse_display_name(field: &Field) -> String {
    let display_name = field.name().trim();

    if display_name != "" {
        return display_name.to_string();
    } else if let Some(name) = field.content_disposition().get_filename() {
        return name.to_string();
    } else if let Some(name) = field.content_disposition().get_filename_ext() {
        return name.to_string();
    } else {
        return "".to_string();
    }
}

async fn insert_valid_files(
    files: Vec<Arc<SoundFile>>,
    db_pool: Data<PgPool>,
) -> Result<Vec<Arc<SoundFile>>, KekServerError> {
    let mut uploaded = Vec::with_capacity(files.len());
    let mut transaction = db_pool.begin().await?;
    for file in files {
        match validate_audio_mime(Arc::clone(&file)).await {
            Ok(_) => {
                file.insert(&mut transaction).await?;
                uploaded.push(file);
            }
            Err(e) => {
                error!("{}", e);
                delete_file(file).await?;
            }
        }
    }
    transaction.commit().await?;

    if uploaded.len() == 0 {
        return Err(KekServerError::NoFilesUploadedError);
    }

    return Ok(uploaded);
}

// TODO: Save file to some disk location
// instead of directory of running process
#[post("/upload")]
pub async fn upload_file(
    mut payload: Multipart,
    snowflake: Data<Mutex<SnowflakeIdGenerator>>,
    user: AuthorizedUser,
    db_pool: Data<PgPool>,
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

        let sound_file = Arc::new(SoundFile::new(
            id,
            parse_display_name(&field),
            *user.get_discord_user().get_id(),
        ));
        files.push(Arc::clone(&sound_file));

        let moved_file = Arc::clone(&sound_file);
        let mut file_handle =
            web::block(move || File::create(&*moved_file.get_id().to_string())).await??;

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

    let uploaded_files = insert_valid_files(files, db_pool).await?;

    return Ok(HttpResponse::Ok().json(uploaded_files));
}
