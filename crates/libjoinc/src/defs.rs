use serde_repr::Deserialize_repr;
use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, Deserialize_repr)]
#[repr(i8)]
pub enum ActiveTaskState {
    Uninitialized,
    Executing,
    Exited,
    WasSignaled,
    ExitUnknown,
    AbortPending,
    Aborted,
    CouldntStart,
    QuitPending,
    Suspended,
    CopyPending,
    #[serde(other)]
    UnknownToJoinc = -1,
}

impl std::default::Default for ActiveTaskState {
    fn default() -> Self {
        ActiveTaskState::Uninitialized
    }
}

impl Display for ActiveTaskState {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            ActiveTaskState::Uninitialized => "Uninitialized",
            ActiveTaskState::Executing => "Executing",
            ActiveTaskState::Suspended => "Suspended",
            ActiveTaskState::AbortPending => "AbortPending",
            ActiveTaskState::Exited => "Exited",
            ActiveTaskState::WasSignaled => "WasSignaled",
            ActiveTaskState::ExitUnknown => "EXIT_Unknown",
            ActiveTaskState::Aborted => "Aborted",
            ActiveTaskState::CouldntStart => "CouldntStart",
            ActiveTaskState::QuitPending => "QuitPending",
            ActiveTaskState::CopyPending => "CopyPending",
            ActiveTaskState::UnknownToJoinc => "Unknown",
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize_repr)]
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

#[derive(Clone, Copy, Debug, Deserialize_repr)]
#[repr(i8)]
pub enum ResultClientState {
    New,
    FilesDownloading,
    FilesDownloaded,
    ComputeError,
    FilesUploading,
    FilesUploaded,
    Aborted,
    UploadFailed,
    #[serde(other)]
    UnknownToJoinc = -1,
}

impl std::default::Default for ResultClientState {
    fn default() -> Self {
        ResultClientState::New
    }
}

impl Display for ResultClientState {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            ResultClientState::New => "new",
            ResultClientState::FilesDownloading => "downloading",
            ResultClientState::FilesDownloaded => "downloaded",
            ResultClientState::ComputeError => "compute error",
            ResultClientState::FilesUploading => "uploading",
            ResultClientState::FilesUploaded => "uploaded",
            ResultClientState::Aborted => "aborted",
            ResultClientState::UploadFailed => "upload failed",
            ResultClientState::UnknownToJoinc => "unknown",
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize_repr)]
#[repr(i8)]
pub enum SchedulerState {
    Uninitialized,
    Preempted,
    Scheduled,
    #[serde(other)]
    UnknownToJoinc = -1,
}

impl std::default::Default for SchedulerState {
    fn default() -> Self {
        SchedulerState::Uninitialized
    }
}

impl Display for SchedulerState {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            SchedulerState::Uninitialized => "uninitialized",
            SchedulerState::Preempted => "preempted",
            SchedulerState::Scheduled => "scheduled",
            SchedulerState::UnknownToJoinc => "unknown",
        })
    }
}
