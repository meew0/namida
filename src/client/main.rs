use super::{Command, Parameter, Session};
use crate::extc;
use ::libc;

pub unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *const libc::c_char) -> libc::c_int {
    let mut command: Command = Command {
        count: 0,
        text: [std::ptr::null::<libc::c_char>(); 10],
    };
    let vla = super::config::MAX_COMMAND_LENGTH as usize;
    let mut command_text: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
    let mut session: Option<Session> = None;
    let mut parameter: Parameter = Parameter::default();
    let mut argc_curr: libc::c_int = 1 as libc::c_int;
    let mut ptr_command_text: *mut libc::c_char =
        &mut *command_text.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_char;

    extc::fprintf(
        extc::stderr,
        b"Tsunami Client for protocol rev %X\nRevision: %s\nCompiled: %s %s\n\0" as *const u8
            as *const libc::c_char,
        crate::common::common::PROTOCOL_REVISION,
        b"v1.1 devel cvsbuild 43\0" as *const u8 as *const libc::c_char,
        b"Nov 16 2023\0" as *const u8 as *const libc::c_char,
        b"21:24:18\0" as *const u8 as *const libc::c_char,
    );
    loop {
        if argc <= 1 as libc::c_int || argc_curr >= argc {
            extc::fprintf(
                extc::stdout,
                b"tsunami> \0" as *const u8 as *const libc::c_char,
            );
            extc::fflush(extc::stdout);
            if (extc::fgets(
                command_text.as_mut_ptr(),
                super::config::MAX_COMMAND_LENGTH,
                extc::stdin,
            ))
            .is_null()
            {
                panic!("Could not read command input");
            }
        } else {
            while argc_curr < argc {
                if extc::strcasecmp(
                    *argv.offset(argc_curr as isize),
                    b"close\0" as *const u8 as *const libc::c_char,
                ) == 0
                    || extc::strcasecmp(
                        *argv.offset(argc_curr as isize),
                        b"quit\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    || extc::strcasecmp(
                        *argv.offset(argc_curr as isize),
                        b"exit\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    || extc::strcasecmp(
                        *argv.offset(argc_curr as isize),
                        b"bye\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    || extc::strcasecmp(
                        *argv.offset(argc_curr as isize),
                        b"help\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    || extc::strcasecmp(
                        *argv.offset(argc_curr as isize),
                        b"dir\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    extc::strcpy(command_text.as_mut_ptr(), *argv.offset(argc_curr as isize));
                    argc_curr += 1 as libc::c_int;
                    break;
                } else if extc::strcasecmp(
                    *argv.offset(argc_curr as isize),
                    b"connect\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if (argc_curr + 1 as libc::c_int) < argc {
                        extc::strcpy(ptr_command_text, *argv.offset(argc_curr as isize));
                        extc::strcat(
                            command_text.as_mut_ptr(),
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        extc::strcat(
                            command_text.as_mut_ptr(),
                            *argv.offset((argc_curr + 1 as libc::c_int) as isize),
                        );
                    } else {
                        extc::fprintf(
                            extc::stderr,
                            b"Connect: no host specified\n\0" as *const u8 as *const libc::c_char,
                        );
                        extc::exit(1 as libc::c_int);
                    }
                    argc_curr += 2 as libc::c_int;
                    break;
                } else if extc::strcasecmp(
                    *argv.offset(argc_curr as isize),
                    b"get\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if (argc_curr + 1 as libc::c_int) < argc {
                        extc::strcpy(ptr_command_text, *argv.offset(argc_curr as isize));
                        extc::strcat(
                            command_text.as_mut_ptr(),
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        extc::strcat(
                            command_text.as_mut_ptr(),
                            *argv.offset((argc_curr + 1 as libc::c_int) as isize),
                        );
                    } else {
                        extc::fprintf(
                            extc::stderr,
                            b"Get: no file specified\n\0" as *const u8 as *const libc::c_char,
                        );
                        extc::exit(1 as libc::c_int);
                    }
                    argc_curr += 2 as libc::c_int;
                    break;
                } else if extc::strcasecmp(
                    *argv.offset(argc_curr as isize),
                    b"set\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if (argc_curr + 2 as libc::c_int) < argc {
                        extc::strcpy(ptr_command_text, *argv.offset(argc_curr as isize));
                        extc::strcat(
                            command_text.as_mut_ptr(),
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        extc::strcat(
                            command_text.as_mut_ptr(),
                            *argv.offset((argc_curr + 1 as libc::c_int) as isize),
                        );
                        extc::strcat(
                            command_text.as_mut_ptr(),
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        extc::strcat(
                            command_text.as_mut_ptr(),
                            *argv.offset((argc_curr + 2 as libc::c_int) as isize),
                        );
                    } else {
                        extc::fprintf(
                            extc::stderr,
                            b"Connect: no host specified\n\0" as *const u8 as *const libc::c_char,
                        );
                        extc::exit(1 as libc::c_int);
                    }
                    argc_curr += 3 as libc::c_int;
                    break;
                } else {
                    extc::fprintf(
                        extc::stderr,
                        b"Unsupported command console command: %s\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(argc_curr as isize),
                    );
                    argc_curr += 1;
                }
            }
        }
        parse_command(&mut command, command_text.as_mut_ptr());

        if command.count as libc::c_int == 0 as libc::c_int {
            continue;
        }

        if let Err(err) = run_command(&mut command, &mut parameter, &mut session) {
            println!("Error while running command: {:?}", err);
        }
    }
}

unsafe fn run_command(
    command: &mut Command,
    parameter: &mut Parameter,
    session: &mut Option<Session>,
) -> anyhow::Result<()> {
    let mut found = true;

    if extc::strcasecmp(
        command.text[0 as libc::c_int as usize],
        b"connect\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        match super::command::command_connect(command, parameter) {
            Ok(new_session) => {
                *session = Some(new_session);
            }
            Err(err) => println!("Error in command_connect: {:?}", err),
        }
    } else if extc::strcasecmp(
        command.text[0 as libc::c_int as usize],
        b"set\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        super::command::command_set(command, parameter)?;
    } else if let Some(session) = session.as_mut() {
        if extc::strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"close\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            super::command::command_close(parameter, session)?;
        } else if extc::strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"get\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            super::command::command_get(command, parameter, session)?;
        } else if extc::strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"dir\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            super::command::command_dir(command, session)?;
        } else if extc::strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"help\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            super::command::command_help(command)?;
        } else if extc::strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"quit\0" as *const u8 as *const libc::c_char,
        ) == 0
            || extc::strcasecmp(
                command.text[0 as libc::c_int as usize],
                b"exit\0" as *const u8 as *const libc::c_char,
            ) == 0
            || extc::strcasecmp(
                command.text[0 as libc::c_int as usize],
                b"bye\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            super::command::command_quit(session);
        } else {
            found = false;
        }
    } else {
        found = false;
    }

    if !found {
        extc::fprintf(
            extc::stderr,
            b"Unrecognized command: '%s'.  Use 'HELP' for help.\n\n\0" as *const u8
                as *const libc::c_char,
            command.text[0 as libc::c_int as usize],
        );
    }

    Ok(())
}

