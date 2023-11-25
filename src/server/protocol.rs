use std::{
    ffi::CString,
    io::{Seek, SeekFrom},
    net::ToSocketAddrs,
    os::fd::AsRawFd,
    time::Instant,
};

use crate::{
    datagram::BlockType,
    extc,
    message::{
        ClientToServer, DirListStatus, FileRequestError, ServerToClient, TransmissionControl,
    },
    types::{BlockIndex, FileSize},
};
use ::libc;
use anyhow::bail;

use super::{Parameter, Session, Transfer};

pub fn ttp_accept_retransmit(
    session: &mut Session,
    parameter: &Parameter,
    retransmission: TransmissionControl,
    datagram_block_buffer: &mut [u8],
    datagram_buffer: &mut [u8],
    iteration: &mut u32,
) -> anyhow::Result<()> {
    match retransmission {
        TransmissionControl::SubmitErrorRate(error_rate) => {
            if error_rate > session.properties.error_rate {
                let mut factor1: f64 = 1.0f64 * session.properties.slower.numerator as f64
                    / session.properties.slower.denominator as f64
                    - 1.0f64;
                let mut factor2: f64 = (1.0f64 + error_rate.0 as f64
                    - session.properties.error_rate.0 as f64)
                    / (100000.0f64 - session.properties.error_rate.0 as f64);
                session.transfer.ipd_current *= 1.0f64 + factor1 * factor2;
            } else {
                session.transfer.ipd_current *= session.properties.faster.numerator as f64
                    / session.properties.faster.denominator as f64;
            }

            session.transfer.ipd_current = session
                .transfer
                .ipd_current
                .clamp(session.properties.ipd_time as f64, 10000.0);

            dbg!(session.transfer.ipd_current);

            /*if (if session.transfer.ipd_current < 10000.0f64 {
                session.transfer.ipd_current
            } else {
                10000.0f64
            }) > session.properties.ipd_time as f64
            {
                if session.transfer.ipd_current < 10000.0f64 {
                    session.transfer.ipd_current
                } else {
                    10000.0f64
                }
            } else {
                session.properties.ipd_time as f64
            };*/

            let stats_line = format!(
                "{:6} {:3.2}µs {:5}µs {:7} {:6.2} {:3}\n",
                error_rate.0,
                session.transfer.ipd_current,
                session.properties.ipd_time,
                session.transfer.block.0,
                100.0f64 * session.transfer.block.0 as f64
                    / session.properties.block_count.0 as f64,
                session.session_id,
            );

            if *iteration % 23 == 0 {
                println!(" erate     ipd  target   block   %done srvNr");
            }
            *iteration += 1;

            print!("{}", stats_line);

            if parameter.transcript_yn {
                crate::common::transcript_warn_error(super::transcript::xscript_data_log_server(
                    session,
                    stats_line.as_str(),
                ));
            }
        }
        TransmissionControl::RestartAt(block) => {
            if block.is_zero() || block > session.properties.block_count {
                bail!("Attempt to restart at illegal block {}", block.0);
            } else {
                session.transfer.block = block;
            }
        }
        TransmissionControl::Retransmit(block) => {
            let datagram = super::io::build_datagram(
                session,
                block,
                BlockType::Retransmission,
                datagram_block_buffer,
            )?;
            datagram.write_to(datagram_buffer);

            unsafe {
                let status = extc::sendto(
                    session.transfer.udp_fd,
                    datagram_buffer.as_ptr() as *const libc::c_void,
                    (6 as libc::c_int as u32).wrapping_add(session.properties.block_size.0) as u64,
                    0 as libc::c_int,
                    extc::__CONST_SOCKADDR_ARG {
                        __sockaddr__: session.transfer.udp_address,
                    },
                    session.transfer.udp_length,
                ) as libc::c_int;
                if status < 0 as libc::c_int {
                    bail!("Could not retransmit block {}", block.0);
                }
            }
        }
        _ => {
            bail!(
                "Received unknown retransmission request: {:?}",
                retransmission
            );
        }
    }
    Ok(())
}

