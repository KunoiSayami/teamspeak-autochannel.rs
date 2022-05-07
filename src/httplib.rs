use crate::datastructures::{
    Channel, Client, CreateChannel, FromJSON, QueryError, QueryResult, ServerInfo, WebQueryStatus,
    WhoAmI,
};
use crate::ApiMethods;
use anyhow::anyhow;
use serde_derive::Deserialize;
use std::time::Duration;

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    body: Option<serde_json::Value>,
    status: WebQueryStatus,
}

pub struct HttpConn {
    url: String,
    api_key: String,
    sid: i64,
    client: reqwest::Client,
}

impl HttpConn {
    pub fn new(server: String, api_key: String, sid: i64) -> anyhow::Result<Self> {
        Ok(Self {
            url: server,
            api_key,
            sid,
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(10))
                .build()
                .map_err(|e| anyhow!("Build client error: {:?}", e))?,
        })
    }

    pub async fn make_request<T: serde::Serialize + ?Sized>(
        &self,
        method: &str,
        payload: &T,
    ) -> anyhow::Result<String> {
        self.client
            .post(format!("{}/{}/{}", self.url, self.sid, method))
            .query(payload)
            .query(&[("api-key", &self.api_key)])
            .send()
            .await
            .map_err(|e| {
                anyhow!(
                    "Got error while request {method}, {error:?}",
                    method = method,
                    error = e
                )
            })?
            .text()
            .await
            .map_err(|e| anyhow!("Got error while get text: {:?}", e))
    }

    pub async fn basic_operation<T: serde::Serialize + ?Sized>(
        &self,
        method: &str,
        payload: &T,
    ) -> QueryResult<()> {
        let response = self.make_request(method, payload).await?;

        //debug!("response => {}", &response);

        let response: Response =
            serde_json::from_str(&response).map_err(|e| anyhow!("Got parser error: {:?}", e))?;
        response.status.into_status().into_result(())
    }

    pub async fn query_operation<T: FromJSON + Sized>(
        &self,
        method: &str,
        payload: &[(&str, &str)],
    ) -> QueryResult<Option<Vec<T>>> {
        let response: String = self.make_request(method, payload).await?;

        //debug!("response => {}", &response);

        let response: Response = serde_json::from_str(&response)
            .map_err(|e| anyhow!("Got error while parse json: {:?}", e))?;

        let status = response.status;
        //let response: T = serde_json::from_value(response.body)
        let response = match response.body {
            None => None,
            Some(element) => {
                let response = if element.is_array() {
                    let mut v = Vec::new();
                    for element in element.as_array().unwrap() {
                        v.push(T::from_value(element.clone())?)
                    }
                    v
                } else {
                    let val: T = T::from_value(element)?;
                    vec![val]
                };
                Some(response)
            }
        };
        status.into_status().into_result(response)
    }

    pub async fn query_operation_non_error<T: FromJSON + Sized>(
        &self,
        method: &str,
        payload: &[(&str, &str)],
    ) -> QueryResult<Vec<T>> {
        let response = self.query_operation(method, payload).await?;
        if response.is_none() {
            return Err(QueryError::static_empty_response());
        }
        Ok(response.unwrap())
    }

    pub async fn query_operation_1<T: FromJSON + Sized>(
        &self,
        method: &str,
        payload: &[(&str, &str)],
    ) -> QueryResult<Option<T>> {
        Ok(self
            .query_operation(method, payload)
            .await?
            .map(|mut v| v.swap_remove(0)))
    }

    pub async fn query_operation_1_non_error<T: FromJSON + Sized>(
        &self,
        method: &str,
        payload: &[(&str, &str)],
    ) -> QueryResult<T> {
        self.query_operation_1(method, payload)
            .await?
            .ok_or_else(QueryError::static_empty_response)
    }
}

#[async_trait::async_trait]
impl ApiMethods for HttpConn {
    async fn who_am_i(&mut self) -> QueryResult<WhoAmI> {
        self.query_operation_1_non_error("whoami", &[]).await
    }

    async fn send_text_message(&mut self, clid: i64, text: &str) -> QueryResult<()> {
        self.basic_operation(
            "sendtextmessage",
            &[
                ("targetmode", "1"),
                ("target", &format!("{}", clid)),
                ("msg", text),
            ],
        )
        .await
    }

    async fn query_server_info(&mut self) -> QueryResult<ServerInfo> {
        self.query_operation_1_non_error("serverinfo", &[]).await
    }

    async fn query_channels(&mut self) -> QueryResult<Vec<Channel>> {
        self.query_operation_non_error("channellist", &[]).await
    }

    async fn create_channel(&mut self, name: &str, pid: i64) -> QueryResult<Option<CreateChannel>> {
        self.query_operation_1(
            "channelcreate",
            &[
                ("channel_name", name),
                ("cpid", &format!("{}", pid)),
                ("channel_codec_quality", "6"),
            ],
        )
        .await
    }

    async fn query_clients(&mut self) -> QueryResult<Vec<Client>> {
        self.query_operation_non_error("clientlist", &[]).await
    }

    async fn move_client_to_channel(&mut self, clid: i64, target_channel: i64) -> QueryResult<()> {
        self.basic_operation(
            "clientmove",
            &[
                ("clid", &format!("{}", clid)),
                ("cid", &format!("{}", target_channel)),
            ],
        )
        .await
    }

    async fn set_client_channel_group(
        &mut self,
        client_database_id: i64,
        channel_id: i64,
        group_id: i64,
    ) -> QueryResult<()> {
        self.basic_operation(
            "setclientchannelgroup",
            &[
                ("cgid", &format!("{}", group_id)),
                ("cid", &format!("{}", channel_id)),
                ("cldbid", &format!("{}", client_database_id)),
            ],
        )
        .await
    }

    async fn add_channel_permission(
        &mut self,
        target_channel: i64,
        permissions: &[(u64, i64)],
    ) -> QueryResult<()> {
        let mut params = vec![("cid", format!("{}", target_channel))];
        for (k, v) in permissions {
            params.push(("permid", format!("{}", k)));
            params.push(("permvalue", format!("{}", v)));
        }
        self.basic_operation("channeladdperm", &params).await
    }

    async fn logout(&mut self) -> QueryResult<()> {
        Ok(())
    }
}
