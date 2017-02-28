mod errors;

use docopt;


const USAGE: &'static str = "
mig demo

Usage:
  mig server <address>
  mig client <address>
  mig bench
  mig --version
  mig (-h | --help)

Options:
  <address> Address to serve on or to connect to in form: hostname:port
  --version  Show version.
  -h --help  Show this screen.
";


#[derive(Debug, RustcDecodable)]
struct DocoptMigArgs {
    cmd_server: bool,
    cmd_client: bool,
    cmd_bench: bool,
    arg_address: String,
    flag_version: bool,
}


#[derive(Debug, Clone)]
pub enum MigCommand {
    Version,
    Bench,
    Server { address: String },
    Client { address: String },
}


pub fn parse_command() -> errors::Result<MigCommand> {
    let docopt_args: DocoptMigArgs =
        docopt::Docopt::new(USAGE)?
        .decode()?;

    match docopt_args {
        DocoptMigArgs { flag_version: true, .. } =>
            Ok(MigCommand::Version),
        DocoptMigArgs { cmd_bench: true, .. } =>
            Ok(MigCommand::Bench),
        DocoptMigArgs { cmd_server: true, arg_address: address, .. } =>
            Ok(MigCommand::Server { address: address }),
        DocoptMigArgs { cmd_client: true, arg_address: address, .. } =>
            Ok(MigCommand::Client { address: address }),
        _ => {
            unreachable!();
        }
    }
}