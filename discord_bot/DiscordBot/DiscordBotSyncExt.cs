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
            var responseJson = JsonConvert.SerializeObject(response);
            SyncWebsocket.Client.Send(responseJson);
            return Task.CompletedTask;
        }

        private Task GuildMemberRemovedEvent(DiscordClient c, GuildMemberRemoveEventArgs args)
        {
            var response = new SyncMessage(SyncOpCode.UpdateUserCache, args.Member.Id, null);
            var responseJson = JsonConvert.SerializeObject(response);
            SyncWebsocket.Client.Send(responseJson);
            return Task.CompletedTask;
        }

        private Task BotJoinedGuildEvent(DiscordClient c, GuildCreateEventArgs args)
        {
            Log.Warning($"Bot joined guild: {args.Guild.Id}");
            SyncWebsocket.TrackedGuilds.Remove(args.Guild);
            var response = new SyncMessage(SyncOpCode.InvalidateGuildsCache, null, args.Guild.Id);
            var responseJson = JsonConvert.SerializeObject(response);
            SyncWebsocket.Client.Send(responseJson);
            return Task.CompletedTask;
        }

        private Task BotLeftGuildEvent(DiscordClient c, GuildDeleteEventArgs args)
        {
            Log.Warning($"Bot left guild: {args.Guild.Id}");
            var response = new SyncMessage(SyncOpCode.InvalidateGuildsCache, null, args.Guild.Id);
            var responseJson = JsonConvert.SerializeObject(response);
            SyncWebsocket.Client.Send(responseJson);
            return Task.CompletedTask;
        }

        private Task ChannelCreatedEvent(DiscordClient c, ChannelCreateEventArgs args)
        {
            try
            {
                Log.Debug("Channel created event fired!");
                if (!SyncWebsocket.TrackedGuilds.ContainsKey(args.Guild))
                {
                    return Task.CompletedTask;
                }
                var guildVoiceChannels = SyncWebsocket.TrackedGuilds[args.Guild];
                guildVoiceChannels.VoiceChannels.Add(args.Channel.Id, new CustomChannel(args.Channel));
                SendSyncMessage(guildVoiceChannels, args.Guild.Id);
            }
            catch (Exception e)
            {
                Log.Error(e.ToString());
            }
            return Task.CompletedTask;
        }

        private Task ChannelDeletedEvent(DiscordClient c, ChannelDeleteEventArgs args)
        {
            try
            {
                Log.Debug("Channel deleted event fired!");
                if (!SyncWebsocket.TrackedGuilds.ContainsKey(args.Guild))
                {
                    return Task.CompletedTask;
                }
                var guildVoiceChannels = SyncWebsocket.TrackedGuilds[args.Guild];
                guildVoiceChannels.VoiceChannels.Remove(args.Channel.Id);

                SendSyncMessage(guildVoiceChannels, args.Guild.Id);
            }
            catch (Exception e)
            {
                Log.Error(e.ToString());
            }
            return Task.CompletedTask;
        }

        private Task ChannelUpdatedEvent(DiscordClient c, ChannelUpdateEventArgs args)
        {
            try
            {
                Log.Debug("Channel updated event fired!");
                if (!SyncWebsocket.TrackedGuilds.ContainsKey(args.Guild))
                {
                    Log.Debug($"Guild [{args.Guild.Id}] is not tracked");
                    return Task.CompletedTask;
                }

                var guildVoiceChannels = SyncWebsocket.TrackedGuilds[args.Guild];
                var customChannel =
                    args.ChannelBefore != null
                    ? guildVoiceChannels.VoiceChannels[args.ChannelBefore.Id]
                    : throw new Exception("args.ChannelBefore is null");
                customChannel.Update(
                        args.ChannelAfter
                        ?? throw new Exception("args.ChannelAfter is null")
                    );
                SendSyncMessage(guildVoiceChannels, args.Guild.Id);
            }
            catch (Exception e)
            {
                Log.Error(e.ToString());
            }

            return Task.CompletedTask;
        }

        private Task VoiceStateUpdatedEvent(DiscordClient c, VoiceStateUpdateEventArgs args)
        {
            try
            {
                Log.Debug("Voice state updated event fired!");
                if (!SyncWebsocket.TrackedGuilds.ContainsKey(args.Guild))
                {
                    Log.Debug($"Guild [{args.Guild.Id}] is not tracked");
                    return Task.CompletedTask;
                }

                var guildVoiceChannels = SyncWebsocket.TrackedGuilds[args.Guild];

                // User connected
                if (args.Before == null && args.After.Channel.Type == ChannelType.Voice)
                {
                    var customChannel = guildVoiceChannels.VoiceChannels[args.After.Channel.Id];
                    customChannel.UpdateChannelMembers(args.After.Channel);
                }
                // User disconnected
                else if (args.After.Channel == null && args.Before?.Channel.Type == ChannelType.Voice)
                {
                    var customChannel = guildVoiceChannels.VoiceChannels[args.Before.Channel.Id];
                    customChannel.RemoveMember(args.Before.User.Id);
                }
                // User switched channels
                else
                {
                    var customChannelBefore =
                        args.Before?.Channel != null
                        ? guildVoiceChannels.VoiceChannels[args.Before.Channel.Id]
                        : throw new Exception("args.Before.Channel is null");
                    customChannelBefore.RemoveMember(args.Before.User.Id);

                    var customChannelAfter =
                        args.After.Channel != null
                        ? guildVoiceChannels.VoiceChannels[args.After.Channel.Id]
                        : throw new Exception("args.After.Channel is null");
                    customChannelAfter.UpdateChannelMembers(args.After.Channel);
                }

                SendSyncMessage(guildVoiceChannels, args.Guild.Id);
            }
            catch (Exception e)
            {
                Log.Error(e.ToString());
            }

            return Task.CompletedTask;
        }

        private void SendSyncMessage(GuildVoiceChannels guildVoiceChannels, ulong guildId)
        {
            var response = new SyncMessage(guildVoiceChannels, guildId);
            var responseJson = JsonConvert.SerializeObject(response);
            Log.Debug(responseJson);
            SyncWebsocket.Client.Send(responseJson);
        }
    }
}
