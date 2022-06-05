using DSharpPlus.Entities;
using DSharpPlus.Lavalink;

namespace KekovBot
{
    public static class LavalinkGuildConnectionExt
    {
        private static Dictionary<DiscordGuild, PlayQueue> PlayQueueDict = Controls.PlayQueueDict;
        private static Dictionary<DiscordGuild, CancellationTokenSource> CancelationTokenDict = Controls.CancelationTokenDict;
        private static HashSet<DiscordGuild> AwaitingDisconnectDict = Controls.AwaitingDisconnectDict;

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
                    try
                    {
                        await conn.Disconnect(guild);
                    }
                    catch {}
                }
            };

            conn.TrackException += async (gc, args) =>
            {
                if (!await playQueue.PlayNext())
                {
                    try
                    {
                        await conn.Disconnect(guild);
                    }
                    catch {}
                }
                Console.WriteLine("Track exception");
            };

            conn.TrackStuck += async (gc, args) =>
            {
                if (!await playQueue.PlayNext())
                {
                    try
                    {
                        await conn.Disconnect(guild);
                    }
                    catch {}
                }
                Console.WriteLine("Track stuck");
            };
        }

        public static async Task Disconnect(this LavalinkGuildConnection conn, DiscordGuild guild, bool isStopCommand = false)
        {
            if (isStopCommand)
            {
                DisconnectCleanup(guild);
                await conn.DisconnectAsync();
                return;
            }

            var cancelToken = CancelationTokenDict[guild];
            if (cancelToken == null)
            {
                Console.WriteLine("Cancel token is null!");
                return;
            }

            var task = Task.Run(async () =>
            {
                AwaitingDisconnectDict.Add(guild);
                await Task.Delay(5000, cancelToken.Token);
                if (cancelToken.IsCancellationRequested)
                {
                    return;
                }
                DisconnectCleanup(guild);
                await conn.DisconnectAsync();
            }, cancelToken.Token);
            await task;
        }

        public static void DisconnectCleanup(DiscordGuild guild)
        {
            PlayQueueDict.Remove(guild);
            CancelationTokenDict.Remove(guild);
            AwaitingDisconnectDict.Remove(guild);
        }
    }
}
