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
        GuiUrls(dto.gui_url)
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
    pub timestamp: f64,
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

    pub disk_usage: f64,
    pub duration_correction_factor: f64,
    pub elapsed_time: f64,
    pub host_expavg_credit: f64,
    pub host_total_credit: f64,
    pub resource_share: f64,
    pub sched_priority: f64,
    pub user_expavg_credit: f64,
    pub user_total_credit: f64,

    pub hostid: i32,
    pub master_fetch_failures: i32,
    pub njobs_error: i32,
    pub njobs_success: i32,
    pub nrpc_failures: i32,

    pub external_cpid: String,
    pub master_url: String,
    pub project_dir: String,
    pub project_name: String,
    pub team_name: String,
    pub user_name: String,
    pub venue: String,

    // TODO Use a suitable type from std::time
    pub download_backoff: f64,
    pub last_rpc_time: f64,
    pub min_rpc_time: f64,
    pub project_files_downloaded_time: f64,
    pub upload_backoff: f64,

    pub sched_rpc_pending: RpcReason,
    pub gui_urls: GuiUrls,
}

// We're calling BOINC's 'result' structure 'task' because of
// the naming clash with Rust's 'Result' type used everywhere.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Task {
    pub state: ResultClientState,

    pub coproc_missing: Bool,
    pub got_server_ack: Bool,
    pub network_wait: Bool,
    pub project_suspended_via_gui: Bool,
    pub ready_to_report: Bool,
    pub report_immediately: Bool,
    pub scheduler_wait: Bool,
    pub suspended_via_gui: Bool,

    pub estimated_cpu_time_remaining: f64,
    pub final_cpu_time: f64,
    pub final_elapsed_time: f64,
    pub received_time: f64,
    pub report_deadline: f64,

    pub exit_status: i32,
    pub signal: i32,
    pub version_num: i32,

    pub name: String,
    pub plan_class: String,
    pub platform: String,
    pub project_url: String,
    pub resources: String,
    pub scheduler_wait_reason: String,
    pub wu_name: String,

    pub active_task: Option<ActiveTask>,
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
