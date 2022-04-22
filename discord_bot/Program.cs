using KekovBot;

MainAsync().GetAwaiter().GetResult();

static async Task MainAsync()
{
    var bot = DiscordBot.GetInstance();
    var ws = new WebSocket("ws://localhost:8080/v1/ws");
    await Task.Delay(-1);
}
