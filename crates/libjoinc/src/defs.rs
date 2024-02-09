use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::fmt::{self, Display};

#[derive(Clone, Copy, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(from = "DeserializedBool")]
pub enum Bool {
    #[default]
    False,
    True,
}

impl From<Bool> for bool {
    fn from(b: Bool) -> Self {
        b == Bool::True
    }
}

impl From<bool> for Bool {
    fn from(b: bool) -> Self {
        match b {
            false => Bool::True,
            true => Bool::True,
        }
    }
}

impl Display for Bool {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            Bool::False => "no",
            Bool::True => "yes",
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize_repr, PartialEq, Eq)]
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
    #[default]
    #[serde(other)]
    UnknownToJoinc = -1,
}

impl Display for ActiveTaskState {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            ActiveTaskState::Uninitialized => "UNINITIALIZED",
            ActiveTaskState::Executing => "EXECUTING",
            ActiveTaskState::Suspended => "SUSPENDED",
            ActiveTaskState::AbortPending => "ABORT_PENDING",
            ActiveTaskState::Exited => "EXITED",
            ActiveTaskState::WasSignaled => "WAS_SIGNALED",
            ActiveTaskState::ExitUnknown => "EXIT_UNKNOWN",
            ActiveTaskState::Aborted => "ABORTED",
            ActiveTaskState::CouldntStart => "COULDNT_START",
            ActiveTaskState::QuitPending => "QUIT_PENDING",
            ActiveTaskState::CopyPending => "COPY_PENDING",
            ActiveTaskState::UnknownToJoinc => "UNKNOWN",
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize_repr, PartialEq, Eq)]
#[repr(i8)]
pub enum MsgInfo {
    Info = 1,
    UserAlert,
    InternalError,
    #[default]
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

#[derive(Clone, Copy, Debug, Default, Deserialize_repr, PartialEq, Eq)]
#[repr(i8)]
pub enum NetworkStatus {
    Online,
    WantConnection,
    WantDisconnect,
    LookupPending,
    #[default]
    #[serde(other)]
    UnknownToJoinc = -1,
}

impl Display for NetworkStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            NetworkStatus::Online => "online",
            NetworkStatus::WantConnection => "need connection",
            NetworkStatus::WantDisconnect => "don't need connection",
            NetworkStatus::LookupPending => "reference site lookup pending",
            NetworkStatus::UnknownToJoinc => "unknown",
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize_repr, PartialEq, Eq)]
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
    #[default]
    #[serde(other)]
    UnknownToJoinc = -1,
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

#[derive(Clone, Copy, Debug, Default, Deserialize_repr, PartialEq, Eq)]
#[repr(i8)]
pub enum RpcReason {
    None,
    UserReq,
    ResultsDue,
    NeedWork,
    TrickleUp,
    AcctMgrReq,
    Init,
    ProjectReq,
    #[default]
    #[serde(other)]
    UnknownToJoinc = -1
}

impl Display for RpcReason {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            RpcReason::None => "",
            RpcReason::UserReq => "Requested by user",
            RpcReason::ResultsDue => "To fetch work",
            RpcReason::NeedWork => "To report completed tasks",
            RpcReason::TrickleUp => "To send trickle-up message",
            RpcReason::AcctMgrReq => "Requested by account manager",
            RpcReason::Init => "Project initialization",
            RpcReason::ProjectReq => "Requested by project",
            RpcReason::UnknownToJoinc => "unknown",
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize_repr, PartialEq, Eq)]
#[repr(i8)]
pub enum RunMode {
    Always = 1,
    Auto,
    Never,
    Restore,
    #[default]
    #[serde(other)]
    UnknownToJoinc = -1
}

impl Display for RunMode {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            RunMode::Always => "always",
            RunMode::Auto => "according to prefs",
            RunMode::Never => "never",
            RunMode::Restore => "restore",
            RunMode::UnknownToJoinc => "unknown",
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize_repr, PartialEq, Eq)]
#[repr(i8)]
pub enum SchedulerState {
    Uninitialized,
    Preempted,
    Scheduled,
    #[default]
    #[serde(other)]
    UnknownToJoinc = -1,
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

#[derive(Clone, Copy, Debug, Default, Deserialize_repr, PartialEq, Eq)]
#[repr(i32)]
pub enum SuspendReason {
    NotSuspended = 0,
    Batteries = 1 << 0,
    UserActive = 1 << 1,
    UserReq = 1 << 2,
    TimeOfDay = 1 << 3,
    Benchmarks = 1 << 4,
    DiskSize = 1 << 5,
    CpuThrottle = 1 << 6,
    NoRecentInput = 1 << 7,
    InitialDelay = 1 << 8,
    ExclusiveAppRunning = 1 << 9,
    CpuUsage = 1 << 10,
    NetworkQuotaExceeded = 1 << 11,
    Os = 1 << 12,
    WifiState = (1 << 12) + 1,
    BatteryCharging = (1 << 12) + 2,
    BatteryOverheated = (1 << 12) + 3,
    NoGuiKeepalive = (1 << 12) + 4,
    #[default]
    #[serde(other)]
    UnknownToJoinc = -1,
}

impl Display for SuspendReason {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            SuspendReason::NotSuspended => "not suspended",
            SuspendReason::Batteries => "on batteries",
            SuspendReason::UserActive => "computer is in use",
            SuspendReason::UserReq => "user request",
            SuspendReason::TimeOfDay => "time of day",
            SuspendReason::Benchmarks => "CPU benchmarks in progress",
            SuspendReason::DiskSize => "need disk space - check preferences",
            SuspendReason::CpuThrottle => "CPU throttled",
            SuspendReason::NoRecentInput => "no recent user activity",
            SuspendReason::InitialDelay => "initial delay",
            SuspendReason::ExclusiveAppRunning => "an exclusive app is running",
            SuspendReason::CpuUsage => "CPU is busy",
            SuspendReason::NetworkQuotaExceeded => "network transfer limit exceeded",
            SuspendReason::Os => "requested by operating system",
            SuspendReason::WifiState => "not connected to WiFi network",
            SuspendReason::BatteryCharging => "battery low",
            SuspendReason::BatteryOverheated => "battery thermal protection",
            SuspendReason::NoGuiKeepalive => "GUI not active",
            SuspendReason::UnknownToJoinc => "unknown",
        })
    }
}
// ----- deserialization helper -----

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
struct DeserializedBool {
    value: Option<String>,
}

impl From<DeserializedBool> for Bool {
    fn from(b: DeserializedBool) -> Self {
        b.value
            .filter(|s| s != &"0".to_string())
            .map(|_| Bool::True)
            .unwrap_or(Bool::False)
    }
}

// ----- Tests -----

#[cfg(test)]
mod tests {
    use super::*;
    use libjoincserde::from_str;
    use serde::Deserialize;

    #[test]
    fn deserializes_booleans() {
        #[derive(Deserialize, Debug, Default, PartialEq, Eq)]
        #[serde(default, rename = "dto")]
        struct Dto {
            a_bool: Bool,
            another_bool: Bool,
            unset_bool: Bool,
            not_set: Bool,
        }

        let expected = Dto {
            a_bool: Bool::True,
            another_bool: Bool::True,
            unset_bool: Bool::False,
            not_set: Bool::False,
        };

        let xml = "<dto><a_bool/><another_bool>1</another_bool><unset_bool>0</unset_bool></dto>";
        let deserialized: Dto = from_str(xml).unwrap();

        assert_eq!(deserialized, expected);
    }
}