pub fn ttp_authenticate_server(session: &mut Session, mut secret: &[u8]) -> anyhow::Result<()> {
    use rand::Rng;

    let mut random: [u8; 64] = [0; 64];
    rand::thread_rng().fill(&mut random);

    session.write(ServerToClient::AuthenticationChallenge(random))?;

    let server_digest: [u8; 16] = crate::common::prepare_proof(&mut random, secret).into();

    let ClientToServer::AuthenticationResponse(client_digest) = session.read()? else {
        bail!("Expected authentication response");
    };

    if server_digest != client_digest {
        session.write(ServerToClient::AuthenticationStatus(false))?;
        bail!("Authentication failed");
    }

    session.write(ServerToClient::AuthenticationStatus(true))?;
    Ok(())
}

pub fn ttp_negotiate_server(session: &mut Session) -> anyhow::Result<()> {
    let mut server_revision = crate::common::PROTOCOL_REVISION;

    session.write(server_revision)?;
    let client_revision: u32 = session.read()?;

    if client_revision != server_revision {
        bail!("Protocol negotiation failed");
    }

    Ok(())
}

pub unsafe fn ttp_open_port_server(
    session: &mut Session,
    parameter: &Parameter,
) -> anyhow::Result<()> {
    let mut address: *mut extc::sockaddr = std::ptr::null_mut::<extc::sockaddr>();
    let mut ipv6_yn: bool = parameter
        .bind
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap()
        .is_ipv6();
    if let Some(client) = &parameter.client {
        let mut result: *mut extc::addrinfo = std::ptr::null_mut::<extc::addrinfo>();
        let mut _errmsg: [libc::c_char; 256] = [0; 256];
        let client_c = CString::new(client.as_str()).unwrap();
        let mut status_0: libc::c_int = extc::getaddrinfo(
            client_c.as_ptr(),
            std::ptr::null::<libc::c_char>(),
            std::ptr::null::<extc::addrinfo>(),
            &mut result,
        );
        if status_0 != 0 {
            bail!(
                "error in getaddrinfo: {}",
                extc::gai_strerror_wrap(status_0),
            );
        }
        ipv6_yn = (*result).ai_family == 10 as libc::c_int;
        // session.properties.ipv6_yn = ipv6_yn;
        session.transfer.udp_length = (*result).ai_addrlen;
        address = extc::malloc((*result).ai_addrlen as libc::c_ulong) as *mut extc::sockaddr;
        if address.is_null() {
            panic!("Could not allocate space for UDP socket address");
        }
        extc::memcpy(
            address as *mut libc::c_void,
            (*result).ai_addr as *const libc::c_void,
            (*result).ai_addrlen as libc::c_ulong,
        );
        if !((*result).ai_canonname).is_null() {
            extc::printf(
                b"Sending data to: %s\n\0" as *const u8 as *const libc::c_char,
                (*result).ai_canonname,
            );
        }
        extc::freeaddrinfo(result);
    } else {
        session.transfer.udp_length = (if ipv6_yn as libc::c_int != 0 {
            ::core::mem::size_of::<extc::sockaddr_in6>() as libc::c_ulong
        } else {
            ::core::mem::size_of::<extc::sockaddr_in>() as libc::c_ulong
        }) as extc::socklen_t;
        address = extc::malloc(session.transfer.udp_length as libc::c_ulong) as *mut extc::sockaddr;
        if address.is_null() {
            panic!("Could not allocate space for UDP socket address");
        }
        extc::getpeername(
            session.client.as_raw_fd(),
            extc::__SOCKADDR_ARG {
                __sockaddr__: address,
            },
            &mut session.transfer.udp_length,
        );
    }

    let ClientToServer::UdpPort(port) = session.read()? else {
        bail!("Expected UDP port number");
    };

    if ipv6_yn {
        (*(address as *mut extc::sockaddr_in6)).sin6_port = port;
    } else {
        (*(address as *mut extc::sockaddr_in)).sin_port = port;
    }
    if parameter.verbose_yn {
        extc::printf(
            b"Sending to client port %d\n\0" as *const u8 as *const libc::c_char,
            extc::__bswap_16(port) as libc::c_int,
        );
    }
    session.transfer.udp_fd = super::network::create_udp_socket_server(parameter)?;
    session.transfer.udp_address = address;
    Ok(())
}

