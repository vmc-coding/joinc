use clap::{Parser, Subcommand, ValueEnum};
use libjoinc::defs::*;
use libjoinc::error::*;
use libjoinc::rpc::commands::*;
use libjoinc::rpc::connection;
use libjoinc::types::*;
use std::fmt;

use chrono::prelude::*;

static JOINCCMD_VERSION: &str = env!("CARGO_PKG_VERSION");
static INDENT2: &str = "  ";
static INDENT3: &str = "   ";
static INDENT4: &str = "    ";

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
    /// Execute an operation on a file transfer
    #[command(visible_alias = "file-transfer")]
    FileTransferOp {
        /// The transfer's project url
        project_url: String,
        /// The transfers's filename
        filename: String,
        /// The operation to execute
        #[arg(value_enum)]
        op: SupportedFileTransferOp,
    },
    /// Show cc status
    GetCCStatus,
    /// Show disk usage
    GetDiskUsage,
    /// Show host info
    GetHostInfo,
    /// Show file transfers
    GetFileTransfers,
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
    #[command(visible_alias = "get-results")]
    GetTasks {
        /// Show only active tasks
        #[arg(long)]
        active_only: bool,
    },
    /// Retry deferred network communication
    NetworkAvailable,
    /// Execute an operation on a project
    #[command(visible_alias = "project")]
    ProjectOp {
        /// The project's url
        project_url: String,
        /// The operation to execute
        #[arg(value_enum)]
        op: SupportedProjectOp,
    },
    /// Tell the client to quit
    Quit,
    /// Read the cc_config.xml file
    ReadCcConfig,
    /// Read the global_prefs_override.xml file
    ReadGlobalPrefsOverride,
    /// Run the benchmarks
    RunBenchmarks,
    /// Set gpu run mode for given duration
    SetGpuMode {
        /// The mode to run
        #[arg(value_enum)]
        mode: SupportedRunMode,
        /// The duration this mode to be set
        #[arg(default_value = "0")]
        duration: f64,
    },
    /// Set network mode for given duration
    SetNetworkMode {
        /// The mode to run
        #[arg(value_enum)]
        mode: SupportedRunMode,
        /// The duration this mode to be set
        #[arg(default_value = "0")]
        duration: f64,
    },
    /// Set run mode for given duration
    SetRunMode {
        /// The mode to run
        #[arg(value_enum)]
        mode: SupportedRunMode,
        /// The duration this mode to be set
        #[arg(default_value = "0")]
        duration: f64,
    },
    /// Execute an operation a task
    #[command(visible_alias = "task")]
    #[command(visible_alias = "result-op")]
    #[command(visible_alias = "result")]
    TaskOp {
        /// The task's project url
        project_url: String,
        /// The task's name
        name: String,
        /// The operation to execute
        #[arg(value_enum)]
        op: SupportedTaskOp,
    },
    /// Show the version of this cli
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
        CliCommand::FileTransferOp { project_url, filename, op } => {
            FileTransferOpCommand::new(project_url, filename, op.into()).execute(connection)?;
        }
        CliCommand::GetCCStatus => {
            print!("{}", GetCCStatusCommand::default().execute(connection)?.display());
        }
        CliCommand::GetDiskUsage => {
            println!("======== Disk usage ========");
            print!("{}", GetDiskUsageSummaryCommand::default().execute(connection)?.display());
        }
        CliCommand::GetHostInfo => {
            print!("{}", GetHostInfoCommand::default().execute(connection)?.display());
        }
        CliCommand::GetFileTransfers => {
            println!("======== File transfers ========");
            for (idx, file_transfer) in GetFileTransfersCommand::default().execute(connection)?.into_iter().enumerate() {
                println!("{}) -----------", idx + 1);
                print!("{}", file_transfer.display());
            }
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
            for (idx, project) in GetProjectStatusCommand::default().execute(connection)?.into_iter().enumerate() {
                println!("{}) -----------", idx + 1);
                print!("{}", project.display());
            }
        }
        CliCommand::GetTasks { active_only } => {
            println!("======== Tasks ========");
            for (idx, task) in GetResultsCommand::new(active_only).execute(connection)?.into_iter().enumerate() {
                println!("{}) -----------", idx + 1);
                print!("{}", task.display());
            }
        }
        CliCommand::NetworkAvailable => NetworkAvailableCommand::default().execute(connection)?,
        CliCommand::ProjectOp { project_url, op } => ProjectOpCommand::new(project_url, op.into()).execute(connection)?,
        CliCommand::Quit => QuitCommand::default().execute(connection)?,
        CliCommand::ReadCcConfig => ReadCCConfigCommand::default().execute(connection)?,
        CliCommand::ReadGlobalPrefsOverride => ReadGlobalPreferencesOverrideCommand::default().execute(connection)?,
        CliCommand::RunBenchmarks => RunBenchmarksCommand::default().execute(connection)?,
        CliCommand::SetGpuMode { mode, duration } => SetGpuModeCommand::new(mode.into(), duration).execute(connection)?,
        CliCommand::SetNetworkMode { mode, duration } => SetNetworkModeCommand::new(mode.into(), duration).execute(connection)?,
        CliCommand::SetRunMode { mode, duration } => SetRunModeCommand::new(mode.into(), duration).execute(connection)?,
        CliCommand::TaskOp { project_url, name, op } => TaskOpCommand::new(project_url, name, op.into()).execute(connection)?,
        CliCommand::Version => panic!("Should've never reached this branch"),
    };

    Ok(())
}

// ----- helpers for parsing cli parameters -----

#[derive(Clone, PartialEq, ValueEnum)]
enum SupportedFileTransferOp {
    Abort,
    Retry,
}

impl From<SupportedFileTransferOp> for FileTransferOp {
    fn from(op: SupportedFileTransferOp) -> Self {
        match op {
            SupportedFileTransferOp::Abort => FileTransferOp::Abort,
            SupportedFileTransferOp::Retry => FileTransferOp::Retry,
        }
    }
}

#[derive(Clone, PartialEq, ValueEnum)]
enum SupportedProjectOp {
    Allowmorework,
    Detach,
    DetachWhenDone,
    DontDetachWhenDone,
    Nomorework,
    Reset,
    Resume,
    Suspend,
    Update,
}

impl From<SupportedProjectOp> for ProjectOp {
    fn from(op: SupportedProjectOp) -> Self {
        match op {
            SupportedProjectOp::Allowmorework => ProjectOp::Allowmorework,
            SupportedProjectOp::Detach => ProjectOp::Detach,
            SupportedProjectOp::DetachWhenDone => ProjectOp::DetachWhenDone,
            SupportedProjectOp::DontDetachWhenDone => ProjectOp::DontDetachWhenDone,
            SupportedProjectOp::Nomorework => ProjectOp::Nomorework,
            SupportedProjectOp::Reset => ProjectOp::Reset,
            SupportedProjectOp::Resume => ProjectOp::Resume,
            SupportedProjectOp::Suspend => ProjectOp::Suspend,
            SupportedProjectOp::Update => ProjectOp::Update,
        }
    }
}

#[derive(Clone, PartialEq, ValueEnum)]
enum SupportedRunMode {
    Always,
    Auto,
    Never,
}

impl From<SupportedRunMode> for RunMode {
    fn from(mode: SupportedRunMode) -> Self {
        match mode {
            SupportedRunMode::Always => RunMode::Always,
            SupportedRunMode::Auto => RunMode::Auto,
            SupportedRunMode::Never => RunMode::Never,
        }
    }
}

#[derive(Clone, PartialEq, ValueEnum)]
enum SupportedTaskOp {
    Abort,
    Resume,
    Suspend
}

impl From<SupportedTaskOp> for TaskOp {
    fn from(op: SupportedTaskOp) -> Self {
        match op {
            SupportedTaskOp::Abort => TaskOp::Abort,
            SupportedTaskOp::Resume => TaskOp::Resume,
            SupportedTaskOp::Suspend => TaskOp::Suspend,
        }
    }
}

// ----- unit conversions -----

fn to_mibi(d: f64) -> f64 {
    d / (1024. * 1024.)
}

fn to_gflops(f: f64) -> f64 {
    f / 1e9
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
    timestamp: Timestamp,
    format: &'static str,
}

impl FormattedTimestamp {
    fn new(timestamp: Timestamp) -> Self {
        Self {
            timestamp,
            format: "%c",
        }
    }

    fn with_format(timestamp: Timestamp, format: &'static str) -> Self {
        Self { timestamp, format }
    }
}

impl fmt::Display for FormattedTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            &Some(self.timestamp.0)
                .filter(|&t| t > 0.)
                .and_then(|t| DateTime::from_timestamp(t as i64, 0))
                .map(|dt| dt
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
        writeln!(f, "{} status", self.0)?;
        writeln!(f, "{INDENT4}{}", match self.4 {
            SuspendReason::NotSuspended => "not suspended".to_string(),
            _ => format!("suspended: {}", self.4),
        })?;
        writeln!(f, "{INDENT4}current mode: {}", self.1)?;
        writeln!(f, "{INDENT4}perm mode: {}", self.3)?;
        writeln!(f, "{INDENT4}perm becomes current in {} sec", self.2 as isize)?;
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

impl fmt::Display for Displayable<DiskUsageSummary> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "total: {}", Usage(self.0.total))?;
        writeln!(f, "free: {}", Usage(self.0.free))?;

        for (idx, project) in self.0.projects.iter().enumerate() {
            writeln!(f, "{}) -----------", idx + 1)?;
            writeln!(f, "{INDENT3}master URL: {}", project.master_url)?;
            writeln!(f, "{INDENT3}disk usage: {}", Usage(project.disk_usage))?;
        }

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

impl fmt::Display for Displayable<FileTransfer> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut direction = "unknown";
        let mut is_active = false;
        let mut time_so_far = 0f64;
        let mut bytes_xferred = 0f64;
        let mut xfer_speed = 0f64;
        let mut estimated_xfer_time_remaining = 0f64;

        if let Some(pfx) = &self.0.persistent_file_xfer {
            direction = if pfx.is_upload.into() { "upload" } else { "download" };
            time_so_far = pfx.time_so_far.into();
        }

        if let Some(xfer) = &self.0.file_xfer {
            is_active = true;
            bytes_xferred = xfer.bytes_xferred;
            xfer_speed = xfer.xfer_speed;
            estimated_xfer_time_remaining = xfer.estimated_xfer_time_remaining.into();
        }

        writeln!(f, "{INDENT3}name: {}", self.0.name)?;
        writeln!(f, "{INDENT3}direction: {}", direction)?;
        writeln!(f, "{INDENT3}sticky: {}", self.0.sticky)?;
        writeln!(f, "{INDENT3}xfer active: {}", is_active.display())?;
        writeln!(f, "{INDENT3}time_so_far: {:.6}", time_so_far)?;
        if is_active {
            writeln!(f, "{INDENT3}estimated_xfer_time_remaining: {:.6}", estimated_xfer_time_remaining)?;
        }
        writeln!(f, "{INDENT3}bytes_xferred: {:.6}", bytes_xferred)?;
        writeln!(f, "{INDENT3}xfer_speed: {:.6}", xfer_speed)?;

        Ok(())
    }
}

impl fmt::Display for Displayable<&Coprocs> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      for (index, coproc) in self.0.amds.iter().enumerate() {
          writeln!(f, "{INDENT2}AMD GPU: {} (CAL version {}, {}MB, {:.0}MB available, {:.0} GFLOPS peak)",
              coproc.name, coproc.version, coproc.local_ram, to_mibi(coproc.available_ram), to_gflops(coproc.peak_flops))?;

          if coproc.count > 1 {
              writeln!(f, "{INDENT4}Count: {}", coproc.count)?;
          }

          if let Some(opencl) = &coproc.opencl {
              writeln!(f, "{INDENT4}OpenCL: AMD {}: {:.64} (driver version {:.64}, device version {:.64}, {:.0}MB, {:.0}MB available, {:.0} GFLOPS peak)",
                  index, opencl.name, opencl.opencl_driver_version, opencl.opencl_device_version.trim(),
                  to_mibi(opencl.global_mem_size), to_mibi(coproc.available_ram), to_gflops(coproc.peak_flops))?;
          }
      }

      Ok(())
  }
}

impl fmt::Display for Displayable<HostInfo> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      writeln!(f, "{INDENT2}timezone: {}", self.0.timezone)?;
      writeln!(f, "{INDENT2}domain name: {}", self.0.domain_name)?;
      writeln!(f, "{INDENT2}IP addr: {}", self.0.ip_addr)?;
      writeln!(f, "{INDENT2}#CPUS: {}", self.0.p_ncpus)?;
      writeln!(f, "{INDENT2}CPU vendor: {}", self.0.p_vendor)?;
      writeln!(f, "{INDENT2}CPU model: {}", self.0.p_model)?;
      writeln!(f, "{INDENT2}CPU FP OPS: {:.6}", self.0.p_fpops)?;
      writeln!(f, "{INDENT2}CPU int OPS: {:.6}", self.0.p_iops)?;
      writeln!(f, "{INDENT2}CPU mem BW: {:.6}", self.0.p_membw)?;
      writeln!(f, "{INDENT2}OS name: {}", self.0.os_name)?;
      writeln!(f, "{INDENT2}OS version: {}", self.0.os_version)?;
      writeln!(f, "{INDENT2}mem size: {:.6}", self.0.m_nbytes)?;
      writeln!(f, "{INDENT2}cache size: {:.6}", self.0.m_cache)?;
      writeln!(f, "{INDENT2}swap size: {:.6}", self.0.m_swap)?;
      writeln!(f, "{INDENT2}disk size: {:.6}", self.0.d_total)?;
      writeln!(f, "{INDENT2}disk free: {:.6}", self.0.d_free)?;
      write!(f, "{}", (&self.0.coprocs).display())?;
      Ok(())
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

impl fmt::Display for Displayable<Project> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let project = &self.0;

        writeln!(f, "{INDENT3}name: {}", project.project_name)?;
        writeln!(f, "{INDENT3}master URL: {}", project.master_url)?;
        writeln!(f, "{INDENT3}user_name: {}", project.user_name)?;
        writeln!(f, "{INDENT3}team_name: {}", project.team_name)?;
        writeln!(f, "{INDENT3}resource share: {:.6}", project.resource_share)?;
        writeln!(f, "{INDENT3}user_total_credit: {:.6}", project.user_total_credit)?;
        writeln!(f, "{INDENT3}user_expavg_credit: {:.6}", project.user_expavg_credit)?;
        writeln!(f, "{INDENT3}host_total_credit: {:.6}", project.host_total_credit)?;
        writeln!(f, "{INDENT3}host_expavg_credit: {:.6}", project.host_expavg_credit)?;
        writeln!(f, "{INDENT3}nrpc_failures: {}", project.nrpc_failures)?;
        writeln!(f, "{INDENT3}master_fetch_failures: {}", project.master_fetch_failures)?;
        writeln!(f, "{INDENT3}master fetch pending: {}", project.master_url_fetch_pending)?;
        writeln!(f, "{INDENT3}scheduler RPC pending: {}", (project.sched_rpc_pending != RpcReason::None).display())?;
        writeln!(f, "{INDENT3}trickle upload pending: {}", project.trickle_up_pending)?;
        writeln!(f, "{INDENT3}attached via Account Manager: {}", project.attached_via_acct_mgr)?;
        writeln!(f, "{INDENT3}ended: {}", project.ended)?;
        writeln!(f, "{INDENT3}suspended via GUI: {}", project.suspended_via_gui)?;
        writeln!(f, "{INDENT3}don't request more work: {}", project.dont_request_more_work)?;
        writeln!(f, "{INDENT3}disk usage: {}", Usage(project.disk_usage))?;
        writeln!(f, "{INDENT3}last RPC: {}", FormattedTimestamp::new(project.last_rpc_time))?;
        writeln!(f)?;
        writeln!(f, "{INDENT3}project files downloaded: {}", FormattedTimestamp::new(project.project_files_downloaded_time))?;

        for gui_url in &project.gui_urls.0 {
            writeln!(f, "GUI URL:")?;
            writeln!(f, "{INDENT3}name: {}", gui_url.name)?;
            writeln!(f, "{INDENT3}description: {}", gui_url.description)?;
            writeln!(f, "{INDENT3}URL: {}", gui_url.url)?;
        }

        writeln!(f, "{INDENT3}jobs succeeded: {}", project.njobs_success)?;
        writeln!(f, "{INDENT3}jobs failed: {}", project.njobs_error)?;
        writeln!(f, "{INDENT3}elapsed time: {:.6}", project.elapsed_time)?;
        writeln!(f, "{INDENT3}cross-project ID: {}", project.external_cpid)?;

        Ok(())
    }
}

