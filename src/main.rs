mod datastructures;

use crate::datastructures::channel::Channel;
use crate::datastructures::client::Client;
use crate::datastructures::{FromQueryString, QueryStatus};
use anyhow::anyhow;
use clap::{arg, Command};
use log::{error, warn};
use serde::Deserialize;
use std::time::Duration;
use telnet::Event;

struct TelnetConn {
    conn: telnet::Telnet,
}

impl TelnetConn {
    fn decode_status(data: Box<[u8]>) -> anyhow::Result<(Option<QueryStatus>, String)> {
        let content =
            String::from_utf8(data.to_vec()).map_err(|e| anyhow!("Got FromUtf8Error: {:?}", e))?;

        debug_assert!(content.contains("error id="));

        for line in content.lines() {
            if line.trim().starts_with("error ") {
                let status = QueryStatus::try_from(line)?;
                if !status.is_ok() {
                    return Err(anyhow!(
                        "Got non ok status: id={} msg={}",
                        status.id(),
                        status.msg()
                    ));
                }

                return Ok((Some(status), content));
            }
        }
        Ok((None, content))
    }

    fn decode_status_with_result<'de, T: FromQueryString<'de> + Sized>(
        data: Box<[u8]>,
    ) -> anyhow::Result<(Option<QueryStatus>, Option<Vec<T>>)> {
        let (status, content) = Self::decode_status(data)?;

        for line in content.lines() {
            if !line.starts_with("error ") {
                let mut v = Vec::new();
                for element in line.split('|') {
                    v.push(T::from_query(element)?);
                }
                return Ok((status, Some(v)));
            }
        }
        Ok((status, None))
    }

    fn connect(server: &str, port: u16) -> anyhow::Result<Self> {
        let conn = telnet::Telnet::connect((server, port), 512)
            .map_err(|e| anyhow!("Got error while connect to {}:{} {:?}", server, port, e))?;
        let mut self_ = Self { conn };

        let content = self_
            .read_data(1)
            .map_err(|e| anyhow!("Got error while read content: {:?}", e))?;

        if content.is_none() {
            warn!("Read none");
        }

        Ok(self_)
    }

    fn read_data(&mut self, timeout: u64) -> anyhow::Result<Option<Box<[u8]>>> {
        match self
            .conn
            .read_timeout(Duration::from_secs(timeout))
            .map_err(|e| anyhow!("Got error while read data: {:?}", e))?
        {
            Event::Data(data) => Ok(Some(data)),
            Event::TimedOut => Ok(None),
            Event::NoData => Ok(None),
            Event::Error(e) => Err(anyhow!("Got error: {:?}", e)),
            _ => Err(anyhow!("Got unknown error")),
        }
    }

    fn write_data(&mut self, payload: &str) -> anyhow::Result<()> {
        self.conn
            .write(payload.as_bytes())
            .map(|size| {
                if size != payload.as_bytes().len() {
                    error!("Error")
                }
            })
            .map_err(|e| anyhow!("Got error while send data: {:?}", e))?;
        Ok(())
    }

    fn write_and_read(&mut self, payload: &str, timeout: u64) -> anyhow::Result<Box<[u8]>> {
        self.write_data(payload)?;
        self
            .read_data(timeout)?
            .ok_or_else(|| anyhow!("Return data is None"))
    }

    fn login(&mut self, user: &str, password: &str) -> anyhow::Result<QueryStatus> {
        let payload = format!("login {} {}\n\r", user, password);
        let data = self.write_and_read(payload.as_str(), 2)?;
        Self::decode_status(data)?
            .0
            .ok_or_else(|| anyhow!("Can't find status line."))
    }

    fn select_server(&mut self, server_id: i32) -> anyhow::Result<QueryStatus> {
        let payload = format!("use {}\n\r", server_id);
        let data = self.write_and_read(payload.as_str(), 2)?;
        Self::decode_status(data)?
            .0
            .ok_or_else(|| anyhow!("Can't find status line."))
    }

    fn query_clients(&mut self) -> anyhow::Result<(QueryStatus, Vec<Client>)> {
        let data = self.write_and_read("clientlist -uid\n\r", 2)?;
        let (status, clients) = Self::decode_status_with_result(data)?;

        Ok((
            status.ok_or_else(|| anyhow!("Can't find status line."))?,
            clients.ok_or_else(|| anyhow!("Can't find result line."))?,
        ))
    }

    fn query_channels(&mut self) -> anyhow::Result<(QueryStatus, Vec<Channel>)> {
        let data = self.write_and_read("channellist\n\r", 2)?;
        let (status, channels) = Self::decode_status_with_result(data)?;

        Ok((
            status.ok_or_else(|| anyhow!("Can't find status line."))?,
            channels.ok_or_else(|| anyhow!("Can't find result line."))?,
        ))
    }
}

fn staff(server: &str, port: u16, user: &str, password: &str, sid: &str) -> anyhow::Result<()> {
    let mut conn = TelnetConn::connect(server, port)?;
    let status = conn.login(user, password)?;
    if !status.is_ok() {
        return Err(anyhow!("Login failed. {:?}", status));
    }
    let status = conn.select_server(
        sid.parse()
            .map_err(|e| anyhow!("Got error while parse sid: {:?}", e))?,
    )?;
    if !status.is_ok() {
        return Err(anyhow!("Select server id failed: {:?}", status));
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .args(&[
            arg!(--server [SERVER] "Teamspeak ServerQuery server address"),
            arg!(--port [PORT] "Teamspeak ServerQuery server port"),
            arg!(<USER> "Teamspeak ServerQuery user"),
            arg!(<PASSWORD> "Teamspeak ServerQuery password"),
            arg!(--sid [SID] "Teamspeak ServerQuery server id"),
        ])
        .get_matches();
    env_logger::Builder::from_default_env().init();
    staff(
        matches.value_of("SERVER").unwrap_or("localhost"),
        matches
            .value_of("PORT")
            .unwrap_or("10011")
            .parse()
            .unwrap_or_else(|e| {
                warn!("Got parse error: {:?}", e);
                10011
            }),
        matches.value_of("USER").unwrap(),
        matches.value_of("PASSWORD").unwrap(),
        matches.value_of("SID").unwrap_or("1"),
    )?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_connection() {
        let mut conn = TelnetConn::connect(env!("QUERY_HOST"), 10011).unwrap();

        let result = conn.login("serveradmin", env!("QUERY_PASSWORD")).unwrap();

        assert!(result.is_ok());

        let result = conn.select_server(1).unwrap();
        assert!(result.is_ok());

        let (status, clients) = conn.query_clients().unwrap();
        assert!(status.is_ok());
        dbg!(clients);

        let (status, channel) = conn.query_channels().unwrap();
        assert!(status.is_ok());
        dbg!(channel);
    }
}
