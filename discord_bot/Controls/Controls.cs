using DSharpPlus;
using DSharpPlus.Entities;
using DSharpPlus.Lavalink;
using KekovBot.Bot;
using KekovBot.Exceptions;
using KekovBot.WebSocket;

namespace KekovBot.Control
{
    public static class Controls
    {
        private static DiscordBot _client = DiscordBot.Instance;
        private static LavalinkExtension _lavalink = _client.DiscordClient.GetLavalink();
        public static Dictionary<DiscordGuild, PlayQueue> PlayQueueDict = new Dictionary<DiscordGuild, PlayQueue>();
        public static Dictionary<DiscordGuild, CancellationTokenSource> CancelationTokenDict = new Dictionary<DiscordGuild, CancellationTokenSource>();
        public static HashSet<DiscordGuild> AwaitingDisconnectDict = new HashSet<DiscordGuild>();

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
            }

            if (!PlayQueueDict.ContainsKey(guild))
            {
                var newPlayQueue = new PlayQueue(connection);
                PlayQueueDict.Add(guild, newPlayQueue);
                CancelationTokenDict.Add(guild, new CancellationTokenSource());
                connection.RegisterConnectionHandlers(newPlayQueue);
            }

            var playQueue = PlayQueueDict[guild];
            if (AwaitingDisconnectDict.Contains(guild))
            {
                var cancelToken = CancelationTokenDict[guild];
                cancelToken.Cancel();
                cancelToken.Dispose();
                CancelationTokenDict[guild] = new CancellationTokenSource();
            }

            if (playQueue.CurrentlyPlaying == null)
            {
                await playQueue.UnconditionalStart(sound);
                return false;
            }
            else
            {
                playQueue.Enqueue(sound);
                return true;
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

                    if (voiceChannel.Type != ChannelType.Voice)
                    {
                        throw new Exception();
                    }

                    if (voiceChannel.Users.Count == 0)
                    {
                        throw new ChannelsEmptyException();
                    }
                }
                catch (ChannelNotFoundException)
                {
                    throw;
                }
                catch (ChannelsEmptyException)
                {
                    throw;
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

            if (msg.FileId == null || msg.DisplayName == null)
            {
                throw new InvalidFileIdException();
            }

            var sound = new Sound((ulong)msg.FileId, msg.DisplayName);
            return await PlaySound(voiceChannel, sound);
        }

        public static async Task Stop(ControlMessage msg)
        {
            DiscordGuild guild = GetGuild(msg);
            PlayQueue? playQueue;
            PlayQueueDict.TryGetValue(guild, out playQueue);
            if (playQueue != null)
            {
                await playQueue.GuildConnection.Disconnect();
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
            if (playQueue != null && playQueue.CurrentlyPlaying != null)
            {
                await playQueue.GuildConnection.StopAsync();
            }
            else
            {
                throw new NotPlayingExpcetion();
            }
        }

        public static async Task<List<Sound>?> GetQueue(ControlMessage msg)
        {
            DiscordGuild guild = GetGuild(msg);
            PlayQueue? playQueue;
            PlayQueueDict.TryGetValue(guild, out playQueue);
            List<Sound>? queue = null;
            if (playQueue != null)
            {
                queue = await playQueue.GetQueueList();
            }
            return queue;
        }
    }
}
