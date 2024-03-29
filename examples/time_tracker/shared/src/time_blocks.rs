use crate::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Client {
    pub id: ClientId,
    pub name: String,
    pub time_blocks: Vec<TimeBlock>,
    pub tracked: Wrapper<Duration>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct TimeBlock {
    pub id: TimeBlockId,
    pub name: String,
    pub status: TimeBlockStatus,
    pub duration: Wrapper<Duration>,
    pub invoice: Option<Invoice>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[serde(crate = "serde")]
pub enum TimeBlockStatus {
    NonBillable,
    Unpaid,
    Paid,
}

impl Default for TimeBlockStatus {
    fn default() -> Self {
        Self::Unpaid
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Invoice {
    pub id: InvoiceId,
    pub custom_id: String,
    pub url: String,
}
