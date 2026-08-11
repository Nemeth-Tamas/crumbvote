use crate::{
    AppState, ENTRY_IMAGE_DIRECTORY,
    admin::{ApiError, api_error, require_authenticated},
};
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, Path, State},
    http::StatusCode,
    routing::{get, patch, post},
};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use std::path::Path as FilePath;

const MAX_NAME_CHARACTERS: usize = 120;
const MAX_DESCRIPTION_CHARACTERS: usize = 2000;

const MAX_IMAGE_BYTES: usize = 8 * 1024 * 1024;

const MAX_IMAGE_UPLOAD_BODY_BYTES: usize = 9 * 1024 * 1024;

const IMAGE_RANDOM_BYTES: usize = 16;

#[derive(Deserialize)]
struct CreateEntryRequest {
    name: String,
    description: Option<String>,
}

#[derive(Deserialize)]
struct UpdateEntryRequest {
    name: String,
    description: Option<String>,
}

#[derive(Serialize)]
struct EntryResponse {
    id: i32,
    event_id: i32,
    number: i32,
    name: String,
    description: Option<String>,
    image_url: Option<String>,
    created_at: i64,
    updated_at: i64,
}

impl From<crumbvote_database::EntryModel> for EntryResponse {
    fn from(entry: crumbvote_database::EntryModel) -> Self {
        Self {
            id: entry.id,
            event_id: entry.event_id,
            number: entry.number,
            name: entry.name,
            description: entry.description,
            image_url: entry
                .image_filename
                .map(|filename| format!("/media/entries/{filename}")),
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/events/{event_id}/entries",
            get(list_entries).post(create_entry),
        )
        .route("/events/{event_id}/entries/{entry_id}", patch(update_entry))
        .route(
            "/events/{event_id}/entries/{entry_id}/image",
            post(upload_entry_image).layer(DefaultBodyLimit::max(MAX_IMAGE_UPLOAD_BODY_BYTES)),
        )
}

async fn list_entries(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(event_id): Path<i32>,
) -> Result<Json<Vec<EntryResponse>>, ApiError> {
    require_authenticated(&state, &jar).await?;

    require_event(&state, event_id).await?;

    let entries = crumbvote_database::list_entries(&state.database, event_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to list entries: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    Ok(Json(entries.into_iter().map(EntryResponse::from).collect()))
}

async fn create_entry(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(event_id): Path<i32>,
    Json(request): Json<CreateEntryRequest>,
) -> Result<(StatusCode, Json<EntryResponse>), ApiError> {
    require_authenticated(&state, &jar).await?;

    let event = require_event(&state, event_id).await?;

    if event.status != "draft" {
        return Err(api_error(StatusCode::CONFLICT, "event_entries_locked"));
    }

    let name = request.name.trim().to_owned();

    validate_name(&name)?;

    let description = request
        .description
        .map(|description| description.trim().to_owned())
        .filter(|description| !description.is_empty());

    validate_description(&description)?;

    let entry = crumbvote_database::create_entry(&state.database, event_id, name, description)
        .await
        .map_err(|error| {
            eprintln!("Failed to create entry: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    Ok((StatusCode::CREATED, Json(EntryResponse::from(entry))))
}

async fn update_entry(
    State(state): State<AppState>,
    jar: CookieJar,
    Path((event_id, entry_id)): Path<(i32, i32)>,
    Json(request): Json<UpdateEntryRequest>,
) -> Result<Json<EntryResponse>, ApiError> {
    require_authenticated(&state, &jar).await?;

    let event = require_event(&state, event_id).await?;

    if event.status != "draft" {
        return Err(api_error(StatusCode::CONFLICT, "event_entries_locked"));
    }

    require_entry(&state, event_id, entry_id).await?;

    let name = request.name.trim().to_owned();

    validate_name(&name)?;

    let description = request
        .description
        .map(|description| description.trim().to_owned())
        .filter(|description| !description.is_empty());

    validate_description(&description)?;

    let updated = crumbvote_database::update_entry(&state.database, entry_id, name, description)
        .await
        .map_err(|error| {
            eprintln!("Failed to update entry: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    let Some(updated) = updated else {
        return Err(api_error(StatusCode::NOT_FOUND, "entry_not_found"));
    };

    Ok(Json(EntryResponse::from(updated)))
}

async fn upload_entry_image(
    State(state): State<AppState>,
    jar: CookieJar,
    Path((event_id, entry_id)): Path<(i32, i32)>,
    mut multipart: Multipart,
) -> Result<Json<EntryResponse>, ApiError> {
    require_authenticated(&state, &jar).await?;

    let event = require_event(&state, event_id).await?;

    if event.status != "draft" {
        return Err(api_error(StatusCode::CONFLICT, "event_entries_locked"));
    }

    let entry = require_entry(&state, event_id, entry_id).await?;

    let mut uploaded_image = None;

    while let Some(field) = multipart.next_field().await.map_err(|error| {
        eprintln!("Failed to parse image upload: {error}");

        api_error(StatusCode::BAD_REQUEST, "invalid_image_upload")
    })? {
        if field.name() != Some("image") {
            continue;
        }

        let content_type = field
            .content_type()
            .map(str::to_owned)
            .ok_or_else(|| api_error(StatusCode::BAD_REQUEST, "unsupported_image_type"))?;

        let extension = image_extension(&content_type)
            .ok_or_else(|| api_error(StatusCode::BAD_REQUEST, "unsupported_image_type"))?;

        let bytes = field.bytes().await.map_err(|error| {
            eprintln!("Failed to read uploaded image: {error}");

            api_error(StatusCode::BAD_REQUEST, "invalid_image_upload")
        })?;

        if bytes.is_empty() {
            return Err(api_error(StatusCode::BAD_REQUEST, "image_empty"));
        }

        if bytes.len() > MAX_IMAGE_BYTES {
            return Err(api_error(StatusCode::PAYLOAD_TOO_LARGE, "image_too_large"));
        }

        if !image_bytes_match_extension(extension, &bytes) {
            return Err(api_error(StatusCode::BAD_REQUEST, "invalid_image_data"));
        }

        uploaded_image = Some((extension, bytes));

        break;
    }

    let Some((extension, bytes)) = uploaded_image else {
        return Err(api_error(StatusCode::BAD_REQUEST, "image_required"));
    };

    let filename = generate_image_filename(extension).map_err(|error| {
        eprintln!("Failed to generate image filename: {error}");

        api_error(StatusCode::INTERNAL_SERVER_ERROR, "image_storage_error")
    })?;

    let image_path = FilePath::new(ENTRY_IMAGE_DIRECTORY).join(&filename);

    tokio::fs::write(&image_path, &bytes)
        .await
        .map_err(|error| {
            eprintln!("Failed to store entry image: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "image_storage_error")
        })?;

    let updated = match crumbvote_database::set_entry_image_filename(
        &state.database,
        entry_id,
        Some(filename.clone()),
    )
    .await
    {
        Ok(Some(updated)) => updated,

        Ok(None) => {
            remove_image_file(&filename).await;

            return Err(api_error(StatusCode::NOT_FOUND, "entry_not_found"));
        }

        Err(error) => {
            remove_image_file(&filename).await;

            eprintln!("Failed to persist entry image: {error}");

            return Err(api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
            ));
        }
    };

    if let Some(previous_filename) = entry.image_filename {
        if previous_filename != filename {
            remove_image_file(&previous_filename).await;
        }
    }

    Ok(Json(EntryResponse::from(updated)))
}

async fn require_event(
    state: &AppState,
    event_id: i32,
) -> Result<crumbvote_database::EventModel, ApiError> {
    let event = crumbvote_database::event_by_id(&state.database, event_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load event: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    event.ok_or_else(|| api_error(StatusCode::NOT_FOUND, "event_not_found"))
}

async fn require_entry(
    state: &AppState,
    event_id: i32,
    entry_id: i32,
) -> Result<crumbvote_database::EntryModel, ApiError> {
    let entry = crumbvote_database::entry_by_id(&state.database, entry_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load entry: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    match entry {
        Some(entry) if entry.event_id == event_id => Ok(entry),

        _ => Err(api_error(StatusCode::NOT_FOUND, "entry_not_found")),
    }
}

fn image_extension(content_type: &str) -> Option<&'static str> {
    match content_type {
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/webp" => Some("webp"),
        _ => None,
    }
}

fn image_bytes_match_extension(extension: &str, bytes: &[u8]) -> bool {
    match extension {
        "jpg" => bytes.starts_with(&[0xff, 0xd8, 0xff]),

        "png" => bytes.starts_with(&[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),

        "webp" => bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP",

        _ => false,
    }
}

fn generate_image_filename(extension: &str) -> Result<String, getrandom::Error> {
    let mut random = [0_u8; IMAGE_RANDOM_BYTES];

    getrandom::fill(&mut random)?;

    Ok(format!("{}.{extension}", hex::encode(random),))
}

async fn remove_image_file(filename: &str) {
    let path = FilePath::new(filename);

    if path.file_name().and_then(|value| value.to_str()) != Some(filename) {
        eprintln!("Refusing to remove invalid image filename: {filename}");

        return;
    }

    let path = FilePath::new(ENTRY_IMAGE_DIRECTORY).join(filename);

    match tokio::fs::remove_file(path).await {
        Ok(()) => {}

        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}

        Err(error) => {
            eprintln!("Failed to remove old entry image: {error}");
        }
    }
}

fn validate_name(name: &str) -> Result<(), ApiError> {
    if name.is_empty() {
        return Err(api_error(StatusCode::BAD_REQUEST, "entry_name_required"));
    }

    if name.chars().count() > MAX_NAME_CHARACTERS {
        return Err(api_error(StatusCode::BAD_REQUEST, "entry_name_too_long"));
    }

    Ok(())
}

fn validate_description(description: &Option<String>) -> Result<(), ApiError> {
    if description
        .as_ref()
        .is_some_and(|description| description.chars().count() > MAX_DESCRIPTION_CHARACTERS)
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "entry_description_too_long",
        ));
    }

    Ok(())
}
