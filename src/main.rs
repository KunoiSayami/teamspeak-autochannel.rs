mod datastructures;
mod httplib;
mod socketlib;

use crate::datastructures::{ApiMethods, Config};
use crate::httplib::HttpConn;
use crate::socketlib::SocketConn;
use anyhow::anyhow;
use clap::{arg, Command};
use log::{error, info};
use redis::Commands;
use std::path::Path;
use std::time::Duration;

enum ConnectMethod {
    Telnet(String, u16, String, String),
    Http(String, String),
}

fn bootstrap_connection(config: &Config, sid: i64) -> anyhow::Result<Box<dyn ApiMethods>> {
    if let Some(cfg) = config.raw_query() {
        init_connection(
            ConnectMethod::Telnet(
                cfg.server(),
                cfg.port(),
                cfg.user().to_string(),
                cfg.password().to_string(),
            ),
            sid,
        )
    } else {
        let cfg = config.web_query().as_ref().unwrap();
        init_connection(
            ConnectMethod::Http(cfg.server(), cfg.api_key().to_string()),
            sid,
        )
    }
}

fn init_connection(method: ConnectMethod, sid: i64) -> anyhow::Result<Box<dyn ApiMethods>> {
    match method {
        ConnectMethod::Telnet(server, port, user, password) => {
            let mut conn = SocketConn::connect(&server, port)?;
            let status = conn.login(&user, &password)?;

            if status.is_err() {
                return Err(anyhow!("Login failed. {:?}", status));
            }

            let status = conn.select_server(sid)?;
            if status.is_err() {
                return Err(anyhow!("Select server id failed: {:?}", status));
            }
            Ok(Box::new(conn))
        }
        ConnectMethod::Http(server, api_key) => {
            let mut conn = HttpConn::new(server, api_key, sid)?;

            let status = conn.who_am_i()?.0;

            if status.is_err() {
                return Err(anyhow!("Check login failed: {:?}", status));
            }
            Ok(Box::new(conn))
        }
    }
}

fn observer(
    conn: Box<dyn ApiMethods>,
    monitor_channels: Vec<i64>,
    privilege_group: i64,
    redis_server: String,
    interval: u64,
) -> anyhow::Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();

    ctrlc::set_handler(move || {
        sender.send(true).expect("Send signal error.");
        info!("Recv SIGINT, send signal to thread.");
    })
    .unwrap();
    staff(
        conn,
        monitor_channels,
        privilege_group,
        redis_server,
        interval,
        receiver,
    )
}

fn staff(
    mut conn: Box<dyn ApiMethods>,
    monitor_channels: Vec<i64>,
    privilege_group: i64,
    redis_server: String,
    interval: u64,
    receiver: std::sync::mpsc::Receiver<bool>,
) -> anyhow::Result<()> {
    info!("Interval is: {}", interval);

    let redis = redis::Client::open(redis_server)
        .map_err(|e| anyhow!("Connect redis server error! {:?}", e))?;
    let mut redis_conn = redis
        .get_connection()
        .map_err(|e| anyhow!("Get redis connection error: {:?}", e))?;

    let (status, who_am_i) = conn.who_am_i()?;
    if status.is_err() {
        return Err(anyhow!("Whoami failed: {:?}", status));
    }

    let (status, server_info) = conn.query_server_info()?;
    if status.is_err() {
        return Err(anyhow!("Query server info error: {:?}", status));
    }

    info!("Connected: {}", who_am_i.clid());

    let mut skip_sleep = false;
    loop {
        if !skip_sleep {
            //std::thread::sleep(Duration::from_millis(interval));
            if let Ok(_) = receiver.recv_timeout(Duration::from_millis(interval)) {
                info!("Exit!");
                break;
            }
        } else {
            skip_sleep = false;
        }
        let (status, clients) = conn.query_clients()?;

        if status.is_err() {
            error!("Got error while query clients: {:?}", status);
            continue;
        }

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

            let ret: Option<i64> = redis_conn.get(&key)?;
            let create_new = ret.is_none();
            let target_channel = if create_new {
                conn.send_text_message(client.client_id(), "I can't find you channel.")
                    .map_err(|e| error!("Got error while send message: {:?}", e))
                    .ok();

                let mut name = format!("{}'s channel", client.client_nickname());
                let channel_id = loop {
                    let (status, create_channel) =
                        match conn.create_channel(&name, client.channel_id()) {
                            Ok(ret) => ret,
                            Err(e) => {
                                error!("Got error while create channel: {:?}", e);
                                continue;
                            }
                        };

                    if status.is_err() {
                        if status.id() == 771 {
                            //let (original, _) = name.rsplit_once("'s").unwrap();
                            name = format!("{}1", name);
                            continue;
                        }
                        error!("Got error while create {:?} channel: {:?}", name, status);
                        continue 'outer;
                    }
                    conn.send_text_message(client.client_id(), "Your Channel has been created!")
                        .map_err(|e| error!("Got error while send message: {:?}", e))
                        .ok();

                    break create_channel.unwrap().cid();
                };
                conn.set_client_channel_group(
                    client.client_database_id(),
                    channel_id,
                    privilege_group,
                )
                .map_err(|e| error!("Got error while set client channel group: {:?}", e))
                .ok();
                channel_id
            } else {
                ret.unwrap()
            };

            let (status, channels) = conn.query_channels()?;

            if status.is_err() {
                error!("Got error while channellist: {:?}", status);
                continue;
            }

            if !channels
                .iter()
                .any(|c| target_channel == c.cid() && c.pid() == client.channel_id())
            {
                redis_conn.del(&key)?;
                skip_sleep = true;
                continue;
            }

            let status = match conn.move_client_to_channel(client.client_id(), target_channel) {
                Ok(ret) => ret,
                Err(e) => {
                    error!("Got error while move client: {:?}", e);
                    continue;
                }
            };

            if status.is_err() {
                if status.id() == 768 {
                    redis_conn.del(&key)?;
                    skip_sleep = true;
                    continue;
                }
                error!("Got error while move client: {:?}", status)
            }

            conn.send_text_message(client.client_id(), "You have been moved into your channel")
                .map_err(|e| error!("Got error while send message: {:?}", e))
                .ok();

            if create_new {
                conn.move_client_to_channel(who_am_i.clid(), client.channel_id())
                    .unwrap();
                //mapper.insert(client.client_database_id(), target_channel);
                redis_conn.set(&key, target_channel)?;
            }

            info!("Move {} to {}", client.client_nickname(), target_channel);
        }
    }
    conn.logout()?;
    Ok(())
}

fn configure_file_bootstrap<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let config = Config::try_from(path.as_ref())?;
    observer(
        bootstrap_connection(&config, config.server().server_id())?,
        config.server().channels(),
        config.server().privilege_group_id(),
        config.server().redis_server(),
        config.misc().interval(),
    )
}

fn main() -> anyhow::Result<()> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(arg!([CONFIG_FILE] "Override default configure file location"))
        .get_matches();

    env_logger::Builder::from_default_env().init();

    configure_file_bootstrap(matches.value_of("CONFIG_FILE").unwrap_or("config.toml"))?;

    Ok(())
}
