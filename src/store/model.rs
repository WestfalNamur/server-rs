use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

//
//  Model
//
// Clone: We need to store but also send a copy to the client
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

// Only Deserialize as we only need to Deserialize the incoming JSON
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

//
//  Model Controller
//
#[derive(Clone)]
pub struct ModelController {
    // Clone, clones Arc and not the vector, the mutex protects the vector and makes access exclusive.
    // Arc:   A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’.
    // Mutex: A mutual exclusion primitive useful for protecting shared data
    // Option:  For prototyping only as do not "delete" but set to None
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

//  "CRUD"
impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone())); // Clone into store, original back to client;

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect(); // Clone tickets to return and keep the originals in the store;

        Ok(tickets)
    }
}
