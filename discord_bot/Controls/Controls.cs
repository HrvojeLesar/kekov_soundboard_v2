using DSharpPlus;
using DSharpPlus.Entities;
using DSharpPlus.Lavalink;

namespace KekovBot
{
    public static class Controls
    {
        private static DiscordBot _client = DiscordBot.Instance;
        private static LavalinkExtension _lavalink = _client.DiscordClient.GetLavalink();
        public static Dictionary<DiscordGuild, PlayQueue> PlayQueueDict = new Dictionary<DiscordGuild, PlayQueue>();

        private static DiscordGuild GetGuild(ControlMessage msg)
        {
            if (msg.GuildId == null)
            {
                throw new InvalidGuildIdException();
            }

            DiscordGuild? guild;
            _client.DiscordClient.Guilds.TryGetValue((ulong)msg.GuildId, out guild);

            if (guild == null)
            {
                throw new GuildNotFoundException();
            }

            return guild;
        }

        private static async Task<bool> PlaySound(DiscordChannel channel, Sound sound)
        {
            if (!_lavalink.ConnectedNodes.Any())
            {
                // TODO: Find out when this throws
                throw new LavalinkConnectionNotEstablishedException();
            }

            var node = _lavalink.GetIdealNodeConnection();

            if (channel.Type != ChannelType.Voice)
            {
                throw new InvalidVoiceChannelException();
            }

            var guild = channel.Guild;
            var connection = _lavalink.GetGuildConnection(guild);

            // Bot isn't currently connected
            if (connection == null)
            {
                connection = await node.ConnectAsync(channel);
                var playQueue = new PlayQueue(connection);
                PlayQueueDict.Add(guild, playQueue);
                connection.RegisterConnectionHandlers(playQueue);
            }

            try
            {
                var playQueue = PlayQueueDict[guild];

                if (playQueue.CurrentlyPlaying == null)
                {
                    playQueue.UnconditionalStart(sound);
                    return false;
                }
                else
                {
                    playQueue.Queue.Enqueue(sound);
                    return true;
                }
            }
            catch (Exception e)
            {
                Console.WriteLine(e);
                await connection.Disconnect(guild);
                return false;
            }
        }

        public static async Task<bool> Play(ControlMessage msg)
        {
            DiscordGuild guild = GetGuild(msg);

            DiscordChannel? voiceChannel = null;
            if (msg.VoiceChannelId != null)
            {
                try
                {
                    voiceChannel = guild.GetChannel((ulong)msg.VoiceChannelId);
                }
                catch (Exception)
                {
                    throw new ChannelNotFoundException();
                }
            }
            else
            {
                foreach (var channel in guild.Channels.Values)
                {
                    if (channel.Type == ChannelType.Voice && channel.Users.Count >= 1)
                    {
                        voiceChannel = channel;
                        break;
                    }
                }
                if (voiceChannel == null)
                {
                    throw new ChannelsEmptyException();
                }
            }

            if (voiceChannel == null)
            {
                throw new ChannelNotFoundException();
            }

            if (msg.FileId == null)
            {
                throw new InvalidFileIdException();
            }

            var sound = new Sound((ulong)msg.FileId);
            return await PlaySound(voiceChannel, sound);
        }

        public static async Task Stop(ControlMessage msg)
        {
            DiscordGuild guild = GetGuild(msg);
            PlayQueue? playQueue;
            PlayQueueDict.TryGetValue(guild, out playQueue);
            if (playQueue != null)
            {
                await playQueue.GuildConnection.Disconnect(guild);
            }
            else
            {
                throw new NotPlayingExpcetion();
            }
        }

        public static async Task Skip(ControlMessage msg)
        {
            DiscordGuild guild = GetGuild(msg);
            PlayQueue? playQueue;
            PlayQueueDict.TryGetValue(guild, out playQueue);
            if (playQueue != null)
            {
                await playQueue.GuildConnection.StopAsync();
            }
            else
            {
                throw new NotPlayingExpcetion();
            }
        }

        public static async Task<List<Sound>> GetQueue(ControlMessage msg)
        {
            DiscordGuild guild = GetGuild(msg);
            PlayQueue? playQueue;
            PlayQueueDict.TryGetValue(guild, out playQueue);
            List<Sound> queue = new List<Sound>();
            if (playQueue != null)
            {
                queue = await playQueue.GetQueueList();
            }
            return queue;
        }
    }
}
