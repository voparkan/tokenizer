use clap::{arg, Arg, Command};

pub fn cli() -> Command<'static> {
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";

    const COMMAND_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("repl")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("COMMAND")
        .subcommand_help_heading("COMMANDS")
        .help_template(PARSER_TEMPLATE)

        // quit
        .subcommand(
            Command::new("quit")
            .alias("exit")
            .about("Quit the REPL")
            .help_template(COMMAND_TEMPLATE))

        // scan
        .subcommand(
            Command::new("scan")
            .about("Search for BLE devices around")
            .args(&[
                Arg::new("timeout").help("Time to scan in seconds").required(false).default_value("5").value_parser(clap::value_parser!(usize)),
                arg!(-a --all ... "Show unnamed peripheral"),
                arg!(-l --list ... "Show last scan list (doesn't run a new scan)"),
            ]).help_template(COMMAND_TEMPLATE))

        // connect
        .subcommand(
            Command::new("connect")
            .about("Connect to a BLE peripheral")
            .args(&[
                arg!(-i --id ... "Connection using the id of the peripheral in the scan list").takes_value(true).exclusive(true).required(true).value_parser(clap::value_parser!(usize)),
                Arg::new("identifier").help("Parse identifier and use it to connect with name, mac or id").exclusive(true).required(true),
            ]).help_template(COMMAND_TEMPLATE))

        // disconnect
        .subcommand(
            Command::new("disconnect")
            .about("Disconnect from BLE peripheral")
            .help_template(COMMAND_TEMPLATE))

}
