using dotenv.net;
using DSharpPlus;
using DSharpPlus.Net;
using DSharpPlus.Lavalink;
using Microsoft.Extensions.Logging;
using Serilog;
using KekovBot.WebSocket;

namespace KekovBot.Bot
{
    public partial class DiscordBot
    {
        public DiscordClient DiscordClient { get; }

        private ControlsWebsocket ControlsWebsocket { get; set; }

        private SyncWebsocket SyncWebsocket { get; set; }

        private static DiscordBot? _instance;

        public static DiscordBot Instance
        {
            get { return _instance ?? (_instance = new DiscordBot()); }
        }

        private DiscordBot()
        {
            var env = DotEnv.Read();
            ControlsWebsocket = new ControlsWebsocket(env["WS_CONTROLS_URL"]);
            SyncWebsocket = new SyncWebsocket(env["WS_SYNC_URL"]);

            Log.Logger = new LoggerConfiguration()
                .MinimumLevel.Debug()
                .WriteTo.Console()
                .CreateLogger();

            var loggerFactory = new LoggerFactory().AddSerilog();

            DiscordClient = new DiscordClient(new DiscordConfiguration()
            {
                LoggerFactory = loggerFactory,
                Token = env["DISCORD_BOT_TOKEN"],
                TokenType = TokenType.Bot,
                Intents = DiscordIntents.GuildMembers
                | DiscordIntents.GuildVoiceStates
                | DiscordIntents.Guilds
            });
            RegisterEventHandlers();
            DiscordClient.ConnectAsync().Wait();
            InitLavalink(); // Should always be initialized after client connection
        }

        private void RegisterEventHandlers()
        {
            DiscordClient.GuildMemberAdded += GuildMemberAddedEvent;
            DiscordClient.GuildMemberRemoved += GuildMemberRemovedEvent;
            DiscordClient.GuildCreated += BotJoinedGuildEvent;
            DiscordClient.GuildDeleted += BotLeftGuildEvent;

            DiscordClient.ChannelCreated += ChannelCreatedEvent;
            DiscordClient.ChannelDeleted += ChannelDeletedEvent;
            DiscordClient.ChannelUpdated += ChannelUpdatedEvent;
            DiscordClient.VoiceStateUpdated += VoiceStateUpdatedEvent;
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
