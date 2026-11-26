//! PolyTabletop line protocol, version `polytabletop/1` (see docs/RETEA.md).
//!
//! Every message is one UTF-8 line ending in `\n`, except `STATE`, whose header
//! announces how many text lines follow. Commands are case-insensitive.

use std::io::{self, BufRead, Write};

pub const VERSION: &str = "polytabletop/1";
/// Longest line accepted from a peer (bytes, without the newline).
pub const MAX_LINE: usize = 1024;
/// Longest STATE body a client accepts (lines).
pub const MAX_STATE_LINES: usize = 400;
pub const MAX_CHAT: usize = 200;

/// Messages sent by the client (the server never trusts them).
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ClientMsg {
    Hello(String),
    Move(String),
    Legal,
    State,
    Chat(String),
    Quit,
}

/// Messages sent by the server.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ServerMsg {
    Hello,
    Start {
        id: String,
        seed: u64,
        side: u8,
    },
    /// `turn`: 1 or 2 = side to move, 0 = game over.
    State {
        turn: u8,
        lines: Vec<String>,
    },
    Ok(String),
    Err(String),
    Legal(Vec<String>),
    Chat {
        side: u8,
        text: String,
    },
    /// `outcome`: win1 | win2 | draw | aborted.
    End {
        outcome: String,
        reason: String,
    },
}

/// Removes control characters (incl. CR/LF) so text can never break framing.
pub fn clean(s: &str) -> String {
    s.chars().filter(|c| !c.is_control()).collect()
}

fn split_word(s: &str) -> (&str, &str) {
    let s = s.trim_start();
    match s.find(char::is_whitespace) {
        Some(i) => (&s[..i], s[i..].trim()),
        None => (s, ""),
    }
}

pub fn parse_client(line: &str) -> Result<ClientMsg, String> {
    let (cmd, rest) = split_word(line);
    match cmd.to_ascii_lowercase().as_str() {
        "hello" => Ok(ClientMsg::Hello(rest.to_string())),
        "move" if !rest.is_empty() => Ok(ClientMsg::Move(clean(rest))),
        "move" => Err("MOVE fără text".into()),
        "legal" => Ok(ClientMsg::Legal),
        "state" => Ok(ClientMsg::State),
        "chat" if !rest.is_empty() => Ok(ClientMsg::Chat(
            clean(rest).chars().take(MAX_CHAT).collect(),
        )),
        "chat" => Err("chat gol".into()),
        "quit" | "bye" => Ok(ClientMsg::Quit),
        "" => Err("linie goală".into()),
        _ => Err("comandă necunoscută".into()),
    }
}

pub fn write_server<W: Write>(w: &mut W, m: &ServerMsg) -> io::Result<()> {
    let mut out = String::new();
    match m {
        ServerMsg::Hello => out.push_str(&format!("HELLO {VERSION}\n")),
        ServerMsg::Start { id, seed, side } => out.push_str(&format!("START {id} {seed} {side}\n")),
        ServerMsg::State { turn, lines } => {
            out.push_str(&format!("STATE {turn} {}\n", lines.len()));
            for l in lines {
                out.push_str(&l.replace(['\r', '\n'], " "));
                out.push('\n');
            }
        }
        ServerMsg::Ok(t) => out.push_str(&format!("OK {}\n", clean(t))),
        ServerMsg::Err(t) => out.push_str(&format!("ERR {}\n", clean(t))),
        ServerMsg::Legal(v) => {
            let v: Vec<String> = v.iter().map(|s| clean(s)).collect();
            out.push_str(&format!("LEGAL {}\n", v.join("\t")))
        }
        ServerMsg::Chat { side, text } => out.push_str(&format!("CHAT {side} {}\n", clean(text))),
        ServerMsg::End { outcome, reason } => {
            out.push_str(&format!("END {outcome} {}\n", clean(reason)))
        }
    }
    w.write_all(out.as_bytes())?;
    w.flush()
}

pub fn write_client<W: Write>(w: &mut W, line: &str) -> io::Result<()> {
    w.write_all(format!("{}\n", clean(line)).as_bytes())?;
    w.flush()
}

/// Outcome of reading one bounded line.
#[derive(Debug, PartialEq, Eq)]
pub enum Line {
    Text(String),
    TooLong,
    BadUtf8,
    Eof,
}

