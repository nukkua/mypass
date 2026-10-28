use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("pancakes");
    let args = env::args().collect::<Vec<_>>();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("mypass: {msg}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    match args.get(1).map(|arg| arg.as_str()) {
        None | Some("ls") => {
            let dir = store_dir()?;
            println!("TODO: list {}", dir.display());
        }
        Some("show") => {
            let name = args.get(2).ok_or("usage: mypass show <name>")?;
            println!("TODO: show {}", name);
        }
        Some(cmd) => return Err(format!("unknown command: {cmd}")),
    }
    Ok(())
}

fn store_dir() -> Result<PathBuf, String> {
    if let Ok(dir) = env::var("PASSWORD_STORE_DIR") {
        return Ok(PathBuf::from(dir));
    }

    let home = env::var("HOME").map_err(|_| "HOME is not set".to_string())?;
    Ok(PathBuf::from(home).join(".password-store"))
}

