using KekovBot;

MainAsync().GetAwaiter().GetResult();

static async Task MainAsync()
{
    var bot = DiscordBot.Instance;
    await Task.Delay(-1);
}
