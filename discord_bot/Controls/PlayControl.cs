using DSharpPlus;
using DSharpPlus.Entities;
using DSharpPlus.Lavalink;

namespace KekovBot
{
    public static class PlayControl
    {
        private static DiscordBot _client = DiscordBot.Instance;
        private static LavalinkExtension _lavalink = _client.DiscordClient.GetLavalink();
        public static Dictionary<DiscordGuild, PlayQueue> PlayQueueDict = new Dictionary<DiscordGuild, PlayQueue>();

        private static async Task PlaySound(DiscordChannel channel, Sound sound)
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
                PlayQueueDict.Add(guild, new PlayQueue(connection));
                connection.RegisterConnectionHandlers();
                // LavalinkTrack track = await connection.GetTrack(file);
                // await connection.PlayAsync(track);
            }

            // TODO: CAN THROW
            var playQueue = PlayQueueDict[guild];
            // check if something is playing
            // if not directly play otherwise add to queue
            if (playQueue.CurrentlyPlaying == null)
            {
                playQueue.UnconditionalStart(sound);
                // not playing
                // start
            }
            else
            {
                playQueue.Queue.Enqueue(sound);
            }
        }

        public static async Task Play(ControlMessage msg)
        {
            DiscordGuild? guild;
            if (msg.GuildId == null)
            {
                throw new InvalidGuildIdException();
            }

            _client.DiscordClient.Guilds.TryGetValue((ulong)msg.GuildId, out guild);

            if (guild == null)
            {
                throw new GuildNotFoundException();
            }

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
            await PlaySound(voiceChannel, sound);
        }
    }
}