/// Reads one line of at most `max` bytes; the rest of an overlong line is discarded.
pub fn read_line_capped<R: BufRead>(r: &mut R, max: usize) -> io::Result<Line> {
    let mut buf: Vec<u8> = Vec::new();
    let mut too_long = false;
    loop {
        let (done, used) = {
            let avail = match r.fill_buf() {
                Ok(a) => a,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            if avail.is_empty() {
                if buf.is_empty() && !too_long {
                    return Ok(Line::Eof);
                }
                (true, 0)
            } else if let Some(i) = avail.iter().position(|&b| b == b'\n') {
                if !too_long {
                    buf.extend_from_slice(&avail[..i]);
                }
                (true, i + 1)
            } else {
                if !too_long {
                    buf.extend_from_slice(avail);
                }
                (false, avail.len())
            }
        };
        r.consume(used);
        if buf.len() > max {
            too_long = true;
            buf.clear();
        }
        if done {
            break;
        }
    }
    if too_long {
        return Ok(Line::TooLong);
    }
    if buf.last() == Some(&b'\r') {
        buf.pop();
    }
    match String::from_utf8(buf) {
        Ok(s) => Ok(Line::Text(s)),
        Err(_) => Ok(Line::BadUtf8),
    }
}

fn bad(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg.to_string())
}

/// Reads one server message (assembling STATE bodies). `Ok(None)` = clean EOF.
pub fn read_server<R: BufRead>(r: &mut R) -> io::Result<Option<ServerMsg>> {
    let line = match read_line_capped(r, 64 * 1024)? {
        Line::Eof => return Ok(None),
        Line::Text(t) => t,
        _ => return Err(bad("linie invalidă de la server")),
    };
    let (cmd, rest) = split_word(&line);
    let m = match cmd.to_ascii_uppercase().as_str() {
        "HELLO" => ServerMsg::Hello,
        "START" => {
            let p: Vec<&str> = rest.split_whitespace().collect();
            if p.len() != 3 {
                return Err(bad("START malformat"));
            }
            let side = match p[2] {
                "1" => 1,
                "2" => 2,
                _ => return Err(bad("START: latură")),
            };
            ServerMsg::Start {
                id: p[0].into(),
                seed: p[1].parse().map_err(|_| bad("START: seed"))?,
                side,
            }
        }
        "STATE" => {
            let p: Vec<&str> = rest.split_whitespace().collect();
            if p.len() != 2 {
                return Err(bad("STATE malformat"));
            }
            let turn: u8 = p[0].parse().map_err(|_| bad("STATE: tură"))?;
            let n: usize = p[1].parse().map_err(|_| bad("STATE: n"))?;
            if turn > 2 || n > MAX_STATE_LINES {
                return Err(bad("STATE: valori"));
            }
            let mut lines = Vec::with_capacity(n);
            for _ in 0..n {
                match read_line_capped(r, 64 * 1024)? {
                    Line::Text(t) => lines.push(t),
                    _ => return Err(bad("STATE trunchiat")),
                }
            }
            ServerMsg::State { turn, lines }
        }
        "OK" => ServerMsg::Ok(rest.into()),
        "ERR" => ServerMsg::Err(rest.into()),
        "LEGAL" => ServerMsg::Legal(
            rest.split('\t')
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect(),
        ),
        "CHAT" => {
            let (s, t) = split_word(rest);
            ServerMsg::Chat {
                side: s.parse().map_err(|_| bad("CHAT: latură"))?,
                text: t.into(),
            }
        }
        "END" => {
            let (o, why) = split_word(rest);
            ServerMsg::End {
                outcome: o.into(),
                reason: why.into(),
            }
        }
        _ => return Err(bad("mesaj necunoscut de la server")),
    };
    Ok(Some(m))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrip_state() {
        let m = ServerMsg::State {
            turn: 2,
            lines: vec!["a".into(), "".into(), "c d".into()],
        };
        let mut v = Vec::new();
        write_server(&mut v, &m).unwrap();
        assert_eq!(read_server(&mut Cursor::new(v)).unwrap(), Some(m));
    }

    #[test]
    fn capped_lines() {
        let mut c = Cursor::new(b"abc\r\nxxxxxxxxxxxx\nok\n\xff\xfe\n".to_vec());
        assert_eq!(
            read_line_capped(&mut c, 5).unwrap(),
            Line::Text("abc".into())
        );
        assert_eq!(read_line_capped(&mut c, 5).unwrap(), Line::TooLong);
        assert_eq!(
            read_line_capped(&mut c, 5).unwrap(),
            Line::Text("ok".into())
        );
        assert_eq!(read_line_capped(&mut c, 5).unwrap(), Line::BadUtf8);
        assert_eq!(read_line_capped(&mut c, 5).unwrap(), Line::Eof);
    }

    #[test]
    fn parse_client_cases() {
        assert_eq!(
            parse_client("move e2e4"),
            Ok(ClientMsg::Move("e2e4".into()))
        );
        assert_eq!(
            parse_client("CHAT hi  there"),
            Ok(ClientMsg::Chat("hi  there".into()))
        );
        assert!(parse_client("move").is_err());
        assert!(parse_client("zzz").is_err());
        assert!(parse_client("").is_err());
    }
}
