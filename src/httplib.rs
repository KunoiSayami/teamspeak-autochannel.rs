use crate::datastructures::{
    Channel, Client, CreateChannel, FromJSON, QueryStatus, ServerInfo, WebQueryStatus, WhoAmI,
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

    pub async fn make_request(
        &self,
        method: &str,
        payload: &[(&str, &str)],
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

    pub async fn basic_operation(
        &self,
        method: &str,
        payload: &[(&str, &str)],
    ) -> anyhow::Result<QueryStatus> {
        let response = self.make_request(method, payload).await?;

        //debug!("response => {}", &response);

        let response: Response =
            serde_json::from_str(&response).map_err(|e| anyhow!("Got parser error: {:?}", e))?;
        Ok(response.status.to_status())
    }

    pub async fn query_operation<T: FromJSON + Sized>(
        &self,
        method: &str,
        payload: &[(&str, &str)],
    ) -> anyhow::Result<(QueryStatus, Option<Vec<T>>)> {
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
        Ok((status.to_status(), response))
    }

    pub async fn query_operation_non_error<T: FromJSON + Sized>(
        &self,
        method: &str,
        payload: &[(&str, &str)],
    ) -> anyhow::Result<(QueryStatus, Vec<T>)> {
        let (status, response) = self.query_operation(method, payload).await?;
        Ok((
            status,
            response.ok_or_else(|| anyhow!("Response is none: (method: {})", method))?,
        ))
    }

    pub async fn query_operation_1<T: FromJSON + Sized>(
        &self,
        method: &str,
        payload: &[(&str, &str)],
    ) -> anyhow::Result<(QueryStatus, Option<T>)> {
        let (status, response) = self.query_operation(method, payload).await?;
        Ok((status, response.map(|mut v| v.remove(0))))
    }

    pub async fn query_operation_1_non_error<T: FromJSON + Sized>(
        &self,
        method: &str,
        payload: &[(&str, &str)],
    ) -> anyhow::Result<(QueryStatus, T)> {
        let (status, response) = self.query_operation_1(method, payload).await?;
        Ok((
            status,
            response.ok_or_else(|| anyhow!("Response is none: (method: {})", method))?,
        ))
    }
}

#[async_trait::async_trait]
impl ApiMethods for HttpConn {
    async fn who_am_i(&mut self) -> anyhow::Result<(QueryStatus, WhoAmI)> {
        self.query_operation_1_non_error("whoami", &[]).await
    }

    async fn send_text_message(&mut self, clid: i64, text: &str) -> anyhow::Result<QueryStatus> {
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

    async fn query_server_info(&mut self) -> anyhow::Result<(QueryStatus, ServerInfo)> {
        self.query_operation_1_non_error("serverinfo", &[]).await
    }

    async fn query_channels(&mut self) -> anyhow::Result<(QueryStatus, Vec<Channel>)> {
        self.query_operation_non_error("channellist", &[]).await
    }

    async fn create_channel(
        &mut self,
        name: &str,
        pid: i64,
    ) -> anyhow::Result<(QueryStatus, Option<CreateChannel>)> {
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

    async fn query_clients(&mut self) -> anyhow::Result<(QueryStatus, Vec<Client>)> {
        self.query_operation_non_error("clientlist", &[]).await
    }

    async fn move_client_to_channel(
        &mut self,
        clid: i64,
        target_channel: i64,
    ) -> anyhow::Result<QueryStatus> {
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
    ) -> anyhow::Result<QueryStatus> {
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

    async fn logout(&mut self) -> anyhow::Result<QueryStatus> {
        Ok(QueryStatus::default())
    }
}
