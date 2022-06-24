using DSharpPlus;
using DSharpPlus.Entities;
using Newtonsoft.Json;

namespace KekovBot
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
            [JsonProperty("username")]
            private string _username;

            [JsonProperty("nickname")]
            private string _nickname;

            [JsonProperty("avatar_hash")]
            private string _avatarHash;

            [JsonIgnore]
            public DiscordMember UserObject;

            public User(DiscordMember memberObject)
            {
                _username = memberObject.Username;
                _nickname = memberObject.Nickname;
                _avatarHash = memberObject.AvatarHash;
                UserObject = memberObject;
            }
        }

        private DiscordChannel? _discordChannel;

        [JsonProperty("users")]
        private List<User> _users;

        [JsonProperty("channel_name")]
        private string _channelName { get; set; }

        [JsonIgnore]
        private DiscordChannel _channel { get; set; }

        public CustomChannel(DiscordChannel channel)
        {
            _channelName = channel.Name;
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

        public void RemoveMember(DiscordMember member)
        {
            _users.Remove(_users.Where(user => user.UserObject.Id == member.Id).First());
        }
    }
}
