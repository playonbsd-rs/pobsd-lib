use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Status {
    #[default]
    Unknown,
    DoesNotRun,
    Launches,
    MajorBugs,
    MediumImpact,
    MinorBugs,
    Completable,
    Perfect,
}

#[derive(Clone, Debug, Default, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameStatus {
    pub status: Status,
    pub message: Option<String>,
}

impl GameStatus {
    pub fn new(status: Status, message: Option<String>) -> Self {
        Self { status, message }
    }
    pub fn from_line(line: &str) -> Self {
        if line.starts_with('0') {
            Self {
                status: Status::DoesNotRun,
                message: line.strip_prefix('0').map(|x| x.trim().into()),
            }
        } else if line.starts_with('1') {
            Self {
                status: Status::DoesNotRun,
                message: line.strip_prefix('1').map(|x| x.trim().into()),
            }
        } else if line.starts_with('2') {
            Self {
                status: Status::DoesNotRun,
                message: line.strip_prefix('2').map(|x| x.trim().into()),
            }
        } else if line.starts_with('3') {
            Self {
                status: Status::DoesNotRun,
                message: line.strip_prefix('3').map(|x| x.trim().into()),
            }
        } else if line.starts_with('4') {
            Self {
                status: Status::DoesNotRun,
                message: line.strip_prefix('4').map(|x| x.trim().into()),
            }
        } else if line.starts_with('5') {
            Self {
                status: Status::DoesNotRun,
                message: line.strip_prefix('5').map(|x| x.trim().into()),
            }
        } else if line.starts_with('6') {
            Self {
                status: Status::DoesNotRun,
                message: line.strip_prefix('6').map(|x| x.trim().into()),
            }
        } else {
            Self {
                status: Status::default(),
                message: None,
            }
        }
    }
}

impl AsRef<GameStatus> for GameStatus {
    fn as_ref(&self) -> &GameStatus {
        self
    }
}

impl PartialEq for GameStatus {
    fn eq(&self, other: &Self) -> bool {
        self.status.eq(&other.status)
    }
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            Status::Unknown => write!(f, ""),
            Status::DoesNotRun => write!(f, "0 {}", self.message.as_deref().unwrap_or("")),
            Status::Launches => write!(f, "1 {}", self.message.as_deref().unwrap_or("")),
            Status::MajorBugs => write!(f, "2 {}", self.message.as_deref().unwrap_or("")),
            Status::MediumImpact => write!(f, "3 {}", self.message.as_deref().unwrap_or("")),
            Status::MinorBugs => write!(f, "4 {}", self.message.as_deref().unwrap_or("")),
            Status::Completable => write!(f, "5 {}", self.message.as_deref().unwrap_or("")),
            Status::Perfect => write!(f, "6 {}", self.message.as_deref().unwrap_or("")),
        }
    }
}