pub fn ttp_open_transfer_server(
    session: &mut Session,
    parameter: &Parameter,
) -> anyhow::Result<()> {
    session.transfer = Transfer::default();

    let mut request: ClientToServer = session.read()?;

    // Check if a file list is being requested, either Dir or Multi
    match request {
        ClientToServer::DirList => {
            session.write(ServerToClient::DirListHeader {
                status: DirListStatus::Ok,
                num_files: parameter
                    .files
                    .len()
                    .try_into()
                    .expect("File count overflow"),
            })?;

            for file_metadata in &parameter.files {
                session.write(ServerToClient::DirListFile(file_metadata.clone()))?;
            }

            let ClientToServer::DirListEnd = session.read()? else {
                bail!("Expected acknowledgment of file listing");
            };

            bail!("File list sent!");
        }
        ClientToServer::MultiRequest => {
            session.write(ServerToClient::MultiFileCount(
                parameter
                    .files
                    .len()
                    .try_into()
                    .expect("File count overflow"),
            ))?;

            let ClientToServer::MultiAcknowledgeCount = session.read()? else {
                bail!("Expected acknowledgment of file count");
            };

            for file_metadata in &parameter.files {
                session.write(ServerToClient::MultiFile(file_metadata.clone()))?;
            }

            let ClientToServer::MultiEnd = session.read()? else {
                bail!("Expected acknowledgment of file list");
            };

            request = session.read()?;
        }
        _ => {} // other requests handled later
    }

    let ClientToServer::FileRequest(requested_path) = request else {
        bail!("Expected file request");
    };
    session.transfer.filename = Some(requested_path.clone());

    if parameter.verbose_yn {
        println!("Request for file: '{}'", requested_path.display());
    };

    match std::fs::File::open(&requested_path) {
        Ok(opened_file) => session.transfer.file = Some(opened_file),
        Err(err) => {
            session.write(ServerToClient::FileResponseOne(Err(
                FileRequestError::Nonexistent,
            )))?;
            bail!(
                "File '{}' does not exist or cannot be read: {}",
                requested_path.display(),
                err
            );
        }
    };

    let ping_s = Instant::now();

    session.write(ServerToClient::FileResponseOne(Ok(())))?;

    let ClientToServer::BlockSize(block_size) = session.read()? else {
        bail!("Expected block size");
    };
    session.properties.block_size = block_size;

    let ClientToServer::TargetRate(target_rate) = session.read()? else {
        bail!("Expected target rate");
    };
    session.properties.target_rate = target_rate;

    let ClientToServer::ErrorRate(error_rate) = session.read()? else {
        bail!("Expected error rate");
    };
    session.properties.error_rate = error_rate;

    let ping_e = Instant::now();

    let ClientToServer::Slowdown(slower) = session.read()? else {
        bail!("Expected slowdown");
    };
    session.properties.slower = slower;

    let ClientToServer::Speedup(faster) = session.read()? else {
        bail!("Expected speedup");
    };
    session.properties.faster = faster;

    let file = session.transfer.file.as_mut().unwrap();
    session.properties.file_size = FileSize(file.seek(SeekFrom::End(0))?);
    file.seek(SeekFrom::Start(0))?;

    let mut block_count_base =
        session.properties.file_size.0 / session.properties.block_size.0 as u64;
    let tail_size = session.properties.file_size.0 % session.properties.block_size.0 as u64;

    if tail_size != 0 {
        block_count_base += 1;
    }

    session.properties.block_count =
        BlockIndex(block_count_base.try_into().expect("block count overflow"));
    session.properties.epoch = crate::common::epoch();

    session.write(ServerToClient::FileSize(session.properties.file_size))?;
    session.write(ServerToClient::BlockSize(session.properties.block_size))?;
    session.write(ServerToClient::BlockCount(session.properties.block_count))?;
    session.write(ServerToClient::Epoch(session.properties.epoch))?;

    session.properties.wait_u_sec = (ping_e - ping_s).as_micros().try_into().unwrap();
    session.properties.wait_u_sec += (session.properties.wait_u_sec as f64 * 0.1f64) as i64;

    session.properties.ipd_time = (1000000 * 8 * session.properties.block_size.0 as i64
        / session.properties.target_rate.0 as i64) as u32;
    session.transfer.ipd_current = (session.properties.ipd_time * 3) as f64;

    if parameter.transcript_yn {
        crate::common::transcript_warn_error(super::transcript::xscript_open_server(
            session, parameter,
        ));
    }
    Ok(())
}
