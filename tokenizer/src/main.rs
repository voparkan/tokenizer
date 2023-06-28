//tokenizer version 0.0.0
//42 school heilbronn hub Gang
//comunity project by Voparkan and Smelicha and crew

extern crate daemonize;
extern crate dirs;

use daemonize::Daemonize;
use std::path::Path;
use std::fs::File;

use crate::btl_support;

fn main()
{
    let run = true;
    let home_dir = dirs::home_dir().expect("REASON");
    let home_dir_path = Path::new(&home_dir);
    let stdout = File::create(home_dir_path.join("tokenizer").join("daemonizer.out")).unwrap();
    let stderr = File::create(home_dir_path.join("tokenizer").join("daemonizer.err")).unwrap();
    let pid_file = File::create(home_dir_path.join("tokenizer").join("daemonizer.pid"));
    let tokenizer = Daemonize::new()
        .pid_file(home_dir_path.join("tokenizer").join("daemonizer.pid")) // Every method except `new` and `start`
        .chown_pid_file(false)      // is optional, see `Daemonize` documentation
        .working_directory(home_dir_path.join("tokenizer")) // for default behaviour.
        .user("nobody")
        .group("nobody") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match pid_file
    {
        Ok(_) => {
            match tokenizer.start() {
                Ok(ok) => {
                    eprintln!("TOKENIZER! \n\nHome directory: {}  \n\nPid file: {}", home_dir.display(), ok);
                    let args: Vec<String> = std::env::args().collect();
                    let exit_code = system_support_btl::main_with_args(args.as_slice());
                    ::std::process::exit(exit_code);
                }
                Err(e) => eprintln!("TOKENIZER! \n\nError: {}", e),
            }
        }
        Err(error) => {
            panic!("Enviroment not ready {}", error);
        }
    }
}
