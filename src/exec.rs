#[allow(dead_code)]
mod ports;

use anyhow::Result;
use serde::Deserialize;
use std::{
    env::{args, current_exe, var},
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use crate::ports::MITM_PORT;

use wait_timeout::ChildExt;

#[derive(Deserialize)]
struct RiotInstalls {
    rc_default: PathBuf,
}

fn add_underscore(path: PathBuf) -> Option<PathBuf> {
    let mut path = path.clone();
    let file_name = path.file_name()?.to_str()?;
    let new_name = format!("_{}", file_name);
    path.set_file_name(new_name);
    Some(path)
}

fn get_riot_client() -> Result<PathBuf> {
    let data_env = var("ProgramData")?;
    let program_data = Path::new(&data_env);
    let riot_path = program_data
        .join("Riot Games")
        .join("RiotClientInstalls.json");
    let mut f = File::open(riot_path)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    let installs: RiotInstalls = serde_json::from_str(&data)?;
    let image = current_exe()?;
    if image == installs.rc_default {
        let new_path = add_underscore(installs.rc_default).unwrap();
        return Ok(new_path);
    }
    Ok(installs.rc_default)
}

fn rm_args(args: &mut Vec<String>, arg_to_remove: &str) -> Option<String> {
    let index = args.iter().position(|arg| arg.starts_with(arg_to_remove))?;
    Some(args.remove(index))
}

fn parse_args() -> Vec<String> {
    let mut args = args().collect::<Vec<_>>();
    args.remove(0);
    rm_args(&mut args, "--client-config-url=");
    let client_config = format!("--client-config-url=http://localhost:{}", MITM_PORT);
    args.push(client_config);
    args
}

fn launch_riot() -> std::process::Child {
    let riot_exe = get_riot_client().expect("Riot games not installed");
    let args = parse_args();
    Command::new(riot_exe).args(args).spawn().unwrap()
}

fn launch_serv() -> std::process::Child {
    let exe = "target\\debug\\riot_middleware.exe";
    Command::new(exe).spawn().unwrap()
}

fn main() -> Result<()> {
    let mut serv = launch_serv();
    let mut riot = launch_riot();
    let exit_write = Arc::new(AtomicBool::new(false));
    let exit_read = exit_write.clone();
    let _ =
        ctrlc::set_handler(move || exit_write.store(true, std::sync::atomic::Ordering::Relaxed));
    loop {
        let one_sec = Duration::from_secs(1);
        let finished = riot.wait_timeout(one_sec).unwrap().is_some();
        if finished {
            break;
        }
        if exit_read.load(std::sync::atomic::Ordering::Relaxed) {
            let _ = riot.kill();
            let _ = serv.kill();
        }
    }
    let _ = serv.kill();
    Ok(())
}
