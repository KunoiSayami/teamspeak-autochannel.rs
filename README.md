# TeamSpeak Auto-Channel 
![GitHub](https://img.shields.io/github/license/KunoiSayami/teamspeak-autochannel.rs?style=for-the-badge) ![Build Release](https://img.shields.io/github/workflow/status/KunoiSayami/teamspeak-autochannel.rs/Build%20Releases?style=for-the-badge) ![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/KunoiSayami/teamspeak-autochannel.rs?style=for-the-badge)

This is a simple Rust implement of TeamSpeak 3 Auto-Channel.

## Features

You and other users can get a temporary channel automatically when you join the sepicified channel.

## Configuration

You should create a config file in the same directory as the binary file.
   
```toml
[server]
server_id = 1 # Server ID
channel_id = [1, 2] # Channel ID
privilege_group_id = 5 # Channel Privilege Group ID
redis_server = "" # Redis Server Address

[misc]
interval = 5 # Interval (sec)

[raw_query]
# This section priority is higher than web_query
server = ""  # TeamSpeak Server Address
port = 10011 # TeamSpeak ServerQuery(Raw) Port
user = "serveradmin" # TeamSpeak ServerQuery Username
password = "114514" # TeamSpeak ServerQuery Password

[web_query]
# This method may significantly increase CPU usage.
server = "https://localhost:10080" # TeamSpeak WebQuery Server Address
api_key = "BAA114514" # TeamSpeak WebQuery API Key
```

| Name | Type | Required |Description | 
| :---: | :---: | :---: | :--- |
| server_id  | integer | Optional |The ID of the server, which you want to get the channel. <br>If there are multiple servers running, you can get the ID via the TeamSpeak 3 Server Query. <br>Generally, the server ID is `1`. | 
| channel_id | integer, array | Required | The ID of the channel, which you want to listen to. | 
| privilege_group_id | integer | Required |The ID of the privilege group, which will be assigned to user who joins the channel specified by `channel_id`. <br>`5` means Channel Admin Generally. | 
| redis_server | string | Required |Redis Server is Required. Redis Server Should be like `redis://redis.example.com?db=0&password=password`. <br>More information about Redis URL can be found [here](https://metacpan.org/pod/URI::redis). |
| interval | integer | Optional |The interval (sec) between each check. |
| raw_query | table | - | **You should choose from `raw_query` and `web_query`.**<br>`raw_query` has higher priority than `web_query`. |
| server | string | Required | TeamSpeak Server Address |
| port | integer | Required | TeamSpeak ServerQuery(Raw) Port |
| user | string | Required | TeamSpeak ServerQuery Username |
| password | string | Required | TeamSpeak ServerQuery Password |
| web_query | table | - | TeamSpeak WebQuery<br>This method may significantly increase CPU usage. |
| server | string | Required | TeamSpeak WebQuery Server Address |
| api_key | string | Required |TeamSpeak WebQuery API Key |
