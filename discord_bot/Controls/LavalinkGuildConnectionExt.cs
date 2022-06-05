using DSharpPlus.Entities;
using DSharpPlus.Lavalink;
using Serilog;

namespace KekovBot
{
    public static class LavalinkGuildConnectionExt
    {
        private static Dictionary<DiscordGuild, PlayQueue> _playQueueDict = Controls.PlayQueueDict;
        private static Dictionary<DiscordGuild, CancellationTokenSource> _cancelationTokenDict = Controls.CancelationTokenDict;
        private static HashSet<DiscordGuild> _awaitingDisconnectDict = Controls.AwaitingDisconnectDict;

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
                _playQueueDict.Remove(guild);
                Log.Warning("Websocket closed");
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
                Log.Error("Track exception");
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
                Log.Error("Track stuck");
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

            var cancelToken = _cancelationTokenDict[guild];
            if (cancelToken == null)
            {
                Log.Warning("Cancel token is null!");
                return;
            }

            var task = Task.Run(async () =>
            {
                _awaitingDisconnectDict.Add(guild);
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
            _playQueueDict.Remove(guild);
            _cancelationTokenDict.Remove(guild);
            _awaitingDisconnectDict.Remove(guild);
        }
    }
}
