using KekovBot.Bot;

MainAsync().GetAwaiter().GetResult();

static async Task MainAsync()
{
    var bot = DiscordBot.Instance;
    await Task.Delay(-1);
}
