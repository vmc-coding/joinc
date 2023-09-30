use serde_repr::Deserialize_repr;
use std::fmt::{self, Display};

#[derive(Deserialize_repr)]
#[repr(i8)]
pub enum MsgInfo {
    Info = 1,
    UserAlert,
    InternalError,
    #[serde(other)]
    UnknownToJoinc = -1,
}

impl Display for MsgInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            MsgInfo::Info => "low",
            MsgInfo::UserAlert => "user notification",
            MsgInfo::InternalError => "internal error",
            MsgInfo::UnknownToJoinc => "unknown",
        })
    }
}
