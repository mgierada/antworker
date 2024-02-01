use clap::{Parser, Subcommand};
use command::open::{open_save_location_income_invoices, open_save_location_invoices};
use db::email::{get_emails, get_emails_current_year_month, get_emails_current_year_month_mailbox};
use dotenv::dotenv;
use email_parser::main::process_emails;
use email_sender::sender::send_emails;
use lazy_static::lazy_static;

use std::env::var;
extern crate imap;
extern crate native_tls;

pub mod command;
pub mod datemath;
pub mod db;
pub mod email_parser;
pub mod email_sender;
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
        year_month: Option<String>,
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
        #[arg(short, long, action, help = "Return all stored emails history")]
        all: bool,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
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
        Commands::Open { year_month, invoice_type } => {
            match invoice_type.as_str() {
                "income" => open_save_location_income_invoices(&year_month.unwrap_or("".to_string())),
                "outcome" => open_save_location_invoices(&year_month.unwrap_or("".to_string())),
                _ => println!("Unknown invoice type")
            }
                
                
        }
        Commands::Db { all, mailbox } => {
            if all {
                get_emails().await.unwrap();
            }
            if mailbox.is_some() {
                get_emails_current_year_month_mailbox(&mailbox.unwrap())
                    .await
                    .unwrap();
            } else {
                get_emails_current_year_month().await.unwrap()
            }
        }
    }
}
