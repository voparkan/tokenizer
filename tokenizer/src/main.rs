//tokenizer version 0.0.0
//42 school heilbronn hub Gang
//comunity project by Voparkan and Smelicha and crew

extern crate daemonize;
extern crate dirs;

use daemonize::Daemonize;
use std::path::Path;
use std::fs::File;

fn main() {
    let run = true;
    let homedir = Path::new(dirs::home_dir());
    let stdout = File::create(homedir.join("/tokenizer/daemonizer.out")).unwrap();
    let stderr = File::create(homedir.join("/tokenizer/daemonizer.err")).unwrap();
    File::create(homedir.join("/tokenizer/daemonizer.pid"));

    let tokenizer = Daemonize::new()
        .pid_file(homedir.join("/tokenizer/daemonizer.pid")) // Every method except `new` and `start`
        .chown_pid_file(false)      // is optional, see `Daemonize` documentation
        .working_directory(homedir.join("/tokenizer")) // for default behaviour.
        .user("nobody")
        .group("nobody") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match tokenizer.start() {
        Ok(_) => {
            while run {
                eprintln!("Hello, world! {}", dirs::home_dir().expect("expected home directory").display());
            }
        }
        Err(e) => eprintln!("Error {}", e),
    }
}

// fn main()
// {
//     let run = true;
//     let stdout = File::create("~/.tokenizer/daemonizer.out"); //.join(".tokenizer").join("daemonizer.out")
//     let stderr = File::create("~/.tokenizer/daemonizer.err"); //.join(".tokenizer").join("daemonizer.err")
//
//     let daemon = Daemonize::new()
//         .pid_file("~/.tokenizer/tokenizer.pid") //dirs::home_dir().join(".tokenizer").join("tokenizer.pid")
//         .chown_pid_file(true)
//         .working_directory("~/.tokenizer") //dirs::home_dir().join(".tokenizer")
//         .user("tokenizer")
//         .group("daemon")
//         .group(2)
//         .umask(0o777)
//         .stdout(stdout)
//         .stderr(stderr)
//         .privileged_action(|| "Executed before drop priviledges");
//
//     match daemon.start() {
//         Ok(_) => {
//             while run {
//                 println!("Hello, world!");
//             }
//         }
//         Err(e) => eprintln!("Error {}", e),
//     }
// }
