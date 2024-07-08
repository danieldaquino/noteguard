use crate::{InputMessage, OutputMessage};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Note {
    pub id: String,
    pub pubkey: String,
    pub content: String,
    pub created_at: i64,
    pub kind: i64,
    pub tags: Vec<Vec<String>>,
    pub sig: String,
}

pub trait NoteFilter {
    fn filter_note(&mut self, msg: &InputMessage) -> OutputMessage;

    /// A key corresponding to an entry in the noteguard.toml file.
    fn name(&self) -> &'static str;
}

// This is a wrapper around the Event type from strfry-policies, which adds some useful methods
impl Note {
    /// Checks if the note references a given pubkey
    pub fn references_pubkey(&self, pubkey: &str) -> bool {
        self.referenced_pubkeys().contains(pubkey)
    }

    /// Retrieves a set of pubkeys referenced by the note
    pub fn referenced_pubkeys(&self) -> std::collections::HashSet<String> {
        std::collections::HashSet::from_iter(self.get_tags("p").iter().cloned())
    }

    /// Retrieves a set of pubkeys relevant to the note
    pub fn relevant_pubkeys(&self) -> std::collections::HashSet<String> {
        let mut pubkeys = self.referenced_pubkeys();
        pubkeys.insert(self.pubkey.clone());
        pubkeys
    }

    /// Retrieves a set of event IDs referenced by the note
    pub fn referenced_event_ids(&self) -> std::collections::HashSet<String> {
        std::collections::HashSet::from_iter(self.get_tags("e").iter().cloned())
    }

    /// Retrieves an array of tags of a specific type from the note
    pub fn get_tags(&self, tag_type: &str) -> Vec<String> {
        self.tags.iter()
            .filter(|tag_tuple| tag_tuple[0] == tag_type)
            .map(|tag_tuple| tag_tuple[1].clone())
            .collect()
    }
}
