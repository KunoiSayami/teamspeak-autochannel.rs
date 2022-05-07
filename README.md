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

# [[permissions]]
# channel_id = 1
# it means set i_channel_needed_permission_modify_power to 75 and i_channel_needed_delete_power to 60
# See: https://gist.github.com/EdisonJwa/630383a12281f3db2186a14e1d0fa901 for more key information
# map = [[86, 75], [133, 60]]

[misc]
interval = 5 # Interval (milliseconds)

# [custom_message]
# channel_not_found = "I can't find you channel."
# create_channel = "Your Channel has been created!"
# move_to_channel = "You have been moved into your channel."

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
| redis_server | string | Required |Redis Server is Required. Redis Server Should be like `redis://[<username>][:<password>@]<hostname>[:port][/<db>]`. <br>More information about Redis URL can be found [here](https://docs.rs/redis/latest/redis/#connection-parameters). |
| permissions | array | Optional |The permission you want to set to the channel.<br>If you are listening to multiple channels, you can set the permission for each channel by just add another `permissions` section. |
| channel_id | integer | Required |The ID of the channel, which you want to add the permission to. |
| map | array | Optional |The permission you want to set to the channel. <br>For example, `[[86, 75], [133, 60]]` means set i_channel_needed_permission_modify_power to 75 and i_channel_needed_delete_power to 60. <br>See [Permission List](#permission-list) for more information. |
| interval | integer | Optional |The interval (milliseconds) between each check. |
| custom_message | string | Optional |The message you want to send to the user who joins the channel. |
| channel_not_found | string | Optional |The message you want to send to the user while user's channel is not found. |
| create_channel | string | Optional |The message you want to send to the user while user's channel is created. |
| move_to_channel | string | Optional |The message you want to send to the user while user is moved to the their channel. |
| raw_query | table | - | **You should choose from `raw_query` and `web_query`.**<br>`raw_query` has higher priority than `web_query`. |
| server | string | Required | TeamSpeak Server Address |
| port | integer | Required | TeamSpeak ServerQuery(Raw) Port |
| user | string | Required | TeamSpeak ServerQuery Username |
| password | string | Required | TeamSpeak ServerQuery Password |
| web_query | table | - | TeamSpeak WebQuery<br>This method may significantly increase CPU usage. |
| server | string | Required | TeamSpeak WebQuery Server Address |
| api_key | string | Required |TeamSpeak WebQuery API Key |


## Permission List
If you want to know the permission ID, you can check the following table.

| ID | Name | Description | 
|---|---|:---|
| 1 | b_serverinstance_help_view | Retrieve information about ServerQuery commands| 
| 2 | b_serverinstance_info_view | Retrieve global server information| 
| 3 | b_serverinstance_virtualserver_list | List virtual servers stored in the database| 
| 4 | b_serverinstance_binding_list | List active IP bindings on multi-homed machines| 
| 5 | b_serverinstance_permission_list | List permissions available available on the server instance| 
| 6 | b_serverinstance_permission_find | Search permission assignments by name or ID| 
| 7 | b_virtualserver_create | Create virtual servers| 
| 8 | b_virtualserver_delete | Delete virtual servers| 
| 9 | b_virtualserver_start_any | Start any virtual server in the server instance| 
| 10 | b_virtualserver_stop_any | Stop any virtual server in the server instance| 
| 11 | b_virtualserver_change_machine_id | Change a virtual servers machine ID| 
| 12 | b_virtualserver_change_template | Edit virtual server default template values| 
| 13 | b_serverquery_login | Login to ServerQuery| 
| 14 | b_serverquery_login_create | Create a new server query login| 
| 15 | b_serverquery_login_delete | Delete a server query login| 
| 16 | b_serverquery_login_list | List server query logins| 
| 17 | b_serverinstance_textmessage_send | Send text messages to all virtual servers at once| 
| 18 | b_serverinstance_log_view | Retrieve global server log| 
| 19 | b_serverinstance_log_add | Write to global server log| 
| 20 | b_serverinstance_stop | Shutdown the server process| 
| 21 | b_serverinstance_modify_settings | Edit global settings| 
| 22 | b_serverinstance_modify_querygroup | Edit global ServerQuery groups| 
| 23 | b_serverinstance_modify_templates | Edit global template groups| 
| 24 | b_virtualserver_select | Select a virtual server| 
| 25 | b_virtualserver_info_view | Retrieve virtual server information| 
| 26 | b_virtualserver_connectioninfo_view | Retrieve virtual server connection information| 
| 27 | b_virtualserver_channel_list | List channels on a virtual server| 
| 28 | b_virtualserver_channel_search | Search for channels on a virtual server| 
| 29 | b_virtualserver_client_list | List clients online on a virtual server| 
| 30 | b_virtualserver_client_search | Search for clients online on a virtual server| 
| 31 | b_virtualserver_client_dblist | List client identities known by the virtual server| 
| 32 | b_virtualserver_client_dbsearch | Search for client identities known by the virtual server| 
| 33 | b_virtualserver_client_dbinfo | Retrieve client information| 
| 34 | b_virtualserver_permission_find | Find permissions| 
| 35 | b_virtualserver_custom_search | Find custom fields| 
| 36 | b_virtualserver_start | Start own virtual server| 
| 37 | b_virtualserver_stop | Stop own virtual server| 
| 38 | b_virtualserver_token_list | List privilege keys available| 
| 39 | b_virtualserver_token_add | Create new privilege keys| 
| 40 | b_virtualserver_token_use | Use a privilege keys to gain access to groups| 
| 41 | b_virtualserver_token_delete | Delete a privilege key| 
| 42 | b_virtualserver_apikey_add | Create a new API key| 
| 43 | b_virtualserver_apikey_manage | Manage existing API keys| 
| 44 | b_virtualserver_log_view | Retrieve virtual server log| 
| 45 | b_virtualserver_log_add | Write to virtual server log| 
| 46 | b_virtualserver_join_ignore_password | Join virtual server ignoring its password| 
| 47 | b_virtualserver_notify_register | Register for server notifications| 
| 48 | b_virtualserver_notify_unregister | Unregister from server notifications| 
| 49 | b_virtualserver_snapshot_create | Create server snapshots| 
| 50 | b_virtualserver_snapshot_deploy | Deploy server snapshots| 
| 51 | b_virtualserver_permission_reset | Reset the server permission settings to default values| 
| 52 | b_virtualserver_modify_name | Modify server name| 
| 53 | b_virtualserver_modify_welcomemessage | Modify welcome message| 
| 54 | b_virtualserver_modify_maxclients | Modify servers max clients| 
| 55 | b_virtualserver_modify_reserved_slots | Modify reserved slots| 
| 56 | b_virtualserver_modify_password | Modify server password| 
| 57 | b_virtualserver_modify_default_servergroup | Modify default Server Group| 
| 58 | b_virtualserver_modify_default_channelgroup | Modify default Channel Group| 
| 59 | b_virtualserver_modify_default_channeladmingroup | Modify default Channel Admin Group| 
| 60 | b_virtualserver_modify_channel_forced_silence | Modify channel force silence value| 
| 61 | b_virtualserver_modify_complain | Modify individual complain settings| 
| 62 | b_virtualserver_modify_antiflood | Modify individual antiflood settings| 
| 63 | b_virtualserver_modify_ft_settings | Modify file transfer settings| 
| 64 | b_virtualserver_modify_ft_quotas | Modify file transfer quotas| 
| 65 | b_virtualserver_modify_hostmessage | Modify individual hostmessage settings| 
| 66 | b_virtualserver_modify_hostbanner | Modify individual hostbanner settings| 
| 67 | b_virtualserver_modify_hostbutton | Modify individual hostbutton settings| 
| 68 | b_virtualserver_modify_port | Modify server port| 
| 69 | b_virtualserver_modify_autostart | Modify server autostart| 
| 70 | b_virtualserver_modify_needed_identity_security_level | Modify required identity security level| 
| 71 | b_virtualserver_modify_priority_speaker_dimm_modificator | Modify priority speaker dimm modificator| 
| 72 | b_virtualserver_modify_log_settings | Modify log settings| 
| 73 | b_virtualserver_modify_min_client_version | Modify min client version| 
| 74 | b_virtualserver_modify_icon_id | Modify server icon| 
| 75 | b_virtualserver_modify_weblist | Modify web server list reporting settings| 
| 76 | b_virtualserver_modify_codec_encryption_mode | Modify codec encryption mode| 
| 77 | b_virtualserver_modify_temporary_passwords | Modify temporary serverpasswords| 
| 78 | b_virtualserver_modify_temporary_passwords_own | Modify own temporary serverpasswords| 
| 79 | b_virtualserver_modify_channel_temp_delete_delay_default | Modify default temporary channel delete delay| 
| 80 | b_virtualserver_modify_nickname | Modify server nicknames| 
| 81 | b_virtualserver_modify_integrations | Modify integrations| 
| 82 | i_channel_min_depth | Min channel creation depth in hierarchy| 
| 83 | i_channel_max_depth | Max channel creation depth in hierarchy| 
| 84 | b_channel_group_inheritance_end | Stop inheritance of channel group permissions| 
| 85 | i_channel_permission_modify_power | Modify channel permission power| 
| 86 | i_channel_needed_permission_modify_power | Needed modify channel permission power| 
| 87 | b_channel_info_view | Retrieve channel information| 
| 88 | b_channel_create_child | Create sub-channels| 
| 89 | b_channel_create_permanent | Create permanent channels| 
| 90 | b_channel_create_semi_permanent | Create semi-permanent channels| 
| 91 | b_channel_create_temporary | Create temporary channels| 
| 92 | b_channel_create_with_topic | Create channels with a topic| 
| 93 | b_channel_create_with_description | Create channels with a description| 
| 94 | b_channel_create_with_password | Create password protected channels| 
| 95 | b_channel_create_with_banner | Create channel with a banner| 
| 96 | b_channel_create_modify_with_codec_opusvoice | Create channels using OPUS (voice) codec| 
| 97 | b_channel_create_modify_with_codec_opusmusic | Create channels using OPUS (music) codec| 
| 98 | i_channel_create_modify_with_codec_maxquality | Create channels with custom codec quality| 
| 99 | i_channel_create_modify_with_codec_latency_factor_min | Create channels with minimal custom codec latency factor| 
| 100 | b_channel_create_with_maxclients | Create channels with custom max clients| 
| 101 | b_channel_create_with_maxfamilyclients | Create channels with custom max family clients| 
| 102 | b_channel_create_with_sortorder | Create channels with custom sort order| 
| 103 | b_channel_create_with_default | Create default channels| 
| 104 | b_channel_create_with_needed_talk_power | Create channels with needed talk power| 
| 105 | b_channel_create_modify_with_force_password | Create new channels only with password| 
| 106 | i_channel_create_modify_with_temp_delete_delay | Max delete delay for temporary channels| 
| 107 | b_channel_modify_parent | Move channels| 
| 108 | b_channel_modify_make_default | Make channel default| 
| 109 | b_channel_modify_make_permanent | Make channel permanent| 
| 110 | b_channel_modify_make_semi_permanent | Make channel semi-permanent| 
| 111 | b_channel_modify_make_temporary | Make channel temporary| 
| 112 | b_channel_modify_name | Modify channel name| 
| 113 | b_channel_modify_topic | Modify channel topic| 
| 114 | b_channel_modify_description | Modify channel description| 
| 115 | b_channel_modify_password | Modify channel password| 
| 116 | b_channel_modify_banner | Modify channel banner| 
| 117 | b_channel_modify_codec | Modify channel codec| 
| 118 | b_channel_modify_codec_quality | Modify channel codec quality| 
| 119 | b_channel_modify_codec_latency_factor | Modify channel codec latency factor| 
| 120 | b_channel_modify_maxclients | Modify channels max clients| 
| 121 | b_channel_modify_maxfamilyclients | Modify channels max family clients| 
| 122 | b_channel_modify_sortorder | Modify channel sort order| 
| 123 | b_channel_modify_needed_talk_power | Change needed channel talk power| 
| 124 | i_channel_modify_power | Channel modify power| 
| 125 | i_channel_needed_modify_power | Needed channel modify power| 
| 126 | b_channel_modify_make_codec_encrypted | Make channel codec encrypted| 
| 127 | b_channel_modify_temp_delete_delay | Modify temporary channel delete delay| 
| 128 | b_channel_delete_permanent | Delete permanent channels| 
| 129 | b_channel_delete_semi_permanent | Delete semi-permanent channels| 
| 130 | b_channel_delete_temporary | Delete temporary channels| 
| 131 | b_channel_delete_flag_force | Force channel delete| 
| 132 | i_channel_delete_power | Delete channel power| 
| 133 | i_channel_needed_delete_power | Needed delete channel power| 
| 134 | b_channel_join_permanent | Join permanent channels| 
| 135 | b_channel_join_semi_permanent | Join semi-permanent channels| 
| 136 | b_channel_join_temporary | Join temporary channels| 
| 137 | b_channel_join_ignore_password | Join channel ignoring its password| 
| 138 | b_channel_join_ignore_maxclients | Ignore channels max clients limit| 
| 139 | i_channel_join_power | Channel join power| 
| 140 | i_channel_needed_join_power | Needed channel join power| 
| 141 | i_channel_subscribe_power | Channel subscribe power| 
| 142 | i_channel_needed_subscribe_power | Needed channel subscribe power| 
| 143 | i_channel_description_view_power | Channel description view power| 
| 144 | i_channel_needed_description_view_power | Needed channel needed description view power| 
| 145 | i_icon_id | Group icon identifier| 
| 146 | i_max_icon_filesize | Max icon filesize in bytes| 
| 147 | b_icon_manage | Enables icon management| 
| 148 | b_group_is_permanent | Group is permanent| 
| 149 | i_group_auto_update_type | Group auto-update type| 
| 150 | i_group_auto_update_max_value | Group auto-update max value| 
| 151 | i_group_sort_id | Group sort id| 
| 152 | i_group_show_name_in_tree | Show group name in tree depending on selected mode| 
| 153 | b_virtualserver_servergroup_list | List server groups| 
| 154 | b_virtualserver_servergroup_permission_list | List server group permissions| 
| 155 | b_virtualserver_servergroup_client_list | List clients from a server group| 
| 156 | b_virtualserver_channelgroup_list | List channel groups| 
| 157 | b_virtualserver_channelgroup_permission_list | List channel group permissions| 
| 158 | b_virtualserver_channelgroup_client_list | List clients from a channel group| 
| 159 | b_virtualserver_client_permission_list | List client permissions| 
| 160 | b_virtualserver_channel_permission_list | List channel permissions| 
| 161 | b_virtualserver_channelclient_permission_list | List channel client permissions| 
| 162 | b_virtualserver_servergroup_create | Create server groups| 
| 163 | b_virtualserver_channelgroup_create | Create channel groups| 
| 164 | i_group_modify_power | Group modify power| 
| 165 | i_group_needed_modify_power | Needed group modify power| 
| 166 | i_group_member_add_power | Group member add power| 
| 167 | i_group_needed_member_add_power | Needed group member add power| 
| 168 | i_group_member_remove_power | Group member delete power| 
| 169 | i_group_needed_member_remove_power | Needed group member delete power| 
| 170 | i_permission_modify_power | Permission modify power| 
| 171 | b_permission_modify_power_ignore | Ignore needed permission modify power| 
| 172 | b_virtualserver_servergroup_delete | Delete server groups| 
| 173 | b_virtualserver_channelgroup_delete | Delete channel groups| 
| 174 | i_client_permission_modify_power | Client permission modify power| 
| 175 | i_client_needed_permission_modify_power | Needed client permission modify power| 
| 176 | i_client_max_clones_uid | Max additional connections per client identity| 
| 177 | i_client_max_idletime | Max idle time in seconds| 
| 178 | i_client_max_avatar_filesize | Max avatar filesize in bytes| 
| 179 | i_client_max_channel_subscriptions | Max channel subscriptions| 
| 180 | b_client_is_priority_speaker | Client is priority speaker| 
| 181 | b_client_skip_channelgroup_permissions | Ignore channel group permissions| 
| 182 | b_client_force_push_to_talk | Force Push-To-Talk capture mode| 
| 183 | b_client_ignore_bans | Ignore bans| 
| 184 | b_client_ignore_antiflood | Ignore antiflood measurements| 
| 185 | b_client_use_reserved_slot | Use an reserved slot| 
| 186 | b_client_use_channel_commander | Use channel commander| 
| 187 | b_client_request_talker | Allow to request talk power| 
| 188 | b_client_avatar_delete_other | Allow deletion of avatars from other clients| 
| 189 | b_client_is_sticky | Client will be sticked to current channel| 
| 190 | b_client_ignore_sticky | Client ignores sticky flag| 
| 191 | b_client_info_view | Retrieve client information| 
| 192 | b_client_permissionoverview_view | Retrieve client permissions overview| 
| 193 | b_client_permissionoverview_own | Retrieve clients own permissions overview| 
| 194 | b_client_remoteaddress_view | View client IP address and port| 
| 195 | i_client_serverquery_view_power | ServerQuery view power| 
| 196 | i_client_needed_serverquery_view_power | Needed ServerQuery view power| 
| 197 | b_client_custom_info_view | View custom fields| 
| 198 | i_client_kick_from_server_power | Client kick power from server| 
| 199 | i_client_needed_kick_from_server_power | Needed client kick power from server| 
| 200 | i_client_kick_from_channel_power | Client kick power from channel| 
| 201 | i_client_needed_kick_from_channel_power | Needed client kick power from channel| 
| 202 | i_client_ban_power | Client ban power| 
| 203 | i_client_needed_ban_power | Needed client ban power| 
| 204 | i_client_move_power | Client move power| 
| 205 | i_client_needed_move_power | Needed client move power| 
| 206 | i_client_complain_power | Complain power| 
| 207 | i_client_needed_complain_power | Needed complain power| 
| 208 | b_client_complain_list | Show complain list| 
| 209 | b_client_complain_delete_own | Delete own complains| 
| 210 | b_client_complain_delete | Delete complains| 
| 211 | b_client_ban_list | Show banlist| 
| 212 | b_client_ban_create | Add a ban| 
| 213 | b_client_ban_delete_own | Delete own bans| 
| 214 | b_client_ban_delete | Delete bans| 
| 215 | i_client_ban_max_bantime | Max bantime| 
| 216 | i_client_private_textmessage_power | Client private message power| 
| 217 | i_client_needed_private_textmessage_power | Needed client private message power| 
| 218 | b_client_server_textmessage_send | Send text messages to virtual server| 
| 219 | b_client_channel_textmessage_send | Send text messages to channel| 
| 220 | b_client_offline_textmessage_send | Send offline messages to clients| 
| 221 | i_client_talk_power | Client talk power| 
| 222 | i_client_needed_talk_power | Needed client talk power| 
| 223 | i_client_poke_power | Client poke power| 
| 224 | i_client_needed_poke_power | Needed client poke power| 
| 225 | b_client_set_flag_talker | Set the talker flag for clients and allow them to speak| 
| 226 | i_client_whisper_power | Client whisper power| 
| 227 | i_client_needed_whisper_power | Client needed whisper power| 
| 228 | b_client_modify_description | Edit a clients description| 
| 229 | b_client_modify_own_description | Allow client to edit own description| 
| 230 | b_client_modify_dbproperties | Edit a clients properties in the database| 
| 231 | b_client_delete_dbproperties | Delete a clients properties in the database| 
| 232 | b_client_create_modify_serverquery_login | Create or modify own ServerQuery account| 
| 233 | b_ft_ignore_password | Browse files without channel password| 
| 234 | b_ft_transfer_list | Retrieve list of running filetransfers| 
| 235 | i_ft_file_upload_power | File upload power| 
| 236 | i_ft_needed_file_upload_power | Needed file upload power| 
| 237 | i_ft_file_download_power | File download power| 
| 238 | i_ft_needed_file_download_power | Needed file download power| 
| 239 | i_ft_file_delete_power | File delete power| 
| 240 | i_ft_needed_file_delete_power | Needed file delete power| 
| 241 | i_ft_file_rename_power | File rename power| 
| 242 | i_ft_needed_file_rename_power | Needed file rename power| 
| 243 | i_ft_file_browse_power | File browse power| 
| 244 | i_ft_needed_file_browse_power | Needed file browse power| 
| 245 | i_ft_directory_create_power | Create directory power| 
| 246 | i_ft_needed_directory_create_power | Needed create directory power| 
| 247 | i_ft_quota_mb_download_per_client | Download quota per client in MByte| 
| 248 | i_ft_quota_mb_upload_per_client | Upload quota per client in MByte| 
| 32769 | i_needed_modify_power_serverinstance_help_view | | 
| 32770 | i_needed_modify_power_serverinstance_info_view | | 
| 32771 | i_needed_modify_power_serverinstance_virtualserver_list | | 
| 32772 | i_needed_modify_power_serverinstance_binding_list | | 
| 32773 | i_needed_modify_power_serverinstance_permission_list | | 
| 32774 | i_needed_modify_power_serverinstance_permission_find | | 
| 32775 | i_needed_modify_power_virtualserver_create | | 
| 32776 | i_needed_modify_power_virtualserver_delete | | 
| 32777 | i_needed_modify_power_virtualserver_start_any | | 
| 32778 | i_needed_modify_power_virtualserver_stop_any | | 
| 32779 | i_needed_modify_power_virtualserver_change_machine_id | | 
| 32780 | i_needed_modify_power_virtualserver_change_template | | 
| 32781 | i_needed_modify_power_serverquery_login | | 
| 32782 | i_needed_modify_power_serverquery_login_create | | 
| 32783 | i_needed_modify_power_serverquery_login_delete | | 
| 32784 | i_needed_modify_power_serverquery_login_list | | 
| 32785 | i_needed_modify_power_serverinstance_textmessage_send | | 
| 32786 | i_needed_modify_power_serverinstance_log_view | | 
| 32787 | i_needed_modify_power_serverinstance_log_add | | 
| 32788 | i_needed_modify_power_serverinstance_stop | | 
| 32789 | i_needed_modify_power_serverinstance_modify_settings | | 
| 32790 | i_needed_modify_power_serverinstance_modify_querygroup | | 
| 32791 | i_needed_modify_power_serverinstance_modify_templates | | 
| 32792 | i_needed_modify_power_virtualserver_select | | 
| 32793 | i_needed_modify_power_virtualserver_info_view | | 
| 32794 | i_needed_modify_power_virtualserver_connectioninfo_view | | 
| 32795 | i_needed_modify_power_virtualserver_channel_list | | 
| 32796 | i_needed_modify_power_virtualserver_channel_search | | 
| 32797 | i_needed_modify_power_virtualserver_client_list | | 
| 32798 | i_needed_modify_power_virtualserver_client_search | | 
| 32799 | i_needed_modify_power_virtualserver_client_dblist | | 
| 32800 | i_needed_modify_power_virtualserver_client_dbsearch | | 
| 32801 | i_needed_modify_power_virtualserver_client_dbinfo | | 
| 32802 | i_needed_modify_power_virtualserver_permission_find | | 
| 32803 | i_needed_modify_power_virtualserver_custom_search | | 
| 32804 | i_needed_modify_power_virtualserver_start | | 
| 32805 | i_needed_modify_power_virtualserver_stop | | 
| 32806 | i_needed_modify_power_virtualserver_token_list | | 
| 32807 | i_needed_modify_power_virtualserver_token_add | | 
| 32808 | i_needed_modify_power_virtualserver_token_use | | 
| 32809 | i_needed_modify_power_virtualserver_token_delete | | 
| 32810 | i_needed_modify_power_virtualserver_apikey_add | | 
| 32811 | i_needed_modify_power_virtualserver_apikey_manage | | 
| 32812 | i_needed_modify_power_virtualserver_log_view | | 
| 32813 | i_needed_modify_power_virtualserver_log_add | | 
| 32814 | i_needed_modify_power_virtualserver_join_ignore_password | | 
| 32815 | i_needed_modify_power_virtualserver_notify_register | | 
| 32816 | i_needed_modify_power_virtualserver_notify_unregister | | 
| 32817 | i_needed_modify_power_virtualserver_snapshot_create | | 
| 32818 | i_needed_modify_power_virtualserver_snapshot_deploy | | 
| 32819 | i_needed_modify_power_virtualserver_permission_reset | | 
| 32820 | i_needed_modify_power_virtualserver_modify_name | | 
| 32821 | i_needed_modify_power_virtualserver_modify_welcomemessage | | 
| 32822 | i_needed_modify_power_virtualserver_modify_maxclients | | 
| 32823 | i_needed_modify_power_virtualserver_modify_reserved_slots | | 
| 32824 | i_needed_modify_power_virtualserver_modify_password | | 
| 32825 | i_needed_modify_power_virtualserver_modify_default_servergroup | | 
| 32826 | i_needed_modify_power_virtualserver_modify_default_channelgroup | | 
| 32827 | i_needed_modify_power_virtualserver_modify_default_channeladmingroup | | 
| 32828 | i_needed_modify_power_virtualserver_modify_channel_forced_silence | | 
| 32829 | i_needed_modify_power_virtualserver_modify_complain | | 
| 32830 | i_needed_modify_power_virtualserver_modify_antiflood | | 
| 32831 | i_needed_modify_power_virtualserver_modify_ft_settings | | 
| 32832 | i_needed_modify_power_virtualserver_modify_ft_quotas | | 
| 32833 | i_needed_modify_power_virtualserver_modify_hostmessage | | 
| 32834 | i_needed_modify_power_virtualserver_modify_hostbanner | | 
| 32835 | i_needed_modify_power_virtualserver_modify_hostbutton | | 
| 32836 | i_needed_modify_power_virtualserver_modify_port | | 
| 32837 | i_needed_modify_power_virtualserver_modify_autostart | | 
| 32838 | i_needed_modify_power_virtualserver_modify_needed_identity_security_level | | 
| 32839 | i_needed_modify_power_virtualserver_modify_priority_speaker_dimm_modificator | | 
| 32840 | i_needed_modify_power_virtualserver_modify_log_settings | | 
| 32841 | i_needed_modify_power_virtualserver_modify_min_client_version | | 
| 32842 | i_needed_modify_power_virtualserver_modify_icon_id | | 
| 32843 | i_needed_modify_power_virtualserver_modify_weblist | | 
| 32844 | i_needed_modify_power_virtualserver_modify_codec_encryption_mode | | 
| 32845 | i_needed_modify_power_virtualserver_modify_temporary_passwords | | 
| 32846 | i_needed_modify_power_virtualserver_modify_temporary_passwords_own | | 
| 32847 | i_needed_modify_power_virtualserver_modify_channel_temp_delete_delay_default | | 
| 32848 | i_needed_modify_power_virtualserver_modify_nickname | | 
| 32849 | i_needed_modify_power_virtualserver_modify_integrations | | 
| 32850 | i_needed_modify_power_channel_min_depth | | 
| 32851 | i_needed_modify_power_channel_max_depth | | 
| 32852 | i_needed_modify_power_channel_group_inheritance_end | | 
| 32853 | i_needed_modify_power_channel_permission_modify_power | | 
| 32854 | i_needed_modify_power_channel_needed_permission_modify_power | | 
| 32855 | i_needed_modify_power_channel_info_view | | 
| 32856 | i_needed_modify_power_channel_create_child | | 
| 32857 | i_needed_modify_power_channel_create_permanent | | 
| 32858 | i_needed_modify_power_channel_create_semi_permanent | | 
| 32859 | i_needed_modify_power_channel_create_temporary | | 
| 32860 | i_needed_modify_power_channel_create_with_topic | | 
| 32861 | i_needed_modify_power_channel_create_with_description | | 
| 32862 | i_needed_modify_power_channel_create_with_password | | 
| 32863 | i_needed_modify_power_channel_create_with_banner | | 
| 32864 | i_needed_modify_power_channel_create_modify_with_codec_opusvoice | | 
| 32865 | i_needed_modify_power_channel_create_modify_with_codec_opusmusic | | 
| 32866 | i_needed_modify_power_channel_create_modify_with_codec_maxquality | | 
| 32867 | i_needed_modify_power_channel_create_modify_with_codec_latency_factor_min | | 
| 32868 | i_needed_modify_power_channel_create_with_maxclients | | 
| 32869 | i_needed_modify_power_channel_create_with_maxfamilyclients | | 
| 32870 | i_needed_modify_power_channel_create_with_sortorder | | 
| 32871 | i_needed_modify_power_channel_create_with_default | | 
| 32872 | i_needed_modify_power_channel_create_with_needed_talk_power | | 
| 32873 | i_needed_modify_power_channel_create_modify_with_force_password | | 
| 32874 | i_needed_modify_power_channel_create_modify_with_temp_delete_delay | | 
| 32875 | i_needed_modify_power_channel_modify_parent | | 
| 32876 | i_needed_modify_power_channel_modify_make_default | | 
| 32877 | i_needed_modify_power_channel_modify_make_permanent | | 
| 32878 | i_needed_modify_power_channel_modify_make_semi_permanent | | 
| 32879 | i_needed_modify_power_channel_modify_make_temporary | | 
| 32880 | i_needed_modify_power_channel_modify_name | | 
| 32881 | i_needed_modify_power_channel_modify_topic | | 
| 32882 | i_needed_modify_power_channel_modify_description | | 
| 32883 | i_needed_modify_power_channel_modify_password | | 
| 32884 | i_needed_modify_power_channel_modify_banner | | 
| 32885 | i_needed_modify_power_channel_modify_codec | | 
| 32886 | i_needed_modify_power_channel_modify_codec_quality | | 
| 32887 | i_needed_modify_power_channel_modify_codec_latency_factor | | 
| 32888 | i_needed_modify_power_channel_modify_maxclients | | 
| 32889 | i_needed_modify_power_channel_modify_maxfamilyclients | | 
| 32890 | i_needed_modify_power_channel_modify_sortorder | | 
| 32891 | i_needed_modify_power_channel_modify_needed_talk_power | | 
| 32892 | i_needed_modify_power_channel_modify_power | | 
| 32893 | i_needed_modify_power_channel_needed_modify_power | | 
| 32894 | i_needed_modify_power_channel_modify_make_codec_encrypted | | 
| 32895 | i_needed_modify_power_channel_modify_temp_delete_delay | | 
| 32896 | i_needed_modify_power_channel_delete_permanent | | 
| 32897 | i_needed_modify_power_channel_delete_semi_permanent | | 
| 32898 | i_needed_modify_power_channel_delete_temporary | | 
| 32899 | i_needed_modify_power_channel_delete_flag_force | | 
| 32900 | i_needed_modify_power_channel_delete_power | | 
| 32901 | i_needed_modify_power_channel_needed_delete_power | | 
| 32902 | i_needed_modify_power_channel_join_permanent | | 
| 32903 | i_needed_modify_power_channel_join_semi_permanent | | 
| 32904 | i_needed_modify_power_channel_join_temporary | | 
| 32905 | i_needed_modify_power_channel_join_ignore_password | | 
| 32906 | i_needed_modify_power_channel_join_ignore_maxclients | | 
| 32907 | i_needed_modify_power_channel_join_power | | 
| 32908 | i_needed_modify_power_channel_needed_join_power | | 
| 32909 | i_needed_modify_power_channel_subscribe_power | | 
| 32910 | i_needed_modify_power_channel_needed_subscribe_power | | 
| 32911 | i_needed_modify_power_channel_description_view_power | | 
| 32912 | i_needed_modify_power_channel_needed_description_view_power | | 
| 32913 | i_needed_modify_power_icon_id | | 
| 32914 | i_needed_modify_power_max_icon_filesize | | 
| 32915 | i_needed_modify_power_icon_manage | | 
| 32916 | i_needed_modify_power_group_is_permanent | | 
| 32917 | i_needed_modify_power_group_auto_update_type | | 
| 32918 | i_needed_modify_power_group_auto_update_max_value | | 
| 32919 | i_needed_modify_power_group_sort_id | | 
| 32920 | i_needed_modify_power_group_show_name_in_tree | | 
| 32921 | i_needed_modify_power_virtualserver_servergroup_list | | 
| 32922 | i_needed_modify_power_virtualserver_servergroup_permission_list | | 
| 32923 | i_needed_modify_power_virtualserver_servergroup_client_list | | 
| 32924 | i_needed_modify_power_virtualserver_channelgroup_list | | 
| 32925 | i_needed_modify_power_virtualserver_channelgroup_permission_list | | 
| 32926 | i_needed_modify_power_virtualserver_channelgroup_client_list | | 
| 32927 | i_needed_modify_power_virtualserver_client_permission_list | | 
| 32928 | i_needed_modify_power_virtualserver_channel_permission_list | | 
| 32929 | i_needed_modify_power_virtualserver_channelclient_permission_list | | 
| 32930 | i_needed_modify_power_virtualserver_servergroup_create | | 
| 32931 | i_needed_modify_power_virtualserver_channelgroup_create | | 
| 32932 | i_needed_modify_power_group_modify_power | | 
| 32933 | i_needed_modify_power_group_needed_modify_power | | 
| 32934 | i_needed_modify_power_group_member_add_power | | 
| 32935 | i_needed_modify_power_group_needed_member_add_power | | 
| 32936 | i_needed_modify_power_group_member_remove_power | | 
| 32937 | i_needed_modify_power_group_needed_member_remove_power | | 
| 32938 | i_needed_modify_power_permission_modify_power | | 
| 32939 | i_needed_modify_power_permission_modify_power_ignore | | 
| 32940 | i_needed_modify_power_virtualserver_servergroup_delete | | 
| 32941 | i_needed_modify_power_virtualserver_channelgroup_delete | | 
| 32942 | i_needed_modify_power_client_permission_modify_power | | 
| 32943 | i_needed_modify_power_client_needed_permission_modify_power | | 
| 32944 | i_needed_modify_power_client_max_clones_uid | | 
| 32945 | i_needed_modify_power_client_max_idletime | | 
| 32946 | i_needed_modify_power_client_max_avatar_filesize | | 
| 32947 | i_needed_modify_power_client_max_channel_subscriptions | | 
| 32948 | i_needed_modify_power_client_is_priority_speaker | | 
| 32949 | i_needed_modify_power_client_skip_channelgroup_permissions | | 
| 32950 | i_needed_modify_power_client_force_push_to_talk | | 
| 32951 | i_needed_modify_power_client_ignore_bans | | 
| 32952 | i_needed_modify_power_client_ignore_antiflood | | 
| 32953 | i_needed_modify_power_client_use_reserved_slot | | 
| 32954 | i_needed_modify_power_client_use_channel_commander | | 
| 32955 | i_needed_modify_power_client_request_talker | | 
| 32956 | i_needed_modify_power_client_avatar_delete_other | | 
| 32957 | i_needed_modify_power_client_is_sticky | | 
| 32958 | i_needed_modify_power_client_ignore_sticky | | 
| 32959 | i_needed_modify_power_client_info_view | | 
| 32960 | i_needed_modify_power_client_permissionoverview_view | | 
| 32961 | i_needed_modify_power_client_permissionoverview_own | | 
| 32962 | i_needed_modify_power_client_remoteaddress_view | | 
| 32963 | i_needed_modify_power_client_serverquery_view_power | | 
| 32964 | i_needed_modify_power_client_needed_serverquery_view_power | | 
| 32965 | i_needed_modify_power_client_custom_info_view | | 
| 32966 | i_needed_modify_power_client_kick_from_server_power | | 
| 32967 | i_needed_modify_power_client_needed_kick_from_server_power | | 
| 32968 | i_needed_modify_power_client_kick_from_channel_power | | 
| 32969 | i_needed_modify_power_client_needed_kick_from_channel_power | | 
| 32970 | i_needed_modify_power_client_ban_power | | 
| 32971 | i_needed_modify_power_client_needed_ban_power | | 
| 32972 | i_needed_modify_power_client_move_power | | 
| 32973 | i_needed_modify_power_client_needed_move_power | | 
| 32974 | i_needed_modify_power_client_complain_power | | 
| 32975 | i_needed_modify_power_client_needed_complain_power | | 
| 32976 | i_needed_modify_power_client_complain_list | | 
| 32977 | i_needed_modify_power_client_complain_delete_own | | 
| 32978 | i_needed_modify_power_client_complain_delete | | 
| 32979 | i_needed_modify_power_client_ban_list | | 
| 32980 | i_needed_modify_power_client_ban_create | | 
| 32981 | i_needed_modify_power_client_ban_delete_own | | 
| 32982 | i_needed_modify_power_client_ban_delete | | 
| 32983 | i_needed_modify_power_client_ban_max_bantime | | 
| 32984 | i_needed_modify_power_client_private_textmessage_power | | 
| 32985 | i_needed_modify_power_client_needed_private_textmessage_power | | 
| 32986 | i_needed_modify_power_client_server_textmessage_send | | 
| 32987 | i_needed_modify_power_client_channel_textmessage_send | | 
| 32988 | i_needed_modify_power_client_offline_textmessage_send | | 
| 32989 | i_needed_modify_power_client_talk_power | | 
| 32990 | i_needed_modify_power_client_needed_talk_power | | 
| 32991 | i_needed_modify_power_client_poke_power | | 
| 32992 | i_needed_modify_power_client_needed_poke_power | | 
| 32993 | i_needed_modify_power_client_set_flag_talker | | 
| 32994 | i_needed_modify_power_client_whisper_power | | 
| 32995 | i_needed_modify_power_client_needed_whisper_power | | 
| 32996 | i_needed_modify_power_client_modify_description | | 
| 32997 | i_needed_modify_power_client_modify_own_description | | 
| 32998 | i_needed_modify_power_client_modify_dbproperties | | 
| 32999 | i_needed_modify_power_client_delete_dbproperties | | 
| 33000 | i_needed_modify_power_client_create_modify_serverquery_login | | 
| 33001 | i_needed_modify_power_ft_ignore_password | | 
| 33002 | i_needed_modify_power_ft_transfer_list | | 
| 33003 | i_needed_modify_power_ft_file_upload_power | | 
| 33004 | i_needed_modify_power_ft_needed_file_upload_power | | 
| 33005 | i_needed_modify_power_ft_file_download_power | | 
| 33006 | i_needed_modify_power_ft_needed_file_download_power | | 
| 33007 | i_needed_modify_power_ft_file_delete_power | | 
| 33008 | i_needed_modify_power_ft_needed_file_delete_power | | 
| 33009 | i_needed_modify_power_ft_file_rename_power | | 
| 33010 | i_needed_modify_power_ft_needed_file_rename_power | | 
| 33011 | i_needed_modify_power_ft_file_browse_power | | 
| 33012 | i_needed_modify_power_ft_needed_file_browse_power | | 
| 33013 | i_needed_modify_power_ft_directory_create_power | | 
| 33014 | i_needed_modify_power_ft_needed_directory_create_power | | 
| 33015 | i_needed_modify_power_ft_quota_mb_download_per_client | | 
| 33016 | i_needed_modify_power_ft_quota_mb_upload_per_client | | 