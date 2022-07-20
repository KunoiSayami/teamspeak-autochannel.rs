# TeamSpeak Auto-Channel 
![GitHub](https://img.shields.io/github/license/KunoiSayami/teamspeak-autochannel.rs?style=for-the-badge) ![Build Release](https://img.shields.io/github/workflow/status/KunoiSayami/teamspeak-autochannel.rs/Build%20Releases?style=for-the-badge) ![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/KunoiSayami/teamspeak-autochannel.rs?style=for-the-badge)

This is a simple Rust implement of TeamSpeak 3 Auto-Channel.

## Features

You and other users can get a temporary channel automatically when you join the sepicified channel.

## Configuration

You should create a config file in the same directory as the binary file.

Web query removed since 3.0.0, please use latest 2.x version instead.


```toml
[server]
server_id = 1 # Server ID
channel_id = [1, 2] # Channel ID
privilege_group_id = 5 # Channel Privilege Group ID
redis_server = "" # Redis Server Address

# [[permissions]]
# channel_id = 1
# it means set i_channel_needed_permission_modify_power to 75 and i_channel_needed_delete_power to 60
# See: https://github.com/KunoiSayami/teamspeak-autochannel.rs/wiki/Permission-List for more key information
# map = [[86, 75], [133, 60]]

[misc]
interval = 5 # Interval (milliseconds)

# [custom_message]
# channel_not_found = "I can't find you channel."
# create_channel = "Your Channel has been created!"
# move_to_channel = "You have been moved into your channel."

[raw_query]
server = ""  # TeamSpeak Server Address
port = 10011 # TeamSpeak ServerQuery(Raw) Port
user = "serveradmin" # TeamSpeak ServerQuery Username
password = "114514" # TeamSpeak ServerQuery Password

# web_query section removed since 3.0.0
```

| Name | Type | Required |Description | 
| :---: | :---: | :---: | :--- |
| server_id  | integer | Optional |The ID of the server, which you want to get the channel. <br>If there are multiple servers running, you can get the ID via the TeamSpeak 3 Server Query. <br>Generally, the server ID is `1`. | 
| channel_id | integer, array | Required | The ID of the channel, which you want to listen to. | 
| privilege_group_id | integer | Required |The ID of the privilege group, which will be assigned to user who joins the channel specified by `channel_id`. <br>`5` means Channel Admin Generally. | 
| redis_server | string | Required |Redis Server is Required. Redis Server Should be like `redis://[<username>][:<password>@]<hostname>[:port][/<db>]`. <br>More information about Redis URL can be found [here](https://docs.rs/redis/latest/redis/#connection-parameters). |
| permissions | array | Optional |The permission you want to set to the channel.<br>If you are listening to multiple channels, you can set the permission for each channel by just add another `permissions` section. |
| channel_id | integer | Required |The ID of the channel, which you want to add the permission to. |
| map | array | Optional |The permission you want to set to the channel. <br>For example, `[[86, 75], [133, 60]]` means set i_channel_needed_permission_modify_power to 75 and i_channel_needed_delete_power to 60. <br>See [Permission List](https://github.com/KunoiSayami/teamspeak-autochannel.rs/wiki/Permission-List) for more information. |
| interval | integer | Optional |The interval (milliseconds) between each check. |
| custom_message | table | Optional |The message you want to send to the user who joins the channel. |
| channel_not_found | string | Optional |The message you want to send to the user while user's channel is not found. |
| create_channel | string | Optional |The message you want to send to the user while user's channel is created. |
| move_to_channel | string | Optional |The message you want to send to the user while user is moved to the their channel. |
| raw_query | table | - | **You should choose from `raw_query` and `web_query`.**<br>`raw_query` has higher priority than `web_query`. |
| server | string | Required | TeamSpeak Server Address |
| port | integer | Required | TeamSpeak ServerQuery(Raw) Port |
| user | string | Required | TeamSpeak ServerQuery Username |
| password | string | Required | TeamSpeak ServerQuery Password |
