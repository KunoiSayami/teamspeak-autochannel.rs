mod datastructures;

use crate::datastructures::{Channel, Client, CreateChannel, WhoAmI};
use crate::datastructures::{FromQueryString, QueryStatus};
use anyhow::anyhow;
use clap::{arg, Command};
use log::{error, info, warn};
use std::collections::HashMap;
use std::time::Duration;
use telnet::Event;

struct TelnetConn {
    conn: telnet::Telnet,
    pid: i64,
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

    fn decode_status_with_result<T: FromQueryString + Sized>(
        data: Box<[u8]>,
    ) -> anyhow::Result<(Option<QueryStatus>, Option<Vec<T>>)> {
        let (status, content) = Self::decode_status(data)?;

        println!("{}", content);

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

    fn connect(server: &str, port: u16, target_id: i64) -> anyhow::Result<Self> {
        let conn = telnet::Telnet::connect((server, port), 65536)
            .map_err(|e| anyhow!("Got error while connect to {}:{} {:?}", server, port, e))?;
        let mut self_ = Self {
            conn,
            pid: target_id,
        };

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
        debug_assert!(payload.ends_with("\n\r"));
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
        self.read_data(timeout)?
            .ok_or_else(|| anyhow!("Return data is None"))
    }

    fn basic_operation(&mut self, payload: &str) -> anyhow::Result<QueryStatus> {
        let data = self.write_and_read(payload, 2)?;
        Self::decode_status(data)?
            .0
            .ok_or_else(|| anyhow!("Can't find status line."))
    }

    fn query_option<T: FromQueryString + Sized>(
        &mut self,
        payload: &str,
    ) -> anyhow::Result<(QueryStatus, Vec<T>)> {
        let data = self.write_and_read(payload, 2)?;
        let (status, clients) = Self::decode_status_with_result(data)?;
        Ok((
            status.ok_or_else(|| anyhow!("Can't find status line."))?,
            clients.ok_or_else(|| anyhow!("Can't find result line."))?,
        ))
    }

    fn login(&mut self, user: &str, password: &str) -> anyhow::Result<QueryStatus> {
        let payload = format!("login {} {}\n\r", user, password);
        self.basic_operation(payload.as_str())
    }

    fn select_server(&mut self, server_id: i32) -> anyhow::Result<QueryStatus> {
        let payload = format!("use {}\n\r", server_id);
        self.basic_operation(payload.as_str())
    }

    fn query_clients(&mut self) -> anyhow::Result<(QueryStatus, Vec<Client>)> {
        self.query_option("clientlist -uid\n\r")
    }

    fn query_channels(&mut self) -> anyhow::Result<(QueryStatus, Vec<Channel>)> {
        self.query_option("channellist\n\r")
    }

    fn set_client_channel_group(
        &mut self,
        clid: i64,
        group_id: i64,
    ) -> anyhow::Result<QueryStatus> {
        todo!()
    }

    fn who_am_i(&mut self) -> anyhow::Result<(QueryStatus, WhoAmI)> {
        let (status, mut ret) = self.query_option("whoami\n\r")?;
        Ok((status, ret.remove(0)))
    }

    fn create_channel(&mut self, name: &str) -> anyhow::Result<(QueryStatus, CreateChannel)> {
        let payload = format!(
            "channelcreate channel_name={name} cpid={pid} channel_codec_quality=6\n\r",
            name = name
                .replace(' ', "\\s")
                .replace('\\', "\\\\")
                .replace('/', "\\/"),
            pid = self.pid
        );
        let (status, mut ret) = self.query_option(payload.as_str())?;
        Ok((status, ret.remove(0)))
    }

    fn move_client_to_channel(
        &mut self,
        clid: i64,
        target_channel: i64,
    ) -> anyhow::Result<QueryStatus> {
        let payload = format!(
            "clientmove clid={clid} cid={cid}\n\r",
            clid = clid,
            cid = target_channel
        );
        self.basic_operation(payload.as_str())
    }
}

fn staff(
    server: &str,
    port: u16,
    user: &str,
    password: &str,
    sid: &str,
    channel_id: &str,
) -> anyhow::Result<()> {
    let channel_id = channel_id
        .parse()
        .map_err(|e| anyhow!("Got parse error while parse channel_id: {:?}", e))?;
    let mut conn = TelnetConn::connect(server, port, channel_id)?;
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

    let (status, who_am_i) = conn.who_am_i()?;

    if !status.is_ok() {
        return Err(anyhow!("Whoami failed: {:?}", status));
    }

    let clid = who_am_i.clid();

    let mut mapper: HashMap<i64, i64> = HashMap::new();

    loop {
        std::thread::sleep(Duration::from_millis(1));
        let (status, clients) = conn.query_clients()?;

        if !status.is_ok() {
            error!("Got error while query clients: {:?}", status);
            continue;
        }

        for client in clients {
            if client.channel_id() != channel_id || client.client_id() == clid {
                continue;
            }

            let ret = mapper.get(&client.client_database_id());
            let create_new = ret.is_none();
            let target_channel = if create_new {
                let name = format!("{}'s channel", client.client_nickname());
                let (status, create_channel) = match conn.create_channel(&name) {
                    Ok(ret) => ret,
                    Err(e) => {
                        error!("Got error while create channel: {:?}", e);
                        continue;
                    }
                };
                // error id=771

                if !status.is_ok() {
                    error!("Got error while create {:?} channel: {:?}", name, status);
                    continue;
                }
                create_channel.cid()
            } else {
                *ret.unwrap()
            };

            let status = match conn.move_client_to_channel(client.client_id(), target_channel) {
                Ok(ret) => ret,
                Err(e) => {
                    error!("Got error while move client: {:?}", e);
                    continue;
                }
            };

            if create_new {
                conn.move_client_to_channel(clid, channel_id).unwrap();
                mapper.insert(client.client_database_id(), target_channel);
            }

            info!("Move {} to {}", client.client_nickname(), target_channel);
        }
    }
}

async fn handler() {}

fn main() -> anyhow::Result<()> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .args(&[
            arg!(--server [SERVER] "Teamspeak ServerQuery server address"),
            arg!(--port [PORT] "Teamspeak ServerQuery server port"),
            arg!(<USER> "Teamspeak ServerQuery user"),
            arg!(<PASSWORD> "Teamspeak ServerQuery password"),
            arg!(--sid [SID] "Teamspeak ServerQuery server id"),
            arg!(<CHANNEL_ID> "Teamspeak server target channel id"),
            arg!(<PRIVILEGE_GROUP> "Target channel privilege group id"),
        ])
        .get_matches();
    env_logger::Builder::from_default_env().init();
    staff(
        matches.value_of("server").unwrap_or("localhost"),
        matches
            .value_of("port")
            .unwrap_or("10011")
            .parse()
            .unwrap_or_else(|e| {
                warn!("Got parse error: {:?}", e);
                10011
            }),
        matches.value_of("USER").unwrap(),
        matches.value_of("PASSWORD").unwrap(),
        matches.value_of("sid").unwrap_or("1"),
        matches.value_of("CHANNEL_ID").unwrap(),
    )?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_current_timestamp() -> u64 {
        let start = std::time::SystemTime::now();
        let since_the_epoch = start
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs()
    }

    #[test]
    fn test_connection() {
        let mut conn = TelnetConn::connect(env!("QUERY_HOST"), 10011, 5).unwrap();

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

        let name = format!("test{}", get_current_timestamp());
        let (status, create_channel) = conn.create_channel(name.as_str()).unwrap();
        assert!(status.is_ok());
        dbg!(create_channel);
    }
}
