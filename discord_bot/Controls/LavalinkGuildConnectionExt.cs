using DSharpPlus.Entities;
using DSharpPlus.Lavalink;

namespace KekovBot
{
    public static class LavalinkGuildConnectionExt
    {
        private static Dictionary<DiscordGuild, PlayQueue> PlayQueueDict = Controls.PlayQueueDict;

        public static async Task<LavalinkTrack> GetTrack(this LavalinkGuildConnection conn, FileInfo file)
        {
            var loadResult = await conn.GetTracksAsync(file);
            if (loadResult.LoadResultType == LavalinkLoadResultType.LoadFailed || loadResult.LoadResultType == LavalinkLoadResultType.NoMatches)
            {
                throw new FileLoadingFailedException();
            }
            return loadResult.Tracks.First();
        }

        public static void RegisterConnectionHandlers(this LavalinkGuildConnection conn, PlayQueue playQueue)
        {
            var guild = conn.Guild;

            conn.DiscordWebSocketClosed += (gc, args) =>
            {
                PlayQueueDict.Remove(guild);
                Console.WriteLine("Websocket closed");
                return Task.CompletedTask;
            };

            conn.PlaybackFinished += async (gc, args) =>
            {
                if (!await playQueue.PlayNext())
                {
                    await conn.Disconnect(guild);
                }
            };

            conn.TrackException += async (gc, args) =>
            {
                if (!await playQueue.PlayNext())
                {
                    await conn.Disconnect(guild);
                }
                Console.WriteLine("Track exception");
            };

            conn.TrackStuck += async (gc, args) =>
            {
                if (!await playQueue.PlayNext())
                {
                    await conn.Disconnect(guild);
                }
                Console.WriteLine("Track stuck");
            };
        }

        public static async Task Disconnect(this LavalinkGuildConnection conn, DiscordGuild guild)
        {
            Console.WriteLine("Disconnect");
            PlayQueueDict.Remove(guild);
            await conn.DisconnectAsync();
        }
    }
}
