use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FriendStatus {
    Pending,
    Accepted,
    Rejected,
    Blocked,
}

impl FriendStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            FriendStatus::Pending => "pending",
            FriendStatus::Accepted => "accepted",
            FriendStatus::Rejected => "rejected",
            FriendStatus::Blocked => "blocked",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "accepted" => Some(Self::Accepted),
            "rejected" => Some(Self::Rejected),
            "blocked" => Some(Self::Blocked),
            _ => None,
        }
    }
}
