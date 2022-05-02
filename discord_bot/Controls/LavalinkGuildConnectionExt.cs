using DSharpPlus.Entities;
using DSharpPlus.Lavalink;

namespace KekovBot
{
    public static class LavalinkGuildConnectionExt
    {
        private static Dictionary<DiscordGuild, PlayQueue> PlayQueueDict = PlayControl.PlayQueueDict;

        public static async Task<LavalinkTrack> GetTrack(this LavalinkGuildConnection conn, FileInfo file)
        {
            var loadResult = await conn.GetTracksAsync(file);
            if (loadResult.LoadResultType == LavalinkLoadResultType.LoadFailed || loadResult.LoadResultType == LavalinkLoadResultType.NoMatches)
            {
                throw new FileLoadingFailedException();
            }
            return loadResult.Tracks.First();
        }

        public static void RegisterConnectionHandlers(this LavalinkGuildConnection conn)
        {
            var guild = conn.Guild;

            conn.DiscordWebSocketClosed += (gc, args) =>
            {
                // Clear queueu
                PlayQueueDict.Remove(guild);
                Console.WriteLine("Websocket closed");
                return Task.CompletedTask;
            };

            conn.PlaybackFinished += async (gc, args) =>
            {
                // If queueueueueu empty disconnect after x seconds
                // TODO: CAN THROW
                var playQueue = PlayQueueDict[guild];
                if (!await playQueue.PlayNext())
                {
                    // Do disconnect
                    PlayQueueDict.Remove(guild);
                    await conn.DisconnectAsync();
                }

                Console.WriteLine("Playback Finished");
                // return Task.CompletedTask;
            };

            conn.PlaybackStarted += (gc, args) =>
            {
                // for debug
                Console.WriteLine("Playback Started");
                return Task.CompletedTask;
            };

            conn.TrackException += async (gc, args) =>
            {
                // skip track
                // TODO: CAN THROW
                var playQueue = PlayQueueDict[guild];
                if (!await playQueue.PlayNext())
                {
                    // Do disconnect
                    PlayQueueDict.Remove(guild);
                    await conn.DisconnectAsync();
                }
                Console.WriteLine("Track exception");
            };

            conn.TrackStuck += async (gc, args) =>
            {
                // skip track
                // TODO: CAN THROW
                var playQueue = PlayQueueDict[guild];
                if (!await playQueue.PlayNext())
                {
                    // Do disconnect
                    PlayQueueDict.Remove(guild);
                    await conn.DisconnectAsync();
                }
                Console.WriteLine("Track stuck");
            };
        }
    }
}
