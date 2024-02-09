use clap::{Parser, Subcommand};
use libjoinc::defs::*;
use libjoinc::error::*;
use libjoinc::rpc::commands::*;
use libjoinc::rpc::connection;
use libjoinc::types::*;
use std::fmt;

use chrono::prelude::*;

const JOINCCMD_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
struct Cli {
    /// Name of the host where the BOINC client is running
    #[arg(long, default_value = "localhost")]
    host: String,

    /// Port on which the BOINC client is listening
    #[arg(long,default_value_t=connection::DEFAULT_PORT)]
    port: u16,

    /// Password to authenticate against the BOINC client
    #[arg(long)]
    passwd: Option<String>,

    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand, PartialEq)]
enum CliCommand {
    /// Show client version
    ClientVersion,
    /// Show cc status
    GetCCStatus,
    /// Show messages
    GetMessages {
        /// Show messages with sequence number > seqno only
        #[arg(default_value = "0")]
        seqno: u32,
    },
    /// Show notices
    GetNotices {
        /// Show notices with sequence number > seqno only
        #[arg(default_value = "0")]
        seqno: u32,
    },
    /// Show status of all attached projects
    #[command(visible_alias = "get-project-status")]
    GetProjects,
    /// Show tasks
    GetTasks {
        /// Show only active tasks
        #[arg(long)]
        active_only: bool,
    },
    /// Read the cc_config.xml file
    ReadCcConfig,
    /// Show the verion of this cli
    Version,
}

fn main() {
    let cli = Cli::parse();

    if cli.command == CliCommand::Version {
        println!("Version: {JOINCCMD_VERSION}");
        std::process::exit(0);
    }

    let mut connection = connection::Connection::open(&cli.host, cli.port).unwrap_or_else(|err| {
        eprintln!("Failed to connect to BOINC client: {}", err.display());
        std::process::exit(1);
    });

    if let Some(passwd) = cli.passwd.as_deref() {
        AuthorizeCommand::new(passwd)
            .execute(&mut connection)
            .unwrap_or_else(|_| {
                eprintln!("Authentication failed. Incorrect password?");
                std::process::exit(1);
            });
    }

    process_command(&mut connection, cli.command).unwrap_or_else(|err| {
        eprintln!("The command failed with: {}", err.display());
        std::process::exit(1);
    });
}

fn process_command(connection: &mut connection::Connection, command: CliCommand) -> Result<()> {
    match command {
        CliCommand::ClientVersion => {
            println!("Client version: {}", ExchangeVersionsCommand::default().execute(connection)?.display());
        }
        CliCommand::GetCCStatus => {
            print!("{}", GetCCStatusCommand::default().execute(connection)?.display());
        }
        CliCommand::GetMessages { seqno } => {
            for msg in GetMessagesCommand::new(seqno).execute(connection)? {
                println!("{}", msg.display());
            }
        }
        CliCommand::GetNotices { seqno } => {
            for notice in GetNoticesCommand::new(seqno).execute(connection)?.into_iter().rev() {
                println!("{}", notice.display());
            }
        }
        CliCommand::GetProjects => {
            println!("======== Projects ========");
            for (idx, project) in GetProjectStatusCommand::default()
                .execute(connection)?
                .iter()
                .enumerate()
            {
                println!("{}) -----------", idx + 1);
                print!("{}", project.display());
            }
        }
        CliCommand::GetTasks { active_only } => {
            println!("\n======== Tasks ========");
            for (idx, task) in GetResultsCommand::new(active_only)
                .execute(connection)?
                .iter()
                .enumerate()
            {
                println!("{}) -----------", idx + 1);
                print!("{}", task.display());
            }
        }
        CliCommand::ReadCcConfig => ReadCCConfigCommand::default().execute(connection)?,
        CliCommand::Version => panic!("Should've never reached this line"),
    };

    Ok(())
}

// ----- unit conversions -----

fn to_mibi(d: f64) -> f64 {
    d / (1024. * 1024.)
}

// ----- helpers for displaying -----

struct Displayable<T>(T);

trait Display<T> {
    fn display(self) -> Displayable<T>;
}

impl<T> Display<T> for T {
    fn display(self) -> Displayable<T> {
        Displayable(self)
    }
}

impl fmt::Display for Displayable<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match &self.0 {
            false => "no",
            true => "yes",
        })
    }
}

struct FormattedTimestamp {
    timestamp: f64,
    format: &'static str,
}

impl FormattedTimestamp {
    fn new(timestamp: f64) -> Self {
        Self {
            timestamp,
            format: "%c",
        }
    }

    fn with_format(timestamp: f64, format: &'static str) -> FormattedTimestamp {
        Self { timestamp, format }
    }
}

