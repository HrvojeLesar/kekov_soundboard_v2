using DSharpPlus.Lavalink;
using KekovBot.Exceptions;

namespace KekovBot.Control
{
    public class PlayQueue
    {
        private static int _queue_limit = 10;
        public Sound? CurrentlyPlaying { get; set; }
        public LavalinkGuildConnection GuildConnection { get; set; }

        private Queue<Sound> _queue { get; set; }

        public PlayQueue(LavalinkGuildConnection guildConnection)
        {
            CurrentlyPlaying = null;
            _queue = new Queue<Sound>();
            GuildConnection = guildConnection;
        }

        public async Task UnconditionalStart(Sound startSound)
        {
            try {
                CurrentlyPlaying = startSound;
                _queue.Clear();
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
            if (_queue.Count > 0)
            {
                CurrentlyPlaying = _queue.Dequeue();
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
                    queue.AddRange(_queue.ToList());
                }
                return queue;
            });
        }

        public void Enqueue(Sound sound) {
            if (_queue.Count < _queue_limit) {
                _queue.Enqueue(sound);
            } else {
                throw new QueueFullException();
            }
        }
    }
}
