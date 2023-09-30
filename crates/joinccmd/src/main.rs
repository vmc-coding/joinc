use clap::{Parser, Subcommand};
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
        #[arg(default_value="0")]
        seqno: u32,
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
        CliCommand::GetMessages{ seqno } => {
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
        CliCommand::ReadCcConfig => ReadCCConfigCommand::default().execute(connection)?,
        CliCommand::Version => panic!("Should've never reached this line"),
    };

    Ok(())
}

fn time_to_string(timestamp: i64) -> String {
    Some(timestamp)
        .filter(|&t| t > 0)
        .and_then(|t| NaiveDateTime::from_timestamp_opt(t, 0))
        .map(|t| {
            Local
                .from_utc_datetime(&t)
                .with_timezone(&Local)
                .to_string()
        })
        .unwrap_or("---".to_string())
}
