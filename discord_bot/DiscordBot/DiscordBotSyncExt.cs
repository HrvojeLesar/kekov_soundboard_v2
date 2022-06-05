using DSharpPlus;
using DSharpPlus.EventArgs;
using Newtonsoft.Json;
using Serilog;

namespace KekovBot
{
    public partial class DiscordBot
    {
        private Task GuildMemberAddedEvent(DiscordClient c, GuildMemberAddEventArgs args)
        {
            var response = new SyncMessage(SyncOpCode.UpdateUserCache, args.Member.Id, null);
            var response_json = JsonConvert.SerializeObject(response);
            SyncWebsocket.Client.Send(response_json);
            return Task.CompletedTask;
        }

        private Task GuildMemberRemovedEvent(DiscordClient c, GuildMemberRemoveEventArgs args)
        {
            var response = new SyncMessage(SyncOpCode.UpdateUserCache, args.Member.Id, null);
            var response_json = JsonConvert.SerializeObject(response);
            SyncWebsocket.Client.Send(response_json);
            return Task.CompletedTask;
        }

        private Task BotJoinedGuildEvent(DiscordClient c, GuildCreateEventArgs args)
        {
            Log.Warning($"Bot joined guild: {args.Guild.Id}");
            var response = new SyncMessage(SyncOpCode.InvalidateGuildsCache, null, args.Guild.Id);
            var response_json = JsonConvert.SerializeObject(response);
            SyncWebsocket.Client.Send(response_json);
            return Task.CompletedTask;
        }

        private Task BotLeftGuildEvent(DiscordClient c, GuildDeleteEventArgs args)
        {
            Log.Warning($"Bot left guild: {args.Guild.Id}");
            var response = new SyncMessage(SyncOpCode.InvalidateGuildsCache, null, args.Guild.Id);
            var response_json = JsonConvert.SerializeObject(response);
            SyncWebsocket.Client.Send(response_json);
            return Task.CompletedTask;
        }
    }
}
