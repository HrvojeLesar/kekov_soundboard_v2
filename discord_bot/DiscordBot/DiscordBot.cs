using dotenv.net;
using DSharpPlus;
using DSharpPlus.Net;
using DSharpPlus.Lavalink;

namespace KekovBot
{
    public partial class DiscordBot
    {
        public DiscordClient DiscordClient { get; }
        private static DiscordBot? _instance;

        private DiscordBot()
        {
            var env = DotEnv.Read();
            DiscordClient = new DiscordClient(new DiscordConfiguration()
            {
                Token = env["DISCORD_BOT_TOKEN"],
                TokenType = TokenType.Bot
            });
            DiscordClient.ConnectAsync().Wait();
            InitLavalink(); // Should always be initialized after client connection
        }

        public static DiscordBot GetInstance()
        {
            if (_instance == null)
            {
                _instance = new DiscordBot();
            }
            return _instance;
        }

        private void InitLavalink()
        {
            var env = DotEnv.Read();
            var endpoint = new ConnectionEndpoint
            {
                Hostname = env["LAVALINK_HOSTNAME"],
                Port = int.Parse(env["LAVALINK_PORT"])
            };

            var lavalinkConfig = new LavalinkConfiguration
            {
                Password = env["LAVALINK_PASSWORD"],
                RestEndpoint = endpoint,
                SocketEndpoint = endpoint,
            };

            var lavalink = DiscordClient.UseLavalink();
            lavalink.ConnectAsync(lavalinkConfig).Wait();
        }
    }
}
