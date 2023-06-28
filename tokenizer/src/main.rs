//tokenizer version 0.0.0
//42 school heilbronn hub Gang
//comunity project by Voparkan and Smelicha and crew

extern crate daemonize;
extern crate dirs;

use daemonize::Daemonize;
use std::path::Path;
use std::fs::File;
use std::time::Instant;
use std::{thread, time};

fn main()
{
    let run = true;
    let loop_duration = time::Duration::from_millis(8000);
    let home_dir = dirs::data_dir().expect("REASON");
    let home_dir_path = Path::new(&home_dir);
    let tokenizer_path = home_dir_path.join("tokenizer");
    let stdout_path = tokenizer_path.join("daemonizer.out");
    let stderr_path = tokenizer_path.join("daemonizer.err");
    let pid_path = tokenizer_path.join("daemonizer.pid");
    let _dir = std::fs::create_dir_all(&tokenizer_path).expect("tokenizer dir was not created");
    let stdout = File::create(&stdout_path).unwrap();
    let stderr = File::create(&stderr_path).unwrap();
    let pid_file = File::create(&pid_path);
    let tokenizer = Daemonize::new()
        .pid_file(pid_path) // Every method except `new` and `start`
        .chown_pid_file(false)      // is optional, see `Daemonize` documentation
        .working_directory(&tokenizer_path) // for default behaviour.
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
                    println!("TOKENIZER! \n\nHome directory: {}  \n\nStdout file: {}", stdout_path.display(), ok);
                    let args: Vec<String> = std::env::args().collect();
                    let exit_code = system_support_btl::main_with_args(args.as_slice());
                    ::std::process::exit(exit_code);
                }
                Err(e) => eprintln!("TOKENIZER! \n\nError: {} \nStderr file: {}", e, stderr_path.display()),
            }
        }
        Err(ref error) => {
            panic!("Enviroment not ready {} \ndir: {:#?}", error, pid_file);
        }
    }
    while run {
        thread::sleep(loop_duration);
        let now = Instant::now();
        println!("TOKENIZER time: {:?}", now.elapsed());

    }
}
