use std::sync::Mutex;

use actix_multipart::{Field, Multipart};
use actix_web::{
    get, post,
    web::{self, scope, Data, Query, ServiceConfig},
    HttpResponse,
};
use futures_util::TryStreamExt;
use log::error;
use serde::Deserialize;
use snowflake::SnowflakeIdGenerator;
use sqlx::PgPool;
use tokio::{
    fs::{remove_file, File},
    io::AsyncWriteExt,
};

use crate::{
    error::errors::KekServerError,
    middleware::auth_middleware::AuthService,
    models::{
        ids::{SoundFileId, UserId},
        sound_file::{SoundFile, MAX_LIMIT},
    },
    utils::auth::AuthorizedUserExt,
};
use lazy_static::lazy_static;

lazy_static! {
    // TODO: Make nicer
    static ref MAX_FILE_SIZE: usize = dotenv::var("MAX_FILE_SIZE")
        .unwrap_or_else(|_| 10_000_000.to_string())
        .parse()
        .unwrap_or(10_000_000);
}

const PUBLIC_SUFFIX: &str = "_p";

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/files")
            .wrap(AuthService)
            .service(upload_file)
            .service(get_public_files),
    );
}

async fn delete_file(sound_file: SoundFile) -> Result<(), KekServerError> {
    let full_file_path = format!("{}{}", dotenv::var("SOUNDFILE_DIR")?, sound_file.id.0);
    return Ok(remove_file(full_file_path).await?);
}

async fn validate_audio_mime(sound_file: &SoundFile) -> Result<(), KekServerError> {
    let full_file_path = format!("{}{}", dotenv::var("SOUNDFILE_DIR")?, sound_file.id.0);
    let mime = web::block(move || infer::get_from_path(full_file_path)).await??;

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
    let display_name = field.name().trim().to_string();

    if !display_name.is_empty() {
        return display_name;
    } else if let Some(name) = field.content_disposition().get_filename() {
        return name.trim().to_string();
    } else if let Some(name) = field.content_disposition().get_filename_ext() {
        return name.to_string();
    } else {
        return "".to_string();
    }
}

async fn insert_valid_files(
    files: Vec<SoundFile>,
    db_pool: Data<PgPool>,
) -> Result<Vec<SoundFile>, KekServerError> {
    let mut uploaded = Vec::with_capacity(files.len());
    let mut transaction = db_pool.begin().await?;
    for file in files {
        match validate_audio_mime(&file).await {
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

    if uploaded.is_empty() {
        return Err(KekServerError::NoFilesUploadedError);
    }

    return Ok(uploaded);
}

fn is_file_public(file_name: &str) -> bool {
    return file_name.ends_with(PUBLIC_SUFFIX);
}

fn create_new_file(id: i64, user_id: UserId, field: &Field) -> SoundFile {
    let mut name = parse_display_name(field);
    let is_public = is_file_public(&name);
    name.truncate(name.len() - PUBLIC_SUFFIX.len());
    return SoundFile::new(SoundFileId(id as u64), name, user_id, is_public);
}

// TODO: full path code repeats, make nicer
#[post("/upload")]
pub async fn upload_file(
    mut payload: Multipart,
    snowflake: Data<Mutex<SnowflakeIdGenerator>>,
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    let mut uploaded_files_size = 0;
    let mut max_file_size_exceeded = false;
    let mut files: Vec<SoundFile> = Vec::new();

    while let Some(mut field) = payload.try_next().await? {
        if mime::AUDIO != field.content_type().type_() {
            continue;
        }

        let id;
        {
            let mut lock = snowflake.lock().unwrap();
            id = lock.generate();
        }

        let sound_file = create_new_file(id, authorized_user.discord_user.id.clone(), &field);
        let full_file_path = format!("{}{}", dotenv::var("SOUNDFILE_DIR")?, sound_file.id.0);
        let mut file_handle = File::create(full_file_path).await?;

        files.push(sound_file);

        while let Some(chunk) = field.try_next().await? {
            uploaded_files_size += chunk.len();
            if uploaded_files_size > *MAX_FILE_SIZE {
                max_file_size_exceeded = true;
                break;
            }
            file_handle.write_all(&chunk).await?;
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

#[derive(Debug, Deserialize)]
pub struct PublicFilesQueryParams {
    limit: Option<i64>,
    page: Option<i64>,
}

#[get("/public")]
pub async fn get_public_files(
    Query(query): Query<PublicFilesQueryParams>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;
    let files = SoundFile::get_public_files(
        query.limit.unwrap_or(MAX_LIMIT),
        query.page.unwrap_or(1),
        &mut transaction,
    )
    .await?;
    transaction.commit().await?;
    return Ok(HttpResponse::Ok().json(files));
}