impl fmt::Display for FormattedTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            &Some(self.timestamp)
                .filter(|&t| t > 0.)
                .and_then(|t| NaiveDateTime::from_timestamp_opt(t as i64, 0))
                .map(|ndt| Local
                    .from_utc_datetime(&ndt)
                    .with_timezone(&Local)
                    .format(self.format)
                    .to_string())
                .unwrap_or("---".to_string())
        )
    }
}

struct FormattedCCState<'a>(&'a str, RunMode, f64, RunMode, SuspendReason);

impl fmt::Display for FormattedCCState<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const INDENT: &str = "    ";

        writeln!(f, "{} status", self.0)?;
        writeln!(f, "{INDENT}{}", match self.4 {
            SuspendReason::NotSuspended => "not suspended".to_string(),
            _ => format!("suspended: {}", self.4),
        })?;
        writeln!(f, "{INDENT}current mode: {}", self.1)?;
        writeln!(f, "{INDENT}perm mode: {}", self.3)?;
        writeln!(f, "{INDENT}perm becomes current in {} sec", self.2 as isize)?;
        Ok(())
    }
}

struct Usage(f64);

impl fmt::Display for Usage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}MB", self.0 / (1024. * 1024.))
    }
}

// ----- displaying libjoinc's types -----

impl fmt::Display for Displayable<CCStatus> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "network connection status: {}", self.0.network_status)?;
        write!(f, "{}", FormattedCCState("CPU",
                self.0.task_mode, self.0.task_mode_delay, self.0.task_mode_perm, self.0.task_suspend_reason))?;
        write!(f, "{}", FormattedCCState("GPU",
                self.0.gpu_mode, self.0.gpu_mode_delay, self.0.gpu_mode_perm, self.0.gpu_suspend_reason))?;
        write!(f, "{}", FormattedCCState("Network",
                self.0.network_mode, self.0.network_mode_delay, self.0.network_mode_perm, self.0.network_suspend_reason))?;
        Ok(())
    }
}

impl fmt::Display for Displayable<Error> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Error::Client(err) => write!(f, "Client error: {}.", err),
            Error::Deserialization(serde_err) => write!(f, "Deserialization error: {}.", serde_err),
            Error::Io(io_err) => write!(f, "IO error: {}.", io_err),
            Error::Rpc(rpc_err) => write!(f, "RPC error: {}.", rpc_err),
            Error::Unauthorized => write!(f, "Unauthorized, please set the password via --passwd <PASSWD>."),
        }
    }
}

impl fmt::Display for Displayable<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ({}) [{}] {}",
            self.0.seqno,
            FormattedTimestamp::with_format(self.0.timestamp, "%d-%b-%Y %H:%M:%S"),
            self.0.priority,
            self.0.project,
            self.0.body.trim()
        )
    }
}

impl fmt::Display for Displayable<Notice> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ({}) {}",
            self.0.seqno,
            FormattedTimestamp::with_format(self.0.create_time, "%d-%b-%Y %H:%M:%S"),
            self.0.description.trim()
        )
    }
}

impl fmt::Display for Displayable<&Project> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const INDENT: &str = "   ";

        let project = &self.0;

        writeln!(f, "{INDENT}name: {}", project.project_name)?;
        writeln!(f, "{INDENT}master URL: {}", project.master_url)?;
        writeln!(f, "{INDENT}user_name: {}", project.user_name)?;
        writeln!(f, "{INDENT}team_name: {}", project.team_name)?;
        writeln!(f, "{INDENT}resource share: {}", project.resource_share)?;
        writeln!(f, "{INDENT}user_total_credit: {}", project.user_total_credit)?;
        writeln!(f, "{INDENT}user_expavg_credit: {}", project.user_expavg_credit)?;
        writeln!(f, "{INDENT}host_total_credit: {}", project.host_total_credit)?;
        writeln!(f, "{INDENT}host_expavg_credit: {}", project.host_expavg_credit)?;
        writeln!(f, "{INDENT}nrpc_failures: {}", project.nrpc_failures)?;
        writeln!(f, "{INDENT}master_fetch_failures: {}", project.master_fetch_failures)?;
        writeln!(f, "{INDENT}master fetch pending: {}", project.master_url_fetch_pending)?;
        writeln!(f, "{INDENT}scheduler RPC pending: {}", (project.sched_rpc_pending != RpcReason::None).display())?;
        writeln!(f, "{INDENT}trickle upload pending: {}", project.trickle_up_pending)?;
        writeln!(f, "{INDENT}attached via Account Manager: {}", project.attached_via_acct_mgr)?;
        writeln!(f, "{INDENT}ended: {}", project.ended)?;
        writeln!(f, "{INDENT}suspended via GUI: {}", project.suspended_via_gui)?;
        writeln!(f, "{INDENT}don't request more work: {}", project.dont_request_more_work)?;
        writeln!(f, "{INDENT}disk usage: {}", Usage(project.disk_usage))?;
        writeln!(f, "{INDENT}last RPC: {}", FormattedTimestamp::new(project.last_rpc_time))?;
        writeln!(f)?;
        writeln!(f, "{INDENT}project files downloaded: {}", FormattedTimestamp::new(project.project_files_downloaded_time))?;

        for gui_url in &project.gui_urls.0 {
            writeln!(f, "GUI URL:")?;
            writeln!(f, "{INDENT}name: {}", gui_url.name)?;
            writeln!(f, "{INDENT}description: {}", gui_url.description)?;
            writeln!(f, "{INDENT}URL: {}", gui_url.url)?;
        }

        writeln!(f, "{INDENT}jobs succeeded: {}", project.njobs_success)?;
        writeln!(f, "{INDENT}jobs failed: {}", project.njobs_error)?;
        writeln!(f, "{INDENT}elapsed time: {}", project.elapsed_time)?;
        writeln!(f, "{INDENT}cross-project ID: {}", project.external_cpid)?;

        Ok(())
    }
}

