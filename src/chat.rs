
use eventsource_client as es;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]