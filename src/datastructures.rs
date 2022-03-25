pub trait FromQueryString: for<'de> Deserialize<'de> {
    fn from_query(data: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        serde_teamspeak_querystring::from_str(data)
            .map_err(|e| anyhow::anyhow!("Got parser error: {:?}", e))
    }
}

pub mod whoami {
    use crate::FromQueryString;
    use serde_derive::Deserialize;

    #[derive(Clone, Debug, Default, Deserialize)]
    pub struct WhoAmI {
        client_id: i64,
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
}

pub mod create_channel {
    use crate::FromQueryString;
    use serde_derive::Deserialize;

    #[derive(Clone, Debug, Default, Deserialize)]
    pub struct CreateChannel {
        cid: i64,
    }

    impl CreateChannel {
        pub fn cid(&self) -> i64 {
            self.cid
        }
    }

    impl FromQueryString for CreateChannel {}
}

pub mod channel {
    use crate::FromQueryString;
    use serde_derive::Deserialize;

    #[allow(dead_code)]
    #[derive(Clone, Debug, Default, Deserialize)]
    pub struct Channel {
        cid: i64,
        pid: i64,
        channel_order: i64,
        channel_name: String,
        total_clients: i64,
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
}

pub mod client {
    use crate::datastructures::FromQueryString;
    use serde_derive::Deserialize;

    #[allow(dead_code)]
    #[derive(Clone, Debug, Default, Deserialize)]
    pub struct Client {
        clid: i64,
        cid: i64,
        client_database_id: i64,
        client_type: i64,
        client_unique_identifier: String,
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
        pub fn client_unique_identifier(&self) -> &str {
            &self.client_unique_identifier
        }
        pub fn client_nickname(&self) -> &str {
            &self.client_nickname
        }
    }

    impl FromQueryString for Client {}

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
    pub struct Server {
        server: Option<String>,
        port: Option<u16>,
        user: String,
        password: String,
        server_id: Option<i64>,
        channel_id: i64,
        privilege_group_id: i64,
        redis_server: Option<String>,
    }

    impl Server {
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
        pub fn server_id(&self) -> i64 {
            self.server_id.unwrap_or(1)
        }
        pub fn channel_id(&self) -> i64 {
            self.channel_id
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
    }

    impl Config {
        pub fn server(&self) -> &Server {
            &self.server
        }
        pub fn misc(&self) -> &Misc {
            &self.misc
        }
    }

    impl TryFrom<&Path> for Config {
        type Error = anyhow::Error;

        fn try_from(path: &Path) -> Result<Self, Self::Error> {
            let content = read_to_string(path).map_err(|e| anyhow!("Read error: {:?}", e))?;

            toml::from_str(&content).map_err(|e| anyhow!("Deserialize toml error: {:?}", e))
        }
    }
}

pub use channel::Channel;
pub use client::Client;
pub use config::Config;
pub use create_channel::CreateChannel;
pub use query_status::QueryStatus;
use serde::Deserialize;
pub use whoami::WhoAmI;
