using dotenv.net;
using DSharpPlus;

namespace KekovBot
{
    public class DiscordBot
    {
        public DiscordClient DiscordClient;
        private static DiscordBot? _instance;

        private DiscordBot()
        {
            var env = DotEnv.Read();
            DiscordClient = new DiscordClient(new DiscordConfiguration()
            {
                Token = env["DISCORD_BOT_TOKEN"],
                TokenType = TokenType.Bot
            });
        }

        public static DiscordBot GetInstance()
        {
            if (_instance == null)
            {
                _instance = new DiscordBot();
            }
            return _instance;
        }

        public async void Start() {
            await DiscordClient.ConnectAsync();
        }
    }
}
