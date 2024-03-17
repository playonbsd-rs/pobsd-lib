//! Provide a Status enum and a GameStatus struct to accurately
//! represents the status of a Game according to the value of
//! the status field in the OpenBSD-Game-Database.
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Represent the status of a Game
pub enum Status {
    #[default]
    /// Status not provided.
    Unknown,
    /// Doesn't run.
    DoesNotRun,
    /// Game launches (not enough information to comment meaningfully
    /// on status beyond launching the game)
    Launches,
    /// Major bugs: potentially game-breaking, making finishing the game
    /// impossible or a chore; noticeably degrading the enjoyment
    /// compared to running the game on other platforms.
    MajorBugs,
    /// Medium-impact bugs: noticeable, but not game-breaking.
    MediumImpact,
    /// Minor bugs: barely noticeable, or not relevant to core game.
    MinorBugs,
    /// Completable: game can be played through until the credits roll,
    /// without major bugs (category 2); doesn't (necessarily) include
    /// optional side content, DLC, optional multiplayer, achievements etc.
    Completable,
    /// 100%: the complete game including optional content like DLC, side
    /// quests, multiplayer can be enjoyed.
    Perfect,
}

impl Into<GameStatus> for Status {
    fn into(self) -> GameStatus {
        GameStatus {
            status: self,
            message: None,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Represent the GameStatus comprising the Status itself plus
/// the additional comment.
pub struct GameStatus {
    /// Status of the Game.
    pub status: Status,
    /// Additional comment.
    pub message: Option<String>,
}

impl GameStatus {
    /// Create a new GameStatus provided a Status and an optional
    /// comment.
    pub fn new(status: Status, message: Option<String>) -> Self {
        Self { status, message }
    }
    /// Create a new GameStatus provided a &str representing the value
    /// of the Status field in the OpenBSD-Game-Database.
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

impl AsRef<Status> for GameStatus {
    fn as_ref(&self) -> &Status {
        &self.status
    }
}

impl Into<Status> for GameStatus {
    fn into(self) -> Status {
        self.status
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
