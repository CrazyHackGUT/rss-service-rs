use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Feed {
    id: Option<u64>,
    url: String,

    // HomePage and title can be dynamically set via Feed URL.
    home_page: Option<String>,
    title: Option<String>,

    updated_at: Option<DateTime<Utc>>,
}
