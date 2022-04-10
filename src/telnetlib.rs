use crate::datastructures::{ApiMethods, Channel, Client, CreateChannel, WhoAmI};
use crate::datastructures::{FromQueryString, QueryStatus};
use anyhow::anyhow;
use log::{error, warn};
use std::time::Duration;
use telnet::Event;

pub struct TelnetConn {
    conn: telnet::Telnet,
}

impl ApiMethods for TelnetConn {
    fn who_am_i(&mut self) -> anyhow::Result<(QueryStatus, WhoAmI)> {
        let (status, mut ret) = self.query_operation_non_error("whoami\n\r")?;
        Ok((status, ret.remove(0)))
    }

    fn send_text_message(&mut self, clid: i64, text: &str) -> anyhow::Result<QueryStatus> {
        let payload = format!(
            "sendtextmessage targetmode=1 target={clid} msg={text}\n\r",
            clid = clid,
            text = Self::escape(text)
        );
        self.basic_operation(&payload)
    }

    #[allow(dead_code)]
    fn query_channels(&mut self) -> anyhow::Result<(QueryStatus, Vec<Channel>)> {
        self.query_operation_non_error("channellist\n\r")
    }

    fn create_channel(
        &mut self,
        name: &str,
        pid: i64,
    ) -> anyhow::Result<(QueryStatus, Option<CreateChannel>)> {
        let payload = format!(
            "channelcreate channel_name={name} cpid={pid} channel_codec_quality=6\n\r",
            name = Self::escape(name),
            pid = pid
        );
        let (status, ret) = self.query_operation(payload.as_str())?;
        Ok((status, ret.map(|mut v| v.remove(0))))
    }

    fn query_clients(&mut self) -> anyhow::Result<(QueryStatus, Vec<Client>)> {
        self.query_operation_non_error("clientlist -uid\n\r")
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

    fn set_client_channel_group(
        &mut self,
        cldbid: i64,
        channel_id: i64,
        group_id: i64,
    ) -> anyhow::Result<QueryStatus> {
        let payload = format!(
            "setclientchannelgroup cgid={group} cid={channel_id} cldbid={cldbid}\n\r",
            group = group_id,
            channel_id = channel_id,
            cldbid = cldbid
        );
        self.basic_operation(&payload)
    }
}

impl TelnetConn {
    fn decode_status(data: &[u8]) -> anyhow::Result<(Option<QueryStatus>, String)> {
        let content =
            String::from_utf8(data.to_vec()).map_err(|e| anyhow!("Got FromUtf8Error: {:?}", e))?;

        debug_assert!(content.contains("error id="));

        for line in content.lines() {
            if line.trim().starts_with("error ") {
                let status = QueryStatus::try_from(line)?;

                return Ok((Some(status), content));
            }
        }
        Ok((None, content))
    }

    fn decode_status_with_result<T: FromQueryString + Sized>(
        data: Box<[u8]>,
    ) -> anyhow::Result<(Option<QueryStatus>, Option<Vec<T>>)> {
        let (status, content) = Self::decode_status(&data)?;

        if let Some(ref q_status) = status {
            if !q_status.is_ok() {
                return Ok((status, None));
            }
        }

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

    fn read_data(&mut self, timeout: u64) -> anyhow::Result<Option<Box<[u8]>>> {
        match self
            .conn
            .read_timeout(Duration::from_secs(timeout))
            .map_err(|e| anyhow!("Got error while read data: {:?}", e))?
        {
            Event::Data(data) => Ok(Some(data)),
            Event::TimedOut => Ok(None),
            Event::NoData => Ok(None),
            Event::Error(e) => Err(anyhow!("Got error on read data: {:?}", e)),
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
        Self::decode_status(&data)?
            .0
            .ok_or_else(|| anyhow!("Can't find status line."))
    }

    fn query_operation_non_error<T: FromQueryString + Sized>(
        &mut self,
        payload: &str,
    ) -> anyhow::Result<(QueryStatus, Vec<T>)> {
        let data = self.write_and_read(payload, 2)?;
        let (status, ret) = Self::decode_status_with_result(data)?;
        Ok((
            status.ok_or_else(|| anyhow!("Can't find status line."))?,
            ret.ok_or_else(|| anyhow!("Can't find result line."))?,
        ))
    }

    fn query_operation<T: FromQueryString + Sized>(
        &mut self,
        payload: &str,
    ) -> anyhow::Result<(QueryStatus, Option<Vec<T>>)> {
        let data = self.write_and_read(payload, 2)?;
        let (status, ret) = Self::decode_status_with_result(data)?;
        let status = status.ok_or_else(|| anyhow!("Can't find status line."))?;
        let ret = if status.is_ok() {
            Some(ret.ok_or_else(|| anyhow!("Can't find result line."))?)
        } else {
            ret
        };
        Ok((status, ret))
    }

    fn escape(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace(' ', "\\s")
            .replace('/', "\\/")
    }
    pub fn connect(server: &str, port: u16) -> anyhow::Result<Self> {
        let conn = telnet::Telnet::connect((server, port), 65536)
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

    pub fn login(&mut self, user: &str, password: &str) -> anyhow::Result<QueryStatus> {
        let payload = format!("login {} {}\n\r", user, password);
        self.basic_operation(payload.as_str())
    }

    pub fn select_server(&mut self, server_id: i64) -> anyhow::Result<QueryStatus> {
        let payload = format!("use {}\n\r", server_id);
        self.basic_operation(payload.as_str())
    }
}
