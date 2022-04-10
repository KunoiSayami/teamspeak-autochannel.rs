pub trait FromQueryString: for<'de> Deserialize<'de> {
    fn from_query(data: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        serde_teamspeak_querystring::from_str(data)
            .map_err(|e| anyhow::anyhow!("Got parser error: {:?}", e))
    }
}

pub trait FromJSON: for<'de> Deserialize<'de> {
    fn from_json(data: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        serde_json::from_str(data).map_err(|e| anyhow::anyhow!("Got parser error: {:?}", e))
    }

    fn from_value(value: Value) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        serde_json::from_value(value).map_err(|e| anyhow::anyhow!("Got parser error: {:?}", e))
    }
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

pub mod whoami {
    use super::{from_str, FromJSON, FromQueryString};
    use serde_derive::Deserialize;

    #[derive(Clone, Debug, Default, Deserialize)]
    pub struct WhoAmI {
        #[serde(deserialize_with = "from_str")]
        client_id: i64,
        #[serde(deserialize_with = "from_str")]
        client_database_id: i64,
    }

    impl WhoAmI {
        pub fn cldbid(&self) -> i64 {
            self.client_database_id
        }
        pub fn clid(&self) -> i64 {
            self.client_id
        }
    }

    impl FromQueryString for WhoAmI {}
    impl FromJSON for WhoAmI {}
}

pub mod create_channel {
    use super::{from_str, FromJSON, FromQueryString};
    use serde_derive::Deserialize;

    #[derive(Clone, Debug, Default, Deserialize)]
    pub struct CreateChannel {
        #[serde(deserialize_with = "from_str")]
        cid: i64,
    }

    impl CreateChannel {
        pub fn cid(&self) -> i64 {
            self.cid
        }
    }

    impl FromQueryString for CreateChannel {}
    impl FromJSON for CreateChannel {}
}

pub mod channel {
    use super::{from_str, FromJSON, FromQueryString};
    use serde_derive::Deserialize;

    #[allow(dead_code)]
    #[derive(Clone, Debug, Default, Deserialize)]
    pub struct Channel {
        #[serde(deserialize_with = "from_str")]
        cid: i64,
        #[serde(deserialize_with = "from_str")]
        pid: i64,
        #[serde(deserialize_with = "from_str")]
        channel_order: i64,
        channel_name: String,
        #[serde(deserialize_with = "from_str")]
        total_clients: i64,
        #[serde(deserialize_with = "from_str")]
        channel_needed_subscribe_power: i64,
    }

    #[allow(dead_code)]
    impl Channel {
        pub fn cid(&self) -> i64 {
            self.cid
        }
        pub fn pid(&self) -> i64 {
            self.pid
        }
        pub fn channel_order(&self) -> i64 {
            self.channel_order
        }
        pub fn channel_name(&self) -> &str {
            &self.channel_name
        }
        pub fn total_clients(&self) -> i64 {
            self.total_clients
        }
        pub fn channel_needed_subscribe_power(&self) -> i64 {
            self.channel_needed_subscribe_power
        }
    }

    impl FromQueryString for Channel {}
    impl FromJSON for Channel {}
}

pub mod client {
    use super::from_str;
    use super::{FromJSON, FromQueryString};
    use serde_derive::Deserialize;

    #[allow(dead_code)]
    #[derive(Clone, Debug, Default, Deserialize)]
    pub struct Client {
        #[serde(deserialize_with = "from_str")]
        clid: i64,
        #[serde(deserialize_with = "from_str")]
        cid: i64,
        #[serde(deserialize_with = "from_str")]
        client_database_id: i64,
        #[serde(deserialize_with = "from_str")]
        client_type: i64,
        //client_unique_identifier: String,
        client_nickname: String,
    }

    #[allow(dead_code)]
    impl Client {
        pub fn client_id(&self) -> i64 {
            self.clid
        }
        pub fn channel_id(&self) -> i64 {
            self.cid
        }
        pub fn client_database_id(&self) -> i64 {
            self.client_database_id
        }
        pub fn client_type(&self) -> i64 {
            self.client_type
        }
        pub fn client_unique_identifier(&self) -> String {
            format!("{}", self.client_database_id)
        }
        pub fn client_nickname(&self) -> &str {
            &self.client_nickname
        }
    }

    impl FromQueryString for Client {}
    impl FromJSON for Client {}

    #[cfg(test)]
    mod test {
        use crate::datastructures::client::Client;
        use crate::datastructures::FromQueryString;

        const TEST_STRING: &str = "clid=8 cid=1 client_database_id=1 client_nickname=serveradmin client_type=1 client_unique_identifier=serveradmin";

        #[test]
        fn test() {
            let result = Client::from_query(TEST_STRING).unwrap();
            assert_eq!(result.client_id(), 8);
            assert_eq!(result.channel_id(), 1);
            assert_eq!(result.client_database_id(), 1);
            assert_eq!(result.client_nickname(), "serveradmin".to_string());
            assert_eq!(result.client_type(), 1);
            assert_eq!(result.client_unique_identifier(), "serveradmin".to_string());
        }
    }
}

pub mod query_status {
    use anyhow::anyhow;
    use serde_derive::Deserialize;

    #[derive(Clone, Debug, Deserialize)]
    pub struct WebQueryStatus {
        code: i32,
        message: String,
    }

    impl WebQueryStatus {
        pub fn to_status(self) -> QueryStatus {
            QueryStatus {
                id: self.code,
                msg: self.message,
            }
        }
    }

