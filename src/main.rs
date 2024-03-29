use crate::crud::delete_email::DeleteEmailDbOps;
use crate::crud::get_email::GetEmailDbOps;
use crate::crud::get_mailbox::GetMailboxDbOps;
use clap::{Parser, Subcommand};
use command::open::open_save_location_invoices;
use crud::delete_mailbox::DeleteMailboxDbOps;
use db::connect::DatabaseConnection;
use dotenv::dotenv;
use email_parser::main::process_emails;
use email_sender::sender::send_emails;
use enums::{DbAction, OpenCommand};
use lazy_static::lazy_static;

use std::env::var;
extern crate imap;
extern crate native_tls;

pub mod command;
pub mod crud;
pub mod datemath;
pub mod db;
pub mod email_parser;
pub mod email_sender;
pub mod enums;
pub mod factories;
pub mod io;
pub mod rules;

lazy_static! {
    pub static ref COMPANY_EMAIL_SERVER: String =
        var("COMPANY_EMAIL_SERVER").expect("COMPANY_EMAIL_SERVER must be set.");
}
lazy_static! {
    pub static ref COMPANY_EMAIL_PORT: u16 = var("COMPANY_EMAIL_PORT")
        .expect("COMPANY_EMAIL_PORT must be set.")
        .parse()
        .expect("COMPANY_EMAIL_PORT must be a valid u16.");
}
lazy_static! {
    pub static ref COMPANY_EMAIL: String =
        var("COMPANY_EMAIL").expect("COMPANY_EMAIL must be set.");
}
lazy_static! {
    pub static ref COMPANY_EMAIL_PASSWORD: String =
        var("COMPANY_EMAIL_PASSWORD").expect("COMPANY_EMAIL_PASSWORD must be set.");
}
lazy_static! {
    pub static ref PRIVATE_EMAIL: String =
        var("PRIVATE_EMAIL").expect("COMPANY_PRIVATE must be set.");
}
lazy_static! {
    pub static ref PRIVATE_EMAIL_PASSWORD: String =
        var("PRIVATE_EMAIL_PASSWORD").expect("PRIVATE_EMAIL_PASSWORD must be set.");
}
lazy_static! {
    pub static ref S_EMAIL: String = var("S_EMAIL").expect("COMPANY_PRIVATE must be set.");
}
lazy_static! {
    pub static ref S_EMAIL_PASSWORD: String =
        var("S_EMAIL_PASSWORD").expect("PRIVATE_EMAIL_PASSWORD must be set.");
}

#[derive(Debug, Parser)]
#[command(name="antworker",version="0.1.0", about = "🐜 Your daily assistant that manages common tasks", author="Maciej Gierada, @mgierada, maciek.gierada@gmail.com", long_about = None, help_template("\
{author-with-newline}
{name}-{version} {about-with-newline} 
{usage-heading} {usage}

{all-args}{after-help}
"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(
        about = "Fetch all emails and save attachments in designated location for the current month."
    )]
    Emails,
    #[command(about = "Send all invoices for the current month to the designated email address.")]
    Send {
        #[arg(short, long, action, help = "Dry run, do not send emails.")]
        dry_run: bool,
    },
    #[command(about = "Open the designated location for the current month.")]
    Open {
        #[arg(
            short,
            long,
            action,
            help = "Define year and month of interest, e.g. 2024_01"
        )]
        year_month_or_year: Option<String>,
        #[arg(help = "Specify the type of invoices (income/outcome)")]
        invoice_type: String,
    },
    #[command(about = "Perform database operations.")]
    Db {
        #[arg(
            short,
            long,
            action,
            help = "Specify the mailbox to fetch historical emails from"
        )]
        mailbox: Option<String>,
        #[arg(
            short,
            long,
            action,
            help = "Define year and month of interest, e.g. 2024_01. If not provided, the current year and month will be used"
        )]
        year_month: Option<String>,
        #[arg(help = "Define the action to perform on the database, e.g. get, remove")]
        action: String,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_conn = DatabaseConnection;
    let args = Cli::parse();
    match args.command {
        Commands::Emails {} => {
            process_emails().await.unwrap();
        }
        Commands::Send { dry_run } => {
            if dry_run {
                println!("Dry run, not sending emails.");
                return send_emails(true);
            }
            send_emails(false)
        }
        Commands::Open {
            year_month_or_year,
            invoice_type,
        } => {
            let command = match invoice_type.as_str() {
                "income" => OpenCommand::Income,
                "outcome" => OpenCommand::Outcome,
                _ => {
                    println!("Unknown invoice type. Allowed values: income, outcome");
                    return;
                }
            };
            match command {
                OpenCommand::Income => {
                    open_save_location_invoices(&year_month_or_year.unwrap_or("".to_string()), true)
                }
                OpenCommand::Outcome => open_save_location_invoices(
                    &year_month_or_year.unwrap_or("".to_string()),
                    false,
                ),
            }
        }
        Commands::Db {
            action,
            mailbox,
            year_month,
        } => {
            let action_item = match action.as_str() {
                "get-email" => DbAction::GetEmail,
                "remove-email" => DbAction::RemoveEmail,
                "get-mailbox" => DbAction::GetMailbox,
                "remove-mailbox" => DbAction::RemoveMailbox,
                _ => {
                    println!("Unknown action. Allowed values: get-mailbox, remove-mailbox, get-email, remove-email");
                    return;
                }
            };
            match action_item {
                DbAction::GetEmail => {
                    db_conn.get_emails().await.unwrap();
                    return;
                }
                DbAction::RemoveEmail => {
                    db_conn.remove_emails().await.unwrap();
                }
                DbAction::GetMailbox => {
                    db_conn.get_mailboxes().await.unwrap();
                    return;
                }
                DbAction::RemoveMailbox => {
                    db_conn.remove_mailbox().await.unwrap();
                }
            }
            match (mailbox, year_month) {
                (Some(mailbox), Some(year_month)) => db_conn
                    .get_emails_current_year_month_mailbox(&mailbox, &year_month)
                    .await
                    .unwrap(),
                (Some(mailbox), None) => db_conn
                    .get_emails_current_year_month_mailbox(&mailbox, "")
                    .await
                    .unwrap(),
                _ => db_conn.get_emails_current_year_month().await.unwrap(),
            }
        }
    }
}
