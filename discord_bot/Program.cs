using KekovBot;

MainAsync().GetAwaiter().GetResult();

static async Task MainAsync()
{
    var bot = DiscordBot.GetInstance();
    var ws = new ControlsWebsocket("ws://localhost:8080/v1/ws/controls");
    var ws2 = new SyncWebsocket("ws://localhost:8080/v1/ws/sync");
    await Task.Delay(-1);
}
