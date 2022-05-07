mod datastructures;
mod httplib;
mod socketlib;

use crate::datastructures::{ApiMethods, Config};
use crate::httplib::HttpConn;
use crate::socketlib::SocketConn;
use anyhow::anyhow;
use clap::{arg, Command};
use log::{debug, error, info};
use once_cell::sync::OnceCell;
use redis::AsyncCommands;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

static MSG_CHANNEL_NOT_FOUND: OnceCell<String> = OnceCell::new();
static MSG_CREATE_CHANNEL: OnceCell<String> = OnceCell::new();
static MSG_MOVE_TO_CHANNEL: OnceCell<String> = OnceCell::new();

enum ConnectMethod {
    Telnet(String, u16, String, String),
    Http(String, String),
}

async fn bootstrap_connection(config: &Config, sid: i64) -> anyhow::Result<Box<dyn ApiMethods>> {
    if let Some(cfg) = config.raw_query() {
        debug!("Login to server using raw query method");
        init_connection(
            ConnectMethod::Telnet(
                cfg.server(),
                cfg.port(),
                cfg.user().to_string(),
                cfg.password().to_string(),
            ),
            sid,
        )
        .await
    } else {
        debug!("Login to server using web query method");
        let cfg = config.web_query().as_ref().unwrap();
        init_connection(
            ConnectMethod::Http(cfg.server(), cfg.api_key().to_string()),
            sid,
        )
        .await
    }
}

async fn init_connection(method: ConnectMethod, sid: i64) -> anyhow::Result<Box<dyn ApiMethods>> {
    match method {
        ConnectMethod::Telnet(server, port, user, password) => {
            let mut conn = SocketConn::connect(&server, port).await?;
            conn.login(&user, &password)
                .await
                .map_err(|e| anyhow!("Login failed. {:?}", e))?;

            conn.select_server(sid)
                .await
                .map_err(|e| anyhow!("Select server id failed: {:?}", e))?;

            Ok(Box::new(conn))
        }
        ConnectMethod::Http(server, api_key) => {
            let mut conn = HttpConn::new(server, api_key, sid)?;

            conn.who_am_i()
                .await
                .map_err(|e| anyhow!("Check login failed: {:?}", e))?;

            Ok(Box::new(conn))
        }
    }
}

async fn observer(
    conn: Box<dyn ApiMethods>,
    monitor_channels: Vec<i64>,
    privilege_group: i64,
    redis_server: String,
    interval: u64,
    channel_permissions: HashMap<i64, Vec<(u64, i64)>>,
) -> anyhow::Result<()> {
    let (sender, receiver) = tokio::sync::oneshot::channel();

    let staff_handler = tokio::spawn(staff(
        conn,
        monitor_channels,
        privilege_group,
        redis_server,
        interval,
        receiver,
        channel_permissions,
    ));

    tokio::select! {
        _ = async {
            tokio::signal::ctrl_c().await.unwrap();
            info!("Recv SIGINT, send signal to thread.");
            sender.send(true).unwrap();
            tokio::signal::ctrl_c().await.unwrap();
            error!("Force exit program.");
            std::process::exit(137);
        } => {
        }
        ret = staff_handler => {
            ret??
        }
    }

    Ok(())
}

