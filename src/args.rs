use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Path to folder
    #[arg(long)]
    pub path: String,
    /// Password
    #[arg(long)]
    pub password: String,
    #[arg(long)]
    /// Password
    pub size: String,
}
