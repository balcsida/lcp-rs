use clap::{Command, crate_description, crate_version};
mod commands;

fn cli() -> Command {
    Command::new("lcp")
        .about("A fictional versioning CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        // .subcommand(
        //     Command::new("deploy")
        //         .about("Deploy to DXP Cloud")
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("list")
        //         .about("Show list of projects and services")
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("log")
        //         .about("Show logs of project, service or instance")
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("scale")
        //         .about("Scale a service")
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("restart")
        //         .about("Restart a service")
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("shell")
        //         .about("Open a shell to a service")
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("login")
        //         .about("Login to DXP Cloud")
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("logout")
        //         .about("Logout from DXP Cloud")
        //         .arg_required_else_help(true),
        // )
        .subcommand(
            Command::new("docs")
                .about("Open the documentation of DXP Cloud in your browser"),
        )
        // .subcommand(
        //     Command::new("remote")
        //         .about("Configure DXP Cloud remotes")
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("update")
        //         .about("Update DXP Cloud CLI")
        //         .arg_required_else_help(true),
        // )
        .subcommand(
            Command::new("version")
                .about("Show current CLI version")
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("docs", _sub_matches)) => {
            println!(
                "Docs opened on your browser."
            );
            open::that("https://help.liferay.com/hc/en-us/categories/360000868032").unwrap();
        }

        // If it matches the version subcommand, print the version from the cargo.toml file
        // and the architecture of the binary
        Some(("version", _sub_matches)) => {
            println!(
                "{} version {} ({})",
                crate_description!(),
                crate_version!(),
                std::env::consts::ARCH
            );
        }

        // If it matches the list subcommand, print the list of projects and services
        // use the module from commands/list.rs
        Some(("list", sub_matches)) => {
            commands::list();
        }

        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()

    }
