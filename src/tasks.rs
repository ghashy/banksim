use time::OffsetDateTime;
use uuid::Uuid;

use crate::RemovableById;

pub fn wait_and_remove(
    object: impl RemovableById + Send + 'static,
    id: Uuid,
    created_at: OffsetDateTime,
) {
    tokio::spawn(async move {
        let interval =
            (created_at + time::Duration::hours(1)) - OffsetDateTime::now_utc();
        // Sleeping
        match interval.try_into() {
            Ok(duration) => {
                tokio::time::sleep(duration).await;
            }
            Err(e) => {
                tracing::error!("Failed to calculate std::time::Duration from time::Duration: {e}")
            }
        }
        // Removing payment
        match object.remove(id) {
            Ok(()) => {
                tracing::info!(
                    "Object with id: {id} is removed after {interval} time!"
                )
            }
            Err(e) => {
                tracing::error!(
                    "Failed to remove object with id: {id}, error: {e}"
                )
            }
        }
    });
}
