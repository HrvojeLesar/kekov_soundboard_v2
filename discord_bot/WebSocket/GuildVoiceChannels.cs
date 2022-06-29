using DSharpPlus;
using DSharpPlus.Entities;
using Newtonsoft.Json;

namespace KekovBot.WebSocket
{
    public class GuildVoiceChannels
    {
        [JsonProperty("channels")]
        public Dictionary<ulong, CustomChannel> VoiceChannels = new Dictionary<ulong, CustomChannel>();
        public GuildVoiceChannels(DiscordGuild guild)
        {
            foreach (var channel in guild.Channels.Values)
            {
                if (channel.Type == ChannelType.Voice)
                {
                    VoiceChannels.Add(channel.Id, new CustomChannel(channel));
                }
            }
        }
    }

    public class CustomChannel
    {
        private class User
        {
            [JsonProperty("id")]
            [JsonConverter(typeof(ToStringConverter))]
            private ulong _id { get; set; }

            [JsonProperty("username")]
            private string _username { get; set; }

            [JsonProperty("nickname")]
            private string _nickname { get; set; }

            [JsonProperty("avatar_hash")]
            private string _avatarHash { get; set; }

            [JsonProperty("discriminator")]
            private string _discriminator { get; set; }

            [JsonIgnore]
            public DiscordMember UserObject;

            public User(DiscordMember memberObject)
            {
                _id = memberObject.Id;
                _username = memberObject.Username;
                _nickname = memberObject.Nickname;
                _avatarHash = memberObject.AvatarHash;
                _discriminator = memberObject.Discriminator;
                UserObject = memberObject;
            }
        }

        private DiscordChannel? _discordChannel;

        [JsonProperty("users")]
        private List<User> _users { get; set; }

        [JsonProperty("channel_name")]
        private string _channelName { get; set; }

        [JsonProperty("id")]
        [JsonConverter(typeof(ToStringConverter))]
        private ulong _id { get; set; }

        [JsonIgnore]
        private DiscordChannel _channel { get; set; }

        public CustomChannel(DiscordChannel channel)
        {
            _channelName = channel.Name;
            _id = channel.Id;
            _channel = channel;
            _users = new List<User>();
            foreach (var user in channel.Users)
            {
                if (_users.Count <= 10)
                {
                    var newUser = new User(user);
                    _users.Add(newUser);
                }
                else
                {
                    break;
                }
            }
        }

        public static CustomChannel? GetCustomChannel(DiscordGuild guild, DiscordChannel discordChannel)
        {
            var trackedGuilds = SyncWebsocket.TrackedGuilds;
            if (!trackedGuilds.ContainsKey(guild))
            {
                return null;
            }
            if (discordChannel.Type != ChannelType.Voice)
            {
                return null;
            }

            var guildVoiceChannels = trackedGuilds[guild];
            CustomChannel? channel;

            if (!guildVoiceChannels.VoiceChannels.TryGetValue(discordChannel.Id, out channel))
            {
                return null;
            }
            return channel;
        }

        public void UpdateChannelMembers(DiscordChannel newChannelData)
        {
            if (newChannelData.Type != ChannelType.Voice)
            {
                return;
            }

            _discordChannel = newChannelData;
            _users.Clear();
            foreach (var user in newChannelData.Users)
            {
                if (_users.Count <= 10)
                {
                    var newUser = new User(user);
                    _users.Add(newUser);
                }
                else
                {
                    break;
                }
            }
        }

        public void Update(DiscordChannel channel)
        {
            _channelName = channel.Name;
            _channel = channel;
        }

        public void RemoveMember(ulong id)
        {
            _users.Remove(_users.Where(user => user.UserObject.Id == id).First());
        }
    }
}
