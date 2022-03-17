use anyhow::anyhow;
use clap::{arg, Command};
use log::{error, warn};
use std::borrow::Borrow;
use std::time::Duration;
use telnet::Event;

struct TelnetConn {
    conn: telnet::Telnet,
}

impl TelnetConn {
    fn new(server: &str, port: u16) -> anyhow::Result<Self> {
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
}

async fn staff(server: &str, port: u16, user: &str, password: &str) -> anyhow::Result<()> {
    let mut conn = TelnetConn::new(server, port)?;
    Ok(())
}

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .args(&[
            arg!(--server [SERVER] "Teamspeak ServerQuery server address"),
            arg!(--port [PORT] "Teamspeak ServerQuery server port"),
            arg!(<USER> "Teamspeak ServerQuery user"),
            arg!(<PASSWORD> "Teamspeak ServerQuery password"),
        ])
        .get_matches();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(staff(
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
        ));
}
