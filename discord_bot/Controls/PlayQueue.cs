using DSharpPlus.Lavalink;

namespace KekovBot
{
    public class PlayQueue
    {
        public Sound? CurrentlyPlaying { get; set; }
        public Queue<Sound> Queue { get; set; }
        public LavalinkGuildConnection GuildConnection { get; set; }

        public PlayQueue(LavalinkGuildConnection guildConnection)
        {
            CurrentlyPlaying = null;
            Queue = new Queue<Sound>();
            GuildConnection = guildConnection;
        }

        public async Task UnconditionalStart(Sound startSound)
        {
            try {
                CurrentlyPlaying = startSound;
                Queue.Clear();
                var track = await GuildConnection.GetTrack(CurrentlyPlaying.FileInfo);
                await GuildConnection.PlayAsync(track);
            } catch (FileLoadingFailedException e) {
                await GuildConnection.Disconnect();
                throw e;
            }
        }

        // Returns `true` when successfully playing next item `false` otherwise
        public async Task<bool> PlayNext()
        {
            if (Queue.Count > 0)
            {
                CurrentlyPlaying = Queue.Dequeue();
                var track = await GuildConnection.GetTrack(CurrentlyPlaying.FileInfo);
                await GuildConnection.PlayAsync(track);
                return true;
            }
            CurrentlyPlaying = null;
            return false;
        }

        public Task<List<Sound>> GetQueueList()
        {
            return Task.Run(() =>
            {
                List<Sound> queue = new List<Sound>();
                if (CurrentlyPlaying != null)
                {
                    queue.Add(CurrentlyPlaying);
                    queue.AddRange(Queue.ToList());
                }
                return queue;
            });
        }
    }
}
