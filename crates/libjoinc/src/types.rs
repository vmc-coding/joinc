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

#[derive(Deserialize)]
pub struct Message {
    pub body: String,
    #[serde(rename = "pri")]
    pub priority: MsgInfo,
    pub project: String,
    pub seqno: i32,
    #[serde(rename = "time")]
    pub timestamp: f64,
}

// We're calling BOINC's 'result' structure 'task' because of
// the naming clash with Rust's 'Result' type used everywhere.
#[derive(Deserialize, Debug, Default)]
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
