use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::{NewNote, Note};

pub async fn create_note(
    pool: web::Data<SqlitePool>,
    new_note: web::Json<NewNote>,
) -> impl Responder {
    let result  = sqlx::query!(
        "INSERT INTO notes (title, content) VALUES (?, ?)",
        new_note.title,
        new_note.content,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(new_note.into_inner()),
        Err(e) => {
            eprintln!("Failed to create note: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_notes(
    pool: web::Data<SqlitePool>
) -> impl Responder {
    let notes = sqlx::query_as!(Note, "SELECT * FROM notes")
        .fetch_all(pool.get_ref())
        .await;

    match notes {
        Ok(notes) => {
            HttpResponse::Ok().json(notes)
        },
        Err(e) => {
            eprintln!("Failed to fetch notes: {}", e);
            HttpResponse::InternalServerError().finish()        
        }
    }
}

pub async fn get_note_by_id(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>
) -> impl Responder {
    let id = *path;
    let note = sqlx::query_as!(Note, "SELECT id, title, content FROM notes WHERE id = ?", id)
        .fetch_optional(pool.get_ref())
        .await;

    match note {
        Ok(Some(note)) => HttpResponse::Ok().json(note),
        Ok(None) => HttpResponse::NotFound().body("Note not found"),
        Err(e) => {
            eprintln!("Failed to fetch note: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_note(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    updated_note: web::Json<NewNote>,
) -> impl Responder {

    let id = *path;
    let updated_note = updated_note.into_inner();
    let existing_note = sqlx::query_as!(Note, "SELECT id, title, content FROM notes WHERE id = ?", id)
        .fetch_optional(pool.get_ref())
        .await;

    match existing_note {
        Ok(Some(_note)) => {
            let result = sqlx::query!(
                "UPDATE notes SET title = ?, content = ? WHERE id = ?",
                updated_note.title,
                updated_note.content,
                id,
        )
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(updated_note), // Return the updated note as JSON
                Err(e) => {
                    eprintln!("Failed to update note: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Note not found"),
        Err(e) => {
            eprintln!("Failed to fetch note: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_note(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = *path;
    let result = sqlx::query_as!(Note, "SELECT id, title, content FROM notes WHERE id = ?", id)
        .fetch_optional(pool.get_ref())
        .await;
    match result {
        Ok(Some(_note)) => {
            let result = sqlx::query!("DELETE FROM notes WHERE id = ?", id)
                .execute(pool.get_ref())
                .await;
            match result {
                Ok(_) => HttpResponse::Ok().body("Note Deleted"),
                Err(e) => {
                    eprintln!("Failed to delete note: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Note not found"),
        Err(e) => {
            eprintln!("Failed to fetch note: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
