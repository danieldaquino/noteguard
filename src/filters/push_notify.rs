use crate::{Action, InputMessage, NoteFilter, OutputMessage};
use serde::Deserialize;
use super::NotificationManager;
use std::thread;

#[derive(Deserialize)]
pub struct PushNotify {
    pub reject: bool,
}

impl NoteFilter for PushNotify {
    fn filter_note(&mut self, msg: &InputMessage) -> OutputMessage {
        // Clone event data and send it to the `notification_manager` to be processed.
        let event_data = msg.event.clone();
        thread::spawn(move || {
            // Attempt to create `notification_manager` and handle the possibility of an error.
            match NotificationManager::new(None, None) {
                Ok(notification_manager) => {
                        // Now, `notification_manager` is guaranteed to be successfully created.
                        notification_manager.send_notifications_if_needed(&event_data);
                }
                Err(e) => {
                    // Handle the error of creating the `notification_manager`.
                    // TODO: Log the error or handle it as needed.
                }
            }
        });
        
        if self.reject {
            return OutputMessage::new(
                msg.event.id.clone(),
                Action::Reject,
                Some("filter configured to reject all notes".to_string()),
            );
        } else {
            return OutputMessage::new(msg.event.id.clone(), Action::Accept, None);
        }
    }

    fn name(&self) -> &'static str {
        "push_notify"
    }
}
