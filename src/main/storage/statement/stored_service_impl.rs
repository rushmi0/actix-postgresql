use crate::model::Event;
use crate::storage::get_pool;
use crate::storage::statement::stored_service::StoredService;
use sqlx::Row;

pub struct StoredServiceImpl;

impl StoredService for StoredServiceImpl {

    async fn save(&self, event: Event) -> bool {
        let query = "INSERT INTO events (id, timestamp, kind, tags) VALUES ($1, $2, $3, $4)";
        let tags_json = serde_json::to_string(&event.tags).unwrap();
        match sqlx::query(query)
            .bind(&event.id)
            .bind(event.timestamp as i64)
            .bind(event.kind as i32)
            .bind(tags_json)
            .execute(get_pool())
            .await
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    async fn delete(&self, event: Event) -> bool {
        let query = "DELETE FROM events WHERE id = $1";
        match sqlx::query(query).bind(&event.id).execute(get_pool()).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    async fn select(&self, id: String) -> Event {
        let query = "SELECT id, timestamp, kind, tags FROM events WHERE id = $1";
        let row = sqlx::query(query)
            .bind(&id)
            .fetch_one(get_pool())
            .await
            .expect("Failed to fetch event");

        Event {
            id: row.get("id"),
            timestamp: row.get::<i64, _>("timestamp") as u64,
            kind: row.get::<i32, _>("kind") as u32,
            tags: serde_json::from_str(&row.get::<String, _>("tags")).unwrap(),
        }
    }
}
