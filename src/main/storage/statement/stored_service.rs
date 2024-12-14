use crate::model::Event;

pub trait StoredService {

    /// Save an event to the storage.
    ///
    /// # Parameters
    /// - `event`: The event to be saved.
    ///
    /// # Returns
    /// - `Some(true)` if the event was successfully saved.
    /// - `Some(false)` if the event could not be saved (e.g., duplicate key).
    /// - `None` if an unexpected error occurred.
    async fn save(&self, event: Event) -> Option<bool>;

    /// Delete an event from the storage.
    ///
    /// # Parameters
    /// - `event`: The event to be deleted.
    ///
    /// # Returns
    /// - `Some(true)` if the event was successfully deleted.
    /// - `Some(false)` if the event was not found.
    /// - `None` if an unexpected error occurred.
    async fn delete(&self, event: Event) -> Option<bool>;

    /// Select an event from the storage by ID.
    ///
    /// # Parameters
    /// - `id`: The ID of the event to select.
    ///
    /// # Returns
    /// - `Some(Event)` if the event with the given ID was found.
    /// - `None` if the event was not found or if an error occurred.
    async fn select(&self, id: String) -> Option<Event>;

}