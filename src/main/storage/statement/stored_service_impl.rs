use log::error;
use crate::model::Event;
use crate::storage::get_pool;
use crate::storage::statement::stored_service::StoredService;
use sqlx::Row;
use sqlx::types::Json;

pub struct StoredServiceImpl;

impl StoredService for StoredServiceImpl {

    /// Save an event to the storage.
    ///
    /// # Returns
    /// - `Some(true)` if the event was successfully saved.
    /// - `Some(false)` if the save operation failed (e.g., duplicate key).
    /// - `None` for unexpected errors.
    async fn save(&self, event: Event) -> Option<bool> {
        let query = "INSERT INTO event (id, timestamp, kind, tags) VALUES ($1, $2, $3, $4)";
        match sqlx::query(query)
            .bind(&event.id)
            .bind(event.timestamp as i64)
            .bind(event.kind as i32)
            .bind(Json(&event.tags))
            .execute(get_pool())
            .await
        {
            Ok(_) => Some(true),
            Err(e) => {
                error!("Failed to save event: {:?}", e);

                // Handle a specific case (e.g., duplicate key) as false
                if e.as_database_error()
                    .map_or(false, |db_err| db_err.code() == Some("23505".into()))
                {
                    Some(false)
                } else {
                    None // Unexpected error
                }
            },
        }
    }

    /// Delete an event from the storage.
    ///
    /// # Returns
    /// - `Some(true)` if the event was successfully deleted.
    /// - `Some(false)` if no event was deleted (e.g., ID not found).
    /// - `None` for unexpected errors.
    async fn delete(&self, event: Event) -> Option<bool> {
        let query = "DELETE FROM events WHERE id = $1";
        match sqlx::query(query)
            .bind(&event.id)
            .execute(get_pool())
            .await
        {
            Ok(result) => Some(result.rows_affected() > 0),
            Err(e) => {
                error!("Failed to delete event: {:?}", e);
                None // Unexpected error
            },
        }
    }

    /// Select an event from the storage by ID.
    ///
    /// # Returns
    /// - `Some(Event)` if the event was found.
    /// - `None` if the event was not found or an error occurred.
    async fn select(&self, id: String) -> Option<Event> {
        let query = "SELECT id, timestamp, kind, tags FROM events WHERE id = $1";
        match sqlx::query(query)
            .bind(&id)
            .fetch_one(get_pool())
            .await
        {
            Ok(row) => Some(Event {
                id: row.get("id"),
                timestamp: row.get::<i64, _>("timestamp") as u64,
                kind: row.get::<i32, _>("kind") as u32,
                tags: serde_json::from_str(&row.get::<String, _>("tags")).unwrap_or_default(),
            }),
            Err(e) => {
                error!("Failed to fetch event: {:?}", e);
                None // Return None if there is an error
            },
        }
    }
}