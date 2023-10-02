use clap::{Parser, Subcommand};
use libjoinc::defs::*;
use libjoinc::error::Result;
use libjoinc::rpc::commands::*;
use libjoinc::rpc::connection;

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
    /// Show messages
    GetMessages {
        /// show messages with sequence number > seqno only
        #[arg(default_value = "0")]
        seqno: u32,
    },
    GetTasks {
        #[arg(default_value = "false")]
        active_only: bool,
    },
    /// Read the cc_config.xml file
    ReadCcConfig,
    /// Show the verion of this cli
    Version,
}

// TODO don't use Debug trait for printing the errors
fn main() {
    let cli = Cli::parse();

    if cli.command == CliCommand::Version {
        println!("Version: {JOINCCMD_VERSION}");
        std::process::exit(0);
    }

    let mut connection = connection::Connection::open(&cli.host, cli.port).unwrap_or_else(|err| {
        eprintln!("Failed to connect to BOINC client: {:?}", err);
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
        eprintln!("The command failed with: {:?}", err);
        std::process::exit(1);
    });
}

fn process_command(connection: &mut connection::Connection, command: CliCommand) -> Result<()> {
    match command {
        CliCommand::ClientVersion => {
            let mut cmd = ExchangeVersionsCommand::default();
            let version = cmd.execute(connection)?;
            println!(
                "Client version: {}.{}.{}",
                version.major, version.minor, version.release
            );
        }
        CliCommand::GetMessages { seqno } => {
            let mut cmd = GetMessagesCommand::new(seqno);
            let msgs = cmd.execute(connection)?;
            for msg in msgs {
                println!(
                    "{}: {} ({}) [{}] {}",
                    msg.seqno,
                    time_to_string(msg.timestamp),
                    msg.priority,
                    msg.project,
                    msg.body.trim()
                );
            }
        }
        CliCommand::GetTasks { active_only } => {
            let mut cmd = GetResultsCommand::new(active_only);
            let tasks = cmd.execute(connection)?;

            println!("");
            println!("======== Tasks ========");

            for (idx, task) in tasks.iter().enumerate() {
                let scheduler_state = task
                    .active_task
                    .as_ref()
                    .map_or(SchedulerState::Uninitialized, |t| t.scheduler_state.clone());
                let active_task_state = task
                    .active_task
                    .as_ref()
                    .map_or(ActiveTaskState::Uninitialized, |at| {
                        at.active_task_state.clone()
                    });

                println!(
                    "\
{}) -----------
   name: {}
   WU name: {}
   project URL: {}
   received: {}
   report deadline: {}
   ready to report: {}
   state: {}
   scheduler state: {}
   active_task_state: {}
   app version num: {}
   resources: {}",
                    idx + 1,
                    task.name,
                    task.wu_name,
                    task.project_url,
                    time_to_string(task.received_time),
                    time_to_string(task.report_deadline),
                    bool_to_string(task.ready_to_report),
                    task.state,
                    scheduler_state,
                    active_task_state,
                    task.version_num,
                    if task.resources.is_empty() {
                        "1 CPU"
                    } else {
                        &task.resources
                    }
                );

                {
                    let istate = task.state as isize;
                    if istate >= 0 && istate <= ResultClientState::FilesDownloaded as isize {
                        if task.suspended_via_gui {
                            println!("   suspended via GUI: yes");
                        }
                        println!(
                            "   estimated CPU time remaining: {:.6}",
                            task.estimated_cpu_time_remaining
                        );
                    }
                }

                if scheduler_state as isize > SchedulerState::Uninitialized as isize {
                    if let Some(active_task) = &task.active_task {
                        println!(
                            "   \
   CPU time at last checkpoint: {:.6}
   current CPU time: {:.6}
   fraction done: {:.6}
   swap size: {:.0} MB
   working set size: {:.0} MB",
                            active_task.checkpoint_cpu_time,
                            active_task.current_cpu_time,
                            active_task.fraction_done,
                            to_mibi(active_task.swap_size),
                            to_mibi(active_task.working_set_size_smoothed)
                        );

                        if active_task.bytes_sent > 0. || active_task.bytes_received > 0. {
                            println!(
                                "   bytes sent: {:.0} received: {:.0}",
                                active_task.bytes_sent, active_task.bytes_received
                            );
                        }
                    }
                }

                if task.state as isize > ResultClientState::FilesDownloaded as isize {
                    println!(
                        "   \
   final CPU time: {}
   final elapsed time: {}
   exit_status: {}
   signal: {}",
                        task.final_cpu_time, task.final_elapsed_time, task.exit_status, task.signal
                    );
                }
            }
        }
        CliCommand::ReadCcConfig => ReadCCConfigCommand::default().execute(connection)?,
        CliCommand::Version => panic!("Should've never reached this line"),
    };

    Ok(())
}

fn bool_to_string(b: bool) -> &'static str {
    match b {
        false => "no",
        true => "yes",
    }
}

fn time_to_string(timestamp: f64) -> String {
    Some(timestamp)
        .filter(|&t| t > 0.)
        .and_then(|t| NaiveDateTime::from_timestamp_opt(t as i64, 0))
        .map(|t| {
            Local
                .from_utc_datetime(&t)
                .with_timezone(&Local)
                .format("%c")
                .to_string()
        })
        .unwrap_or("---".to_string())
}

fn to_mibi(d: f64) -> f64 {
    d / (1024. * 1024.)
}
