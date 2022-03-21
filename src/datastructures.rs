pub trait FromQueryString {
    fn from_query(data: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub mod channel {
    use crate::FromQueryString;
    use serde_derive::Deserialize;

    #[derive(Clone, Debug, Default, Deserialize)]
    pub struct Channel {
        cid: i64,
        pid: i64,
        channel_order: i64,
        channel_name: String,
        total_clients: i64,
        channel_needed_subscribe_power: i64,
    }

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

    impl FromQueryString for Channel {
        fn from_query(data: &str) -> anyhow::Result<Self>
        where
            Self: Sized,
        {
            Ok(serde_teamspeak_querystring::from_str(data)
                .map_err(|e| anyhow::anyhow!("Got parser error: {:?}", e))?)
        }
    }
}

pub mod client {
    use crate::datastructures::FromQueryString;
    use anyhow::anyhow;
    use serde_derive::Deserialize;

    #[derive(Clone, Debug, Default, Deserialize)]
    pub struct Client {
        clid: i64,
        cid: i64,
        client_database_id: i64,
        client_type: i64,
        client_unique_identifier: String,
        client_nickname: String,
    }

    impl Client {
        pub fn clid(&self) -> i64 {
            self.clid
        }
        pub fn cid(&self) -> i64 {
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

    impl FromQueryString for Client {
        fn from_query(data: &str) -> anyhow::Result<Self>
        where
            Self: Sized,
        {
            Ok(serde_teamspeak_querystring::from_str(data)
                .map_err(|e| anyhow::anyhow!("Got parser error: {:?}", e))?)
        }
    }

    #[cfg(test)]
    mod test {
        use crate::datastructures::client::Client;
        use crate::datastructures::FromQueryString;

        const TEST_STRING: &str = "clid=8 cid=1 client_database_id=1 client_nickname=serveradmin client_type=1 client_unique_identifier=serveradmin";

        #[test]
        fn test() {
            let result = Client::from_query(TEST_STRING).unwrap();
            assert_eq!(result.clid(), 8);
            assert_eq!(result.cid(), 1);
            assert_eq!(result.client_database_id(), 1);
            assert_eq!(result.client_nickname(), "serveradmin".to_string());
            assert_eq!(result.client_type(), 1);
            assert_eq!(result.client_unique_identifier(), "serveradmin".to_string());
        }
    }
}

pub mod query_status {
    use anyhow::anyhow;

    #[derive(Clone, Debug)]
    pub struct QueryStatus {
        id: i32,
        msg: String,
    }

    impl QueryStatus {
        pub fn new(id: i32, msg: String) -> Self {
            Self { id, msg }
        }

        pub fn id(&self) -> i32 {
            self.id
        }
        pub fn msg(&self) -> &str {
            &self.msg
        }

        pub fn is_ok(&self) -> bool {
            self.id == 0
        }
    }

    impl TryFrom<&str> for QueryStatus {
        type Error = anyhow::Error;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let (_, line) = value
                .split_once("error ")
                .ok_or_else(|| anyhow!("Split error: {}", value))?;
            let (id, msg) = line
                .split_once(' ')
                .ok_or_else(|| anyhow!("Split error: {}", line))?;
            debug_assert!(id.contains('='));
            debug_assert!(msg.contains('='));
            let (_, id) = id.split_once('=').unwrap();
            let (_, msg) = msg.split_once('=').unwrap();
            Ok(Self::new(
                id.parse()
                    .map_err(|e| anyhow!("Got parse error: {:?}", e))?,
                msg.replace("\\s", " ").to_string(),
            ))
        }
    }
}

pub use query_status::QueryStatus;
use serde::Deserialize;