impl fmt::Display for Displayable<&Task> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const INDENT: &str = "   ";

        let task = &self.0;
        let scheduler_state = task
            .active_task
            .as_ref()
            .map_or(SchedulerState::Uninitialized, |t| t.scheduler_state.clone());
        let active_task_state = task
            .active_task
            .as_ref()
            .map_or(ActiveTaskState::Uninitialized, |at| at.active_task_state.clone());

        writeln!(f, "{INDENT}name: {}", task.name)?;
        writeln!(f, "{INDENT}WU name: {}", task.wu_name)?;
        writeln!(f, "{INDENT}project URL: {}", task.project_url)?;
        writeln!(f, "{INDENT}received: {}", FormattedTimestamp::new(task.received_time))?;
        writeln!(f, "{INDENT}report deadline: {}", FormattedTimestamp::new(task.report_deadline))?;
        writeln!(f, "{INDENT}ready to report: {}", task.ready_to_report)?;
        writeln!(f, "{INDENT}state: {}", task.state)?;
        writeln!(f, "{INDENT}scheduler state: {}", scheduler_state)?;
        writeln!(f, "{INDENT}active_task_state: {}", active_task_state)?;
        writeln!(f, "{INDENT}app version num: {}", task.version_num)?;
        writeln!(f, "{INDENT}resources: {}",
            Some(task.resources.as_str()).filter(|rs| !rs.is_empty()).get_or_insert("1 CPU"))?;

        if task.state as isize >= 0
            && task.state as isize <= ResultClientState::FilesDownloaded as isize
        {
            if task.suspended_via_gui.into() {
                writeln!(f, "{INDENT}suspended via GUI: yes")?;
            }
            writeln!(f, "{INDENT}estimated CPU time remaining: {:.6}", task.estimated_cpu_time_remaining)?;
            if let Some(active_task) = &task.active_task {
                writeln!(f, "{INDENT}elapsed task time: {:.6}", active_task.elapsed_time)?;
            }
        }

        if scheduler_state as isize > SchedulerState::Uninitialized as isize {
            if let Some(active_task) = &task.active_task {
                writeln!(f, "{INDENT}slot: {}", active_task.slot)?;
                writeln!(f, "{INDENT}PID: {}", active_task.pid)?;
                writeln!(f, "{INDENT}CPU time at last checkpoint: {:.6}", active_task.checkpoint_cpu_time)?;
                writeln!(f, "{INDENT}current CPU time: {:.6}", active_task.current_cpu_time)?;
                writeln!(f, "{INDENT}fraction done: {:.6}", active_task.fraction_done)?;
                writeln!(f, "{INDENT}swap size: {:.0} MB", to_mibi(active_task.swap_size))?;
                writeln!(f, "{INDENT}working set size: {:.0} MB", to_mibi(active_task.working_set_size_smoothed))?;

                if active_task.bytes_sent > 0. || active_task.bytes_received > 0. {
                    writeln!(f, "{INDENT}bytes sent: {:.0} received: {:.0}",
                        active_task.bytes_sent, active_task.bytes_received)?;
                }
            }
        }

        if task.state as isize > ResultClientState::FilesDownloaded as isize {
            writeln!(f, "{INDENT}final CPU time: {}", task.final_cpu_time)?;
            writeln!(f, "{INDENT}final elapsed time: {}", task.final_elapsed_time)?;
            writeln!(f, "{INDENT}exit_status: {}", task.exit_status)?;
            writeln!(f, "{INDENT}signal: {}", task.signal)?;
        }

        Ok(())
    }
}

impl fmt::Display for Displayable<Version> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