async fn staff(
    mut conn: Box<dyn ApiMethods>,
    monitor_channels: Vec<i64>,
    privilege_group: i64,
    redis_server: String,
    interval: u64,
    mut receiver: tokio::sync::oneshot::Receiver<bool>,
    channel_permissions: HashMap<i64, Vec<(u64, i64)>>,
) -> anyhow::Result<()> {
    info!(
        "Interval is: {}, version: {}",
        interval,
        env!("CARGO_PKG_VERSION")
    );

    let redis = redis::Client::open(redis_server)
        .map_err(|e| anyhow!("Connect redis server error! {:?}", e))?;
    let mut redis_conn = redis
        .get_async_connection()
        .await
        .map_err(|e| anyhow!("Get redis connection error: {:?}", e))?;

    let who_am_i = conn
        .who_am_i()
        .await
        .map_err(|e| anyhow!("Whoami failed: {:?}", e))?;

    let server_info = conn
        .query_server_info()
        .await
        .map_err(|e| anyhow!("Query server info error: {:?}", e))?;

    info!("Connected: {}", who_am_i.clid());

    let mut skip_sleep = false;
    loop {
        if !skip_sleep {
            //std::thread::sleep(Duration::from_millis(interval));
            if tokio::time::timeout(Duration::from_millis(interval), &mut receiver)
                .await
                .is_ok()
            {
                info!("Exit!");
                break;
            }
        } else {
            skip_sleep = false;
        }
        let clients = match conn
            .query_clients()
            .await
            .map_err(|e| error!("Got error while query clients: {:?}", e))
        {
            Ok(clients) => clients,
            Err(_) => continue,
        };

        'outer: for client in clients {
            if client.client_database_id() == who_am_i.cldbid()
                || !monitor_channels.iter().any(|v| *v == client.channel_id())
                || client.client_type() == 1
            {
                continue;
            }
            let key = format!(
                "ts_autochannel_{}_{server_id}_{pid}",
                client.client_database_id(),
                server_id = server_info.virtualserver_unique_identifier(),
                pid = client.channel_id()
            );

            let ret: Option<i64> = redis_conn.get(&key).await?;
            let create_new = ret.is_none();
            let target_channel = if create_new {
                conn.send_text_message(client.client_id(), MSG_CHANNEL_NOT_FOUND.get().unwrap())
                    .await
                    .map_err(|e| error!("Got error while send message: {:?}", e))
                    .ok();

                let mut name = format!("{}'s channel", client.client_nickname());
                let channel_id = loop {
                    let create_channel = match conn.create_channel(&name, client.channel_id()).await
                    {
                        Ok(ret) => ret,
                        Err(e) => {
                            if e.code() == 771 {
                                name = format!("{}1", name);
                                continue;
                            }
                            error!("Got error while create {:?} channel: {:?}", name, e);
                            continue 'outer;
                        }
                    };

                    conn.send_text_message(client.client_id(), MSG_CREATE_CHANNEL.get().unwrap())
                        .await
                        .map_err(|e| error!("Got error while send message: {:?}", e))
                        .ok();

                    break create_channel.unwrap().cid();
                };

                conn.set_client_channel_group(
                    client.client_database_id(),
                    channel_id,
                    privilege_group,
                )
                .await
                .map_err(|e| error!("Got error while set client channel group: {:?}", e))
                .ok();

                conn.add_channel_permission(channel_id, &[(133, 75)])
                    .await
                    .map_err(|e| error!("Got error while set default channel permissions: {:?}", e))
                    .ok();

                if let Some(permissions) = channel_permissions.get(&client.channel_id()) {
                    conn.add_channel_permission(channel_id, permissions)
                        .await
                        .map_err(|e| error!("Got error while set channel permissions: {:?}", e))
                        .ok();
                }

                channel_id
            } else {
                ret.unwrap()
            };

            match conn
                .move_client_to_channel(client.client_id(), target_channel)
                .await
            {
                Ok(ret) => ret,
                Err(e) => {
                    if e.code() == 768 {
                        redis_conn.del(&key).await?;
                        skip_sleep = true;
                        continue;
                    }
                    error!("Got error while move client: {:?}", e);
                    continue;
                }
            };

            conn.send_text_message(client.client_id(), MSG_MOVE_TO_CHANNEL.get().unwrap())
                .await
                .map_err(|e| error!("Got error while send message: {:?}", e))
                .ok();

            if create_new {
                conn.move_client_to_channel(who_am_i.clid(), client.channel_id())
                    .await
                    .map_err(|e| anyhow!("Unable move self out of channel. {:?}", e))?;
                //mapper.insert(client.client_database_id(), target_channel);
                redis_conn.set(&key, target_channel).await?;
            }

            info!("Move {} to {}", client.client_nickname(), target_channel);
        }
    }
    conn.logout().await?;
    Ok(())
}

async fn configure_file_bootstrap<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let config = Config::try_from(path.as_ref())?;
    MSG_CHANNEL_NOT_FOUND
        .set(config.message().channel_not_found())
        .unwrap();
    MSG_CREATE_CHANNEL
        .set(config.message().create_channel())
        .unwrap();
    MSG_MOVE_TO_CHANNEL
        .set(config.message().move_to_channel())
        .unwrap();
    observer(
        bootstrap_connection(&config, config.server().server_id()).await?,
        config.server().channels(),
        config.server().privilege_group_id(),
        config.server().redis_server(),
        config.misc().interval(),
        config.channel_permissions(),
    )
    .await
}

fn main() -> anyhow::Result<()> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(arg!([CONFIG_FILE] "Override default configure file location"))
        .get_matches();

    env_logger::Builder::from_default_env().init();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(configure_file_bootstrap(
            matches.value_of("CONFIG_FILE").unwrap_or("config.toml"),
        ))?;

    Ok(())
}
