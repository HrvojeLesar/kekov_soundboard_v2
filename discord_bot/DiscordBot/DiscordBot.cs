using dotenv.net;
using DSharpPlus;
using DSharpPlus.Net;
using DSharpPlus.Lavalink;
using DSharpPlus.EventArgs;
using Newtonsoft.Json;

namespace KekovBot
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
            ControlsWebsocket = new ControlsWebsocket("ws://localhost:8080/v1/ws/controls");
            SyncWebsocket = new SyncWebsocket("ws://localhost:8080/v1/ws/sync");

            var env = DotEnv.Read();
            DiscordClient = new DiscordClient(new DiscordConfiguration()
            {
                Token = env["DISCORD_BOT_TOKEN"],
                TokenType = TokenType.Bot,
                Intents = DiscordIntents.GuildMembers
                | DiscordIntents.GuildVoiceStates
            });
            RegisterEventHandlers();
            DiscordClient.ConnectAsync().Wait();
            InitLavalink(); // Should always be initialized after client connection
        }

        private void RegisterEventHandlers()
        {
            DiscordClient.GuildMemberAdded += GuildMemberAddedEvent;
            DiscordClient.GuildMemberRemoved += GuildMemberRemovedEvent;
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

        private Task GuildMemberAddedEvent(DiscordClient c, GuildMemberAddEventArgs args)
        {
            var response = new SyncMessage(args.Member.Id);
            var response_json = JsonConvert.SerializeObject(response);
            SyncWebsocket.Client.Send(response_json);
            return Task.CompletedTask;
        }

        private Task GuildMemberRemovedEvent(DiscordClient c, GuildMemberRemoveEventArgs args)
        {
            var response = new SyncMessage(args.Member.Id);
            var response_json = JsonConvert.SerializeObject(response);
            SyncWebsocket.Client.Send(response_json);
            return Task.CompletedTask;
        }
    }
}
