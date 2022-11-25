use std::fmt::Display;

use log::{debug, info};
use rand::Rng;
use subprocess::{PopenError, Redirection};

use crate::interface::{self, NHCommand};

pub trait NHRunnable {
    fn run(&self) -> anyhow::Result<()>;
}

impl NHRunnable for interface::NHCommand {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            NHCommand::Os(os_args) => os_args.run(),
            NHCommand::Clean(clean_args) => clean_args.run(),
            NHCommand::Home(home_args) => home_args.run(),
            s => todo!("Subcommand {s:?} not yet implemented!"),
        }
    }
}

pub fn run_command_capture(
    cmd: &Vec<&str>,
    message: Option<&str>,
) -> Result<String, subprocess::PopenError> {
    if let Some(m) = message {
        info!("{}", m);
    }

    debug!("{}", cmd.join(" "));

    let (head, tail) = cmd.split_at(1);
    let head = *head.first().unwrap();

    subprocess::Exec::cmd(head)
        .args(tail)
        .stdout(Redirection::Pipe)
        .capture()
        .map(|c| c.stdout_str().trim().to_owned())
}

pub fn run_command<S>(cmd: &Vec<&str>, message: Option<S>, dry: bool) -> Result<(), PopenError>
where
    S: AsRef<str> + std::fmt::Display,
{
    if let Some(m) = message {
        info!("{}", m);
    }

    debug!("{}", cmd.join(" "));

    if !dry {
        let (head, tail) = cmd.split_at(1);
        let head = *head.first().unwrap();

        let exit = subprocess::Exec::cmd(head).args(tail).popen()?.wait()?;

        if !exit.success() {
            let _msg = match exit {
                subprocess::ExitStatus::Exited(code) => code.to_string(),
                subprocess::ExitStatus::Signaled(code) => code.to_string(),
                _ => format!("Unknown error: {:?}", exit),
            };

            return Err(PopenError::LogicError("FIXME"));
        };
    }

    Ok(())
}

pub fn mk_temp<P>(prefix: P) -> String
where
    P: AsRef<str> + Display,
{
    let suffix_bytes: Vec<_> = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(10)
        .collect();

    let suffix = std::str::from_utf8(&suffix_bytes).unwrap();

    format!("{}{}", &prefix, &suffix)
}
