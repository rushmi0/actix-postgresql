use crate::model::Event;

pub trait StoredService {

    /// Save an event to the storage.
    ///
    /// # Parameters
    /// - `event`: The event to be saved.
    ///
    /// # Returns
    /// - `true` if the event was successfully saved.
    /// - `false` otherwise.
    async fn save(&self, event: Event) -> bool;

    /// Delete an event from the storage.
    ///
    /// # Parameters
    /// - `event`: The event to be deleted.
    ///
    /// # Returns
    /// - `true` if the event was successfully deleted.
    /// - `false` otherwise.
    async fn delete(&self, event: Event) -> bool;

    /// Select an event from the storage by ID.
    ///
    /// # Parameters
    /// - `id`: The ID of the event to select.
    ///
    /// # Returns
    /// - The event with the given ID, if found.
    async fn select(&self, id: String) -> Event;

}