    impl From<WebQueryStatus> for QueryStatus {
        fn from(status: WebQueryStatus) -> Self {
            status.to_status()
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug, Deserialize)]
    pub struct QueryStatus {
        id: i32,
        msg: String,
    }

    impl QueryStatus {
        pub fn id(&self) -> i32 {
            self.id
        }
        pub fn _msg(&self) -> &str {
            &self.msg
        }

        pub fn is_ok(&self) -> bool {
            self.id == 0
        }

        pub fn is_err(&self) -> bool {
            self.id != 0
        }
    }

    impl TryFrom<&str> for QueryStatus {
        type Error = anyhow::Error;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let (_, line) = value
                .split_once("error ")
                .ok_or_else(|| anyhow!("Split error: {}", value))?;
            serde_teamspeak_querystring::from_str(line)
                .map_err(|e| anyhow!("Got error while parse string: {:?} {:?}", line, e))
        }
    }
}

pub mod config {
    use anyhow::anyhow;
    use serde_derive::Deserialize;
    use std::fs::read_to_string;
    use std::path::Path;

    #[derive(Clone, Debug, Deserialize)]
    #[serde(untagged)]
    pub enum Channels {
        Single(i64),
        Multiple(Vec<i64>),
    }

    impl Channels {
        fn to_vec(&self) -> Vec<i64> {
            match self {
                Channels::Single(id) => {
                    vec![*id]
                }
                Channels::Multiple(ids) => ids.clone(),
            }
        }
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct WebQuery {
        server: Option<String>,
        api_key: String,
    }

    impl WebQuery {
        pub fn server(&self) -> String {
            if let Some(server) = &self.server {
                server.clone()
            } else {
                String::from("http://localhost:10080")
            }
        }
        pub fn api_key(&self) -> &str {
            &self.api_key
        }
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct RawQuery {
        server: Option<String>,
        port: Option<u16>,
        user: String,
        password: String,
    }

    impl RawQuery {
        pub fn server(&self) -> String {
            if let Some(server) = &self.server {
                server.clone()
            } else {
                String::from("127.0.0.1")
            }
        }
        pub fn port(&self) -> u16 {
            self.port.unwrap_or(10011)
        }
        pub fn user(&self) -> &str {
            &self.user
        }
        pub fn password(&self) -> &str {
            &self.password
        }
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Server {
        server_id: Option<i64>,
        channel_id: Channels,
        privilege_group_id: i64,
        redis_server: Option<String>,
    }

    impl Server {
        pub fn server_id(&self) -> i64 {
            self.server_id.unwrap_or(1)
        }
        #[deprecated(since = "0.7.0", note = "Use channels() instead of this function")]
        #[allow(dead_code)]
        pub fn channel_id(&self) -> Vec<i64> {
            self.channels()
        }
        pub fn channels(&self) -> Vec<i64> {
            self.channel_id.to_vec()
        }
        pub fn privilege_group_id(&self) -> i64 {
            self.privilege_group_id
        }
        pub fn redis_server(&self) -> String {
            if let Some(server) = &self.redis_server {
                server.clone()
            } else {
                String::from("redis://127.0.0.1")
            }
        }
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Misc {
        interval: Option<u64>,
    }

    impl Misc {
        pub fn interval(&self) -> u64 {
            self.interval.unwrap_or(5)
        }
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Config {
        server: Server,
        misc: Misc,
        raw_query: Option<RawQuery>,
        web_query: Option<WebQuery>,
    }

    impl Config {
        pub fn server(&self) -> &Server {
            &self.server
        }
        pub fn misc(&self) -> &Misc {
            &self.misc
        }
        pub fn raw_query(&self) -> &Option<RawQuery> {
            &self.raw_query
        }
        pub fn web_query(&self) -> &Option<WebQuery> {
            &self.web_query
        }
    }

    impl TryFrom<&Path> for Config {
        type Error = anyhow::Error;

        fn try_from(path: &Path) -> Result<Self, Self::Error> {
            let content = read_to_string(path).map_err(|e| anyhow!("Read error: {:?}", e))?;

            let result: Self =
                toml::from_str(&content).map_err(|e| anyhow!("Deserialize toml error: {:?}", e))?;

            if result.raw_query().is_none() && result.web_query().is_none() {
                Err(anyhow!("Both RawQuery and WebQuery are empty, exit."))
            } else {
                Ok(result)
            }
        }
    }
}

pub trait ApiMethods {
    fn who_am_i(&mut self) -> anyhow::Result<(QueryStatus, WhoAmI)>;
    fn send_text_message(&mut self, clid: i64, text: &str) -> anyhow::Result<QueryStatus>;

    fn query_channels(&mut self) -> anyhow::Result<(QueryStatus, Vec<Channel>)>;
    fn create_channel(
        &mut self,
        name: &str,
        pid: i64,
    ) -> anyhow::Result<(QueryStatus, Option<CreateChannel>)>;
    fn query_clients(&mut self) -> anyhow::Result<(QueryStatus, Vec<Client>)>;
    fn move_client_to_channel(
        &mut self,
        clid: i64,
        target_channel: i64,
    ) -> anyhow::Result<QueryStatus>;
    fn set_client_channel_group(
        &mut self,
        cldbid: i64,
        channel_id: i64,
        group_id: i64,
    ) -> anyhow::Result<QueryStatus>;
}

pub use channel::Channel;
pub use client::Client;
pub use config::Config;
pub use create_channel::CreateChannel;
pub use query_status::{QueryStatus, WebQueryStatus};
use serde::Deserialize;
use serde_json::Value;
pub use whoami::WhoAmI;
