# namida

<sup><sup><sup>[waves become tears](https://evergreen2.bandcamp.com/album/-)</sup></sup></sup>

namida is a tool for fast file downloads over high-latency and/or unreliable networks. It uses UDP for bulk data transmission, together with a minimal TCP control stream to mediate retransmission of lost data.

namida is based upon [Tsunami](https://tsunami-udp.sourceforge.net), a 2000s-era protocol and software suite for UDP-based file transmission. While Tsunami is still usable today, it has essentially not been updated since 2009, and has several problems that make it annoying to use nowadays. So, I created namida by first converting Tsunami's source code to Rust using [C2Rust](https://github.com/immunant/c2rust), manually converting the generated unsafe code to safe, more idiomatic Rust, and then making various improvements.

In the process I also removed some parts of Tsunami. In particular, after 2006 Tsunami was primarily maintained by Finnish [VLBI](https://en.wikipedia.org/wiki/Very-long-baseline_interferometry) scientists (primarily Jan Wagner at Metsähovi Radio Observatory), who added support for VLBI-specific real-time networking hardware. I do not have access to any of this hardware, so I would not be able to port these parts even if I wanted to, and presumably the VLBI people are either still happily using Tsunami or have their own updated tools anyway. (If you know which one it is, let me know, I am curious!)

# Features

- Dynamic UDP transfer rate adjustment to avoid overloading the client (it may still use all the available bandwidth, leaving none for other applications. It is highly recommended to use the `--rate` command to limit the transfer rate to a suitable value)
- Optional lossy transfer mode: if some amount of data loss can be tolerated, namida can be configured to allow packets to be dropped, with an optional limit. By default, transfers are always lossless.

New features compared to Tsunami:

- Simple CLI that allows everything to be done in one command invocation (in return, Tsunami's FTP-like interactive console has been removed)
- Client-side NAT traversal: UDP packets can be received even if the client is behind NAT, without any additional manual configuration required.
- Encrypted communication by default: [snow](https://github.com/mcginty/snow) is used to encrypt both TCP and UDP communication.
- Resumption of interrupted transfers: if parts of a file to be downloaded are already present locally, those parts will be skipped by default.

While namida is based on software that has been used in production for 20 years, there are still many parts I'm unhappy with. Also, my “improvements” might have introduced new bugs. Expect more updates in the future.

# Usage

For now, clone the repo and build it using `cargo build --release`. The same executable is used for the client and the server.

Run a namida server providing all files in the local directory:

```
$ namida serve
```

Run a namida server providing only some specific files:

```
$ namida serve file1.txt file2.txt
```

List the files a server has available:

```
$ namida dir --server example.com
```

Get a specific file from a server:

```
$ namida get --server example.com file1.txt
```

Get all files from a server:

```
$ namida get --server example.com --all
```

Many more options are available for the individual subcommands. Run `namida help [command]` to get more information.

# Licencing information

namida is available under the same licence as Tsunami (both the original Tsunami from Indiana University, and Jan Wagner's updated version), which is a permissive BSD-style licence with the additional restriction that derivative programs may not be called “Tsunami” without permission from Indiana University. See `LICENSE.txt` for the full licence text.
