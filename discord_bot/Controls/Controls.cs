using DSharpPlus;
using DSharpPlus.Entities;
using DSharpPlus.Lavalink;

namespace KekovBot
{
    public static class Controls
    {
        private static DiscordBot _client = DiscordBot.GetInstance();

        private static async Task PlayTrack(DiscordChannel channel, FileInfo file)
        {
            var lava = _client.DiscordClient.GetLavalink();
            if (!lava.ConnectedNodes.Any())
            {
                throw new Exception("Lavalink connection is not established");
            }

            var node = lava.GetIdealNodeConnection();

            if (channel.Type != ChannelType.Voice)
            {
                throw new Exception("Not a valid voice channel");
            }

            LavalinkGuildConnection conn = await node.ConnectAsync(channel);
            LavalinkTrack track = await node.GetTrack(file);
            await conn.PlayAsync(track);
        }

        private static async Task<LavalinkTrack> GetTrack(this LavalinkNodeConnection node, FileInfo file)
        {
            var loadResult = await node.Rest.GetTracksAsync(file);
            if (loadResult.LoadResultType == LavalinkLoadResultType.LoadFailed || loadResult.LoadResultType == LavalinkLoadResultType.NoMatches)
            {
                throw new Exception("Failed to load file");
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
                    throw new ChannelsEmptyException();
                }

                if (voiceChannel != null)
                {
                    var file = new FileInfo(@"./cj.wav");
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
