use docopt;


const USAGE: &'static str = "
mig demo

Usage:
  mig server <host> <port>
  mig client <host> <port>
  mig --version
  mig (-h | --help)

Options:
  <host> Host to serve on or to connect to.
  <port> Port to serve on or to connect to.
  --version  Show version.
  -h --help  Show this screen.
";


#[derive(Debug, RustcDecodable)]
struct DocoptMigArgs {
    cmd_server: bool,
    cmd_client: bool,
    arg_host: String,
    arg_port: i32,
    flag_version: bool,
}


#[derive(Debug, Clone)]
pub enum MigCommand {
    Version,
    Server { host: String, port: i32 },
    Client { host: String, port: i32 },
}


fn parse_args() -> DocoptMigArgs {
    docopt::Docopt::new(USAGE)
    .and_then(|docopt| docopt.decode())
    .unwrap_or_else(|e| e.exit())
}


pub fn parse_command() -> MigCommand {
    match parse_args() {
        DocoptMigArgs { flag_version: true, .. } => MigCommand::Version,
        DocoptMigArgs { cmd_server: true, arg_host: host, arg_port: port, .. } =>
            MigCommand::Server { host: host, port: port },
        DocoptMigArgs { cmd_client: true, arg_host: host, arg_port: port, .. } =>
            MigCommand::Client { host: host, port: port },
        _ => {
            unreachable!();
        }
    }
}