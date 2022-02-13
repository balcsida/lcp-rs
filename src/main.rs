use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
#[clap(global_setting(AppSettings::SubcommandRequiredElseHelp))]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]

struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Delete service(s)
    Delete {
        /// Name of the project to delete
        #[clap(short, long)]
        project: String,
        /// Environment name to delete (e.g. "dev")
        #[clap(short, long)]
        environment: String,
        /// Service name (e.g. "liferay")
        #[clap(short, long)]
        service: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    match &cli.command {
        Commands::Delete { project, environment, service } => {
            println!("'alcp delete' was used");

            println!("Project is: {:?}", project);
            println!("Environment is: {:?}", environment);
            println!("Service is: {:?}", service)
        }
    }
}