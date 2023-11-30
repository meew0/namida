# namida

namida is a tool for fast file downloads over high-latency and/or unreliable networks. It uses UDP for bulk data transmission, together with a minimal TCP control stream to mediate retransmission of lost data.

namida is based upon [Tsunami](https://tsunami-udp.sourceforge.net), a 2000s-era protocol and software suite for UDP-based file transmission. While Tsunami is still usable today, it has essentially not been updated since 2009, and has several problems that make it annoying to use nowadays. So, I created namida by first converting Tsunami's source code to Rust using [C2Rust](https://github.com/immunant/c2rust), manually converting the generated unsafe code to safe, more idiomatic Rust, and then making various improvements.

In the process I also removed some parts of Tsunami. In particular, after 2006 Tsunami was primarily maintained by Finnish [VLBI](https://en.wikipedia.org/wiki/Very-long-baseline_interferometry) scientists (primarily Jan Wagner at Metsähovi Radio Observatory), who added support for VLBI-specific real-time networking hardware. I do not have access to any of this hardware, so I would not be able to port these parts even if I wanted to, and presumably the VLBI people are either still happily using Tsunami or have their own updated tools anyway. (If you know which one it is, let me know, I am curious!)
