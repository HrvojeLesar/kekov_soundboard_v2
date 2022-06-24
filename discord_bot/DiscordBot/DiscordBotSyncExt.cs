using System.Linq;
using DSharpPlus;
using DSharpPlus.Entities;
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

        private Task ChannelUpdatedEvent(DiscordClient c, ChannelUpdateEventArgs args)
        {
            Log.Debug("Channel updated event fired!");
            var channel = CustomChannel.GetCustomChannel(args.Guild, args.ChannelBefore);
            if (channel == null)
            {
                return Task.CompletedTask;
            }

            return Task.CompletedTask;
        }

        private Task VoiceStateUpdatedEvent(DiscordClient c, VoiceStateUpdateEventArgs args)
        {
            try
            {
                Log.Debug("Voice state updated event fired!");
                var channel = CustomChannel.GetCustomChannel(
                        args.Guild,
                        args.Channel != null
                            ? args.Channel
                            : args.Before.Channel
                        );
                if (channel == null)
                {
                    return Task.CompletedTask;
                }

                if (args.Channel != null)
                {
                    if (args.Before?.Channel != null)
                    {
                        var channelBefore = CustomChannel.GetCustomChannel(args.Guild, args.Before.Channel);
                        channelBefore?.RemoveMember(args.After.Member);
                    }
                    channel.UpdateChannelMembers(args.Channel);
                }
                else
                {
                    channel.RemoveMember(args.After.Member);
                }

                var guildVoiceChannels = SyncWebsocket.TrackedGuilds[args.Guild];
                var response = new SyncMessage(guildVoiceChannels, args.Guild.Id);
                var response_json = JsonConvert.SerializeObject(response);
                Console.WriteLine(response_json);
                SyncWebsocket.Client.Send(response_json);
            }
            catch (Exception e)
            {
                Log.Error(e.ToString());
            }

            return Task.CompletedTask;
        }

    }
}
