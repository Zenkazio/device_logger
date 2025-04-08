use clap::{Arg, ArgAction, Command};

pub fn is_server() -> bool {
    let server_arg = Arg::new("server")
        .action(ArgAction::SetTrue)
        .long("server")
        .short('s')
        .help("Startet die Anwendung als Server (default ist Client)");
    let matches = Command::new("server").arg(server_arg).get_matches();

    matches.get_flag("server")
}
