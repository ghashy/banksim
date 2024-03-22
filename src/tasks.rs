use time::OffsetDateTime;
use tokio::sync::oneshot::Receiver;
use uuid::Uuid;

use crate::RemovableById;

pub fn wait_hour_and_remove(
    mut object: impl RemovableById + Send + 'static,
    notifier: Receiver<()>,
    id: Uuid,
    created_at: OffsetDateTime,
) {
    tokio::spawn(async move {
        let interval =
            (created_at + time::Duration::hours(1)) - OffsetDateTime::now_utc();
        // Sleeping
        let duration = match interval.try_into() {
            Ok(duration) => duration,
            Err(e) => {
                tracing::error!("Failed to calculate std::time::Duration from time::Duration: {e}");
                return;
            }
        };
        tokio::select! {
            _ = tokio::time::sleep(duration) => {
                tracing::info!("Task on watching {id} time is out, removing!");
            }
            _ = notifier => {
                tracing::info!("Task on watching {id} entity got removing request!");
            }
        }

        // Removing payment
        match object.remove(id) {
            Ok(()) => {
                tracing::info!("Object with id: {id} is removed!")
            }
            Err(e) => {
                tracing::error!(
                    "Failed to remove object with id: {id}, error: {e}"
                )
            }
        }
    });
}
