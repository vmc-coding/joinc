use crate::defs::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ActiveTask {
    pub active_task_state: ActiveTaskState,
    pub scheduler_state: SchedulerState,

    pub needs_shmem: Bool,
    pub too_large: Bool,

    pub pid: i32,
    pub slot: i32,

    pub bytes_received: f64,
    pub bytes_sent: f64,
    pub checkpoint_cpu_time: f64,
    pub current_cpu_time: f64,
    pub elapsed_time: f64,
    pub fraction_done: f64,
    pub progress_rate: f64,
    pub swap_size: f64,
    pub working_set_size_smoothed: f64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CCStatus {
	pub gpu_mode: RunMode,
	pub gpu_mode_delay: f64,
	pub gpu_mode_perm: RunMode,
	pub gpu_suspend_reason: SuspendReason,

	pub network_mode: RunMode,
	pub network_mode_delay: f64,
	pub network_mode_perm: RunMode,
	pub network_suspend_reason: SuspendReason,

	pub task_mode: RunMode,
	pub task_mode_delay: f64,
	pub task_mode_perm: RunMode,
	pub task_suspend_reason: SuspendReason,

    pub ams_password_error: Bool,
    pub disallow_attach: Bool,
    pub manager_must_quit: Bool,
    pub simple_gui_only: Bool,

    pub max_event_log_lines: i32,

    pub network_status: NetworkStatus,
}

#[derive(Copy, Clone, Debug, Default, Deserialize)]
#[serde(default)]
#[serde(transparent)]
pub struct Duration(pub f64); // in seconds

impl From<Duration> for f64 {
    fn from(d: Duration) -> Self {
        d.0
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct FileTransfer {
    pub sticky: Bool,

    pub project_backoff: Duration,

    pub nbytes: f64,
    pub max_nbytes: f64,

    pub status: i32,

    pub file_xfer: Option<FileXfer>,
    pub persistent_file_xfer: Option<PersistentFileXfer>,

    pub name: String,
    pub project_name: String,
    pub project_url: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct FileXfer {
    pub estimated_xfer_time_remaining: Duration,

    pub bytes_xferred: f64,
    pub xfer_speed: f64,
    pub file_offset: f64,

    pub url: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GuiUrl {
    pub name: String,
    pub description: String,
    pub url: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(from = "GuiUrlsDto")]
pub struct GuiUrls(pub Vec<GuiUrl>);

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct GuiUrlsDto {
    gui_url: Vec<GuiUrl>,
}

impl From<GuiUrlsDto> for GuiUrls {
    fn from(dto: GuiUrlsDto) -> Self {
        GuiUrls ( dto.gui_url )
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Message {
    pub body: String,
    #[serde(rename = "pri")]
    pub priority: MsgInfo,
    pub project: String,
    pub seqno: i32,
    #[serde(rename = "time")]
    pub timestamp: Timestamp,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Notice {
    pub is_private: Bool,

    pub seqno: i32,

    pub category: String,
    pub description: String,
    pub link: String,
    pub project_name: String,
    pub title: String,

    pub arrival_time: Timestamp,
    pub create_time: Timestamp,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct PersistentFileXfer {
    pub is_upload: Bool,

    pub time_so_far: Duration,

    pub last_bytes_xferred: f64,

    pub num_retries: i32,

    pub first_request_time: Timestamp,
    pub next_request_time: Timestamp,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Project {
    pub anonymous_platform: Bool,
    pub attached_via_acct_mgr: Bool,
    pub detach_when_done: Bool,
    pub dont_request_more_work: Bool,
    pub ended: Bool,
    pub master_url_fetch_pending: Bool,
    pub non_cpu_intensive: Bool,
    pub scheduler_rpc_in_progress: Bool,
    pub suspended_via_gui: Bool,
    pub trickle_up_pending: Bool,

    pub download_backoff: Duration,
    pub upload_backoff: Duration,

    pub disk_usage: f64,
    pub duration_correction_factor: f64,
    pub elapsed_time: f64,
    pub host_expavg_credit: f64,
    pub host_total_credit: f64,
    pub resource_share: f64,
    pub sched_priority: f64,
    pub user_expavg_credit: f64,
    pub user_total_credit: f64,

    pub gui_urls: GuiUrls,

    pub hostid: i32,
    pub master_fetch_failures: i32,
    pub njobs_error: i32,
    pub njobs_success: i32,
    pub nrpc_failures: i32,

    pub sched_rpc_pending: RpcReason,

    pub external_cpid: String,
    pub master_url: String,
    pub project_dir: String,
    pub project_name: String,
    pub team_name: String,
    pub user_name: String,
    pub venue: String,

    pub last_rpc_time: Timestamp,
    pub min_rpc_time: Timestamp,
    pub project_files_downloaded_time: Timestamp,
}

// We're calling BOINC's 'result' structure 'task' because of
// the naming clash with Rust's 'Result' type used everywhere.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Task {
    pub coproc_missing: Bool,
    pub got_server_ack: Bool,
    pub network_wait: Bool,
    pub project_suspended_via_gui: Bool,
    pub ready_to_report: Bool,
    pub report_immediately: Bool,
    pub scheduler_wait: Bool,
    pub suspended_via_gui: Bool,

    pub estimated_cpu_time_remaining: Duration,
    pub final_cpu_time: Duration,
    pub final_elapsed_time: Duration,

    pub exit_status: i32,
    pub signal: i32,
    pub version_num: i32,

    pub active_task: Option<ActiveTask>,

    pub state: ResultClientState,

    pub name: String,
    pub plan_class: String,
    pub platform: String,
    pub project_url: String,
    pub resources: String,
    pub scheduler_wait_reason: String,
    pub wu_name: String,

    pub received_time: Timestamp,
    pub report_deadline: Timestamp,
}

#[derive(Copy, Clone, Debug, Default, Deserialize)]
#[serde(default)]
#[serde(transparent)]
pub struct Timestamp(pub f64); // seconds since epoch in UTC

impl From<Timestamp> for f64 {
    fn from(t: Timestamp) -> Self {
        t.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "version")]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub release: i32,
}

impl std::default::Default for Version {
    fn default() -> Self {
        Self {
            major: 7,
            minor: 22,
            release: 0,
        }
    }
}

impl Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("{}.{}.{}", self.major, self.minor, self.release))
    }
}
