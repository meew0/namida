use std::io::Write;

use super::{Parameter, Session};

/// The main function for the client running in interactive mode.
#[allow(clippy::missing_errors_doc)]
pub fn interactive(mut parameter: Parameter) -> anyhow::Result<()> {
    // show version / build information
    eprintln!(
        "namida client for protocol revision {}\nVersion: {} (revision {})\nCompiled: {}\n",
        crate::version::NAMIDA_PROTOCOL_REVISION,
        crate::version::NAMIDA_VERSION,
        &crate::version::GIT_HASH[0..7],
        crate::version::COMPILE_DATE_TIME,
    );

    let mut session: Option<Session> = None;

    // while the command loop is still running
    loop {
        // present the prompt
        print!("namida> ");
        std::io::stdout().flush()?;

        // read next command
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        // parse the command
        let command = parse_command(&input);

        // make sure we have at least one word
        if command.is_empty() {
            continue;
        }

        let result = run_command(&command, &mut parameter, &mut session);
        if let Err(err) = result {
            println!("Error while running command: {err:?}");
        }
    }
}

fn run_command(
    command: &[&str],
    parameter: &mut Parameter,
    session: &mut Option<Session>,
) -> anyhow::Result<()> {
    let mut found = true;
    let instruction = command[0];

    if instruction.eq_ignore_ascii_case("connect") {
        let connect_result = super::command::connect(command, parameter);
        match connect_result {
            Ok(new_session) => {
                *session = Some(new_session);
            }
            Err(err) => println!("Error in command_connect: {err:?}"),
        }
    } else if instruction.eq_ignore_ascii_case("set") {
        super::command::set(command, parameter)?;
    } else if instruction.eq_ignore_ascii_case("help") {
        super::command::help(command);
    } else if instruction.eq_ignore_ascii_case("quit")
        || instruction.eq_ignore_ascii_case("exit")
        || instruction.eq_ignore_ascii_case("bye")
    {
        super::command::quit();
    } else if instruction.eq_ignore_ascii_case("close") {
        super::command::close(parameter, session.take())?;
    } else if let Some(session) = session.as_mut() {
        if instruction.eq_ignore_ascii_case("get") {
            super::command::get(command, parameter, session)?;
        } else if instruction.eq_ignore_ascii_case("dir") {
            super::command::dir(command, session)?;
        } else {
            found = false;
        }
    } else {
        found = false;
    }

    if !found {
        eprintln!(
            "Unrecognized command: '{instruction}'.  Use 'HELP' for help. Some commands are only available after connecting to a server.",
        );
    }

    Ok(())
}

#[must_use]
fn parse_command(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
