use rustyline::error::ReadlineError;
use rustyline::Editor;

pub mod commands;
mod cli;

use crate::controllers;
use controllers::BleController;
use std::error::Error;

pub struct Repl<'a> {
    bt: &'a mut dyn BleController,
    editor: Editor<()>,
}

impl Repl<'_> {
    pub async fn new(bt: &mut dyn BleController) -> Repl {
        Repl {
            bt,
            editor: Editor::<()>::new().unwrap(),
        }
    }
    fn get_line(&mut self) -> String {
        let readline = self.editor.readline(">> ");
        match readline {
            Ok(line) => {
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                std::process::exit(exitcode::OSERR);
            }
            Err(ReadlineError::Eof) => {
                println!("eof");
                std::process::exit(exitcode::OK);
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }


    async fn execute_command(&mut self, matches: clap::ArgMatches) -> Result<(), Box<dyn Error>> {
        match matches.subcommand() {
            Some(("quit", _)) => {
                println!("EOF, bye");
                std::process::exit(exitcode::OK);
            }

            Some(("scan", mt)) => {
                let show_all = mt.contains_id("all");

                if mt.contains_id("list") {
                    return commands::scan::print_scan_list(&self.bt.get_scan_list(), show_all);
                }

                let timeout = *mt.get_one::<usize>("timeout").unwrap();

                commands::scan::run(self.bt, timeout, true, show_all).await?;
            }


            Some(("connect", mt)) => {
                if mt.contains_id("id") {
                    let index = *mt.get_one::<usize>("id").unwrap();
                    commands::connect::by_index(self.bt, index).await?;
                }
            }

            Some(("disconnect", _mt)) => {
                if !self.bt.is_connected() {
                    Err("Error: You must be connected to a peripheral to run this command")?;
                } else {
                    commands::disconnect::run(self.bt).await?;
                }
            }
            _ => {
                eprintln!("Error: Unknown command: '{:?}'", matches);
            }
        }
        Ok(())
    }

    pub async fn start(&mut self) -> ! {
        loop {
            let line = self.get_line();

            if line.trim().is_empty() {
                continue;
            }

            let args = match shlex::split(&line).ok_or("Error: Invalid Quotes") {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };

            let matches = cli::cli().try_get_matches_from(&args);

            if matches.is_err() {
                println!("{}", matches.unwrap_err());
                continue;
            } else {
                match self.execute_command(matches.unwrap()).await {
                    Ok(_) => (),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}
