using dotenv.net;
using DSharpPlus;
using DSharpPlus.Entities;
using DSharpPlus.Lavalink;

namespace KekovBot
{
    public static class Controls
    {
        private static DiscordBot _client = DiscordBot.Instance;

        private static async Task PlayTrack(DiscordChannel channel, FileInfo file)
        {
            var lava = _client.DiscordClient.GetLavalink();
            if (!lava.ConnectedNodes.Any())
            {
                throw new LavalinkConnectionNotEstablishedException();
            }

            var node = lava.GetIdealNodeConnection();

            if (channel.Type != ChannelType.Voice)
            {
                throw new InvalidVoiceChannelException();
            }

            LavalinkGuildConnection conn = await node.ConnectAsync(channel);
            LavalinkTrack track = await conn.GetTrack(file);
            await conn.PlayAsync(track);
        }

        private static async Task<LavalinkTrack> GetTrack(this LavalinkGuildConnection conn, FileInfo file)
        {
            var loadResult = await conn.GetTracksAsync(file);
            if (loadResult.LoadResultType == LavalinkLoadResultType.LoadFailed || loadResult.LoadResultType == LavalinkLoadResultType.NoMatches)
            {
                throw new FileLoadingFailedException();
            }
            return loadResult.Tracks.First();
        }

        public static async Task Play(ControlMessage msg)
        {
            DiscordGuild? guild;
            if (msg.GuildId != null)
            {
                _client.DiscordClient.Guilds.TryGetValue((ulong)msg.GuildId, out guild);
            }
            else
            {
                throw new InvalidGuildIdException();
            }

            if (guild != null)
            {
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

                if (voiceChannel != null)
                {
                    var env = DotEnv.Read();
                    var file = new FileInfo($"{env["SOUNDFILE_DIR"]}{msg.FileId}");
                    await PlayTrack(voiceChannel, file);
                }
                else
                {
                    throw new ChannelNotFoundException();
                }

            }
            else
            {
                throw new GuildNotFoundException();
            }
        }
    }
}
