use std::io::Write;

use super::{Parameter, Session};

pub fn interactive(mut parameter: Parameter) {
    // TODO: automatically generate these
    let compile_date = "Nov 16 2023";
    let compile_time = "21:24:18";

    eprintln!(
        "namida client for protocol rev {:X}\nRevision: {}\nCompiled: {} {}\n\0",
        crate::common::PROTOCOL_REVISION,
        crate::common::NAMIDA_VERSION,
        compile_date,
        compile_time
    );

    let mut session: Option<Session> = None;

    loop {
        print!("namida> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read command input");

        let command = parse_command(&input);

        if command.is_empty() {
            continue;
        }

        let result = run_command(&command, &mut parameter, &mut session);
        if let Err(err) = result {
            println!("Error while running command: {:?}", err);
        }
    }
}

fn run_command(
    command: &[&str],
    parameter: &mut Parameter,
    session: &mut Option<Session>,
) -> anyhow::Result<()> {
    let mut found = true;

    if command[0].eq_ignore_ascii_case("connect") {
        let connect_result = unsafe { super::command::command_connect(command, parameter) };
        match connect_result {
            Ok(new_session) => {
                *session = Some(new_session);
            }
            Err(err) => println!("Error in command_connect: {:?}", err),
        }
    } else if command[0].eq_ignore_ascii_case("set") {
        unsafe {
            super::command::command_set(command, parameter)?;
        }
    } else if command[0].eq_ignore_ascii_case("help") {
        unsafe {
            super::command::command_help(command)?;
        }
    } else if command[0].eq_ignore_ascii_case("quit")
        || command[0].eq_ignore_ascii_case("exit")
        || command[0].eq_ignore_ascii_case("bye")
    {
        super::command::command_quit(session);
    } else if let Some(session) = session.as_mut() {
        if command[0].eq_ignore_ascii_case("close") {
            unsafe {
                super::command::command_close(parameter, session)?;
            }
        } else if command[0].eq_ignore_ascii_case("get") {
            unsafe {
                super::command::command_get(command, parameter, session)?;
            }
        } else if command[0].eq_ignore_ascii_case("dir") {
            unsafe {
                super::command::command_dir(command, session)?;
            }
        } else {
            found = false;
        }
    } else {
        found = false;
    }

    if !found {
        eprintln!(
            "Unrecognized command: '{}'.  Use 'HELP' for help.",
            command[0],
        );
    }

    Ok(())
}

pub fn parse_command(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