impl fmt::Display for Displayable<Task> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let task = &self.0;
        let scheduler_state = task
            .active_task
            .as_ref()
            .map_or(SchedulerState::Uninitialized, |t| t.scheduler_state.clone());
        let active_task_state = task
            .active_task
            .as_ref()
            .map_or(ActiveTaskState::Uninitialized, |at| at.active_task_state.clone());

        writeln!(f, "{INDENT3}name: {}", task.name)?;
        writeln!(f, "{INDENT3}WU name: {}", task.wu_name)?;
        writeln!(f, "{INDENT3}project URL: {}", task.project_url)?;
        writeln!(f, "{INDENT3}received: {}", FormattedTimestamp::new(task.received_time))?;
        writeln!(f, "{INDENT3}report deadline: {}", FormattedTimestamp::new(task.report_deadline))?;
        writeln!(f, "{INDENT3}ready to report: {}", task.ready_to_report)?;
        writeln!(f, "{INDENT3}state: {}", task.state)?;
        writeln!(f, "{INDENT3}scheduler state: {}", scheduler_state)?;
        writeln!(f, "{INDENT3}active_task_state: {}", active_task_state)?;
        writeln!(f, "{INDENT3}app version num: {}", task.version_num)?;
        writeln!(f, "{INDENT3}resources: {}",
            Some(task.resources.as_str()).filter(|rs| !rs.is_empty()).get_or_insert("1 CPU"))?;

        if task.state as isize >= 0
            && task.state as isize <= ResultClientState::FilesDownloaded as isize
        {
            if task.suspended_via_gui.into() {
                writeln!(f, "{INDENT3}suspended via GUI: yes")?;
            }
            writeln!(f, "{INDENT3}estimated CPU time remaining: {:.6}", task.estimated_cpu_time_remaining.0)?;
            if let Some(active_task) = &task.active_task {
                writeln!(f, "{INDENT3}elapsed task time: {:.6}", active_task.elapsed_time.0)?;
            }
        }

        if scheduler_state as isize > SchedulerState::Uninitialized as isize {
            if let Some(active_task) = &task.active_task {
                writeln!(f, "{INDENT3}slot: {}", active_task.slot)?;
                writeln!(f, "{INDENT3}PID: {}", active_task.pid)?;
                writeln!(f, "{INDENT3}CPU time at last checkpoint: {:.6}", active_task.checkpoint_cpu_time.0)?;
                writeln!(f, "{INDENT3}current CPU time: {:.6}", active_task.current_cpu_time.0)?;
                writeln!(f, "{INDENT3}fraction done: {:.6}", active_task.fraction_done)?;
                writeln!(f, "{INDENT3}swap size: {:.0} MB", to_mibi(active_task.swap_size))?;
                writeln!(f, "{INDENT3}working set size: {:.0} MB", to_mibi(active_task.working_set_size_smoothed))?;

                if active_task.bytes_sent > 0. || active_task.bytes_received > 0. {
                    writeln!(f, "{INDENT3}bytes sent: {:.0} received: {:.0}",
                        active_task.bytes_sent, active_task.bytes_received)?;
                }
            }
        }

        if task.state as isize > ResultClientState::FilesDownloaded as isize {
            writeln!(f, "{INDENT3}final CPU time: {:.6}", task.final_cpu_time.0)?;
            writeln!(f, "{INDENT3}final elapsed time: {:.6}", task.final_elapsed_time.0)?;
            writeln!(f, "{INDENT3}exit_status: {}", task.exit_status)?;
            writeln!(f, "{INDENT3}signal: {}", task.signal)?;
        }

        Ok(())
    }
}

impl fmt::Display for Displayable<Version> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
