// See https://aka.ms/new-console-template for more information
using KekovBot;

MainAsync().GetAwaiter().GetResult();

static async Task MainAsync()
{
    var bot = DiscordBot.GetInstance();
    bot.Start();
    await Task.Delay(-1);
}