pub unsafe fn parse_command(command: &mut Command, mut buffer: *mut libc::c_char) {
    command.count = 0 as libc::c_int as u8;
    while *(*extc::__ctype_b_loc()).offset(*buffer as libc::c_int as isize) as libc::c_int
        & extc::_ISspace as libc::c_int as libc::c_ushort as libc::c_int
        != 0
        && *buffer as libc::c_int != 0
    {
        buffer = buffer.offset(1);
    }
    while (command.count as libc::c_int) < 10 as libc::c_int && *buffer as libc::c_int != 0 {
        let fresh0 = command.count;
        command.count = (command.count).wrapping_add(1);
        command.text[fresh0 as usize] = buffer;
        while *buffer as libc::c_int != 0
            && *(*extc::__ctype_b_loc()).offset(*buffer as libc::c_int as isize) as libc::c_int
                & extc::_ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            buffer = buffer.offset(1);
        }
        while *buffer as libc::c_int != 0
            && *(*extc::__ctype_b_loc()).offset(*buffer as libc::c_int as isize) as libc::c_int
                & extc::_ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            let fresh1 = buffer;
            buffer = buffer.offset(1);
            *fresh1 = '\0' as i32 as libc::c_char;
        }
    }
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *const libc::c_char,
        ) as i32)
    }
}
