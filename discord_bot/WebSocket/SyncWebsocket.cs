using System.Reactive.Linq;
using DSharpPlus.Entities;
using Newtonsoft.Json;
using Serilog;
using Websocket.Client;

namespace KekovBot
{
    public class SyncWebsocket : WebsocketController
    {
        public static Dictionary<DiscordGuild, GuildVoiceChannels> TrackedGuilds = new Dictionary<DiscordGuild, GuildVoiceChannels>();
        public SyncWebsocket(String uri) : base(uri)
        {
            SetupClientEvents();
            StartClient();
        }

        private void SetupClientEvents()
        {
            _client.DisconnectionHappened.Subscribe(info =>
            {
                Log.Warning($"Websocket disconnection happaned, type: {info.Type}");
            });
            _client.ReconnectionHappened.Subscribe(info =>
            {
                TrackedGuilds.Clear();
                Log.Warning($"Websocket reconnection happaned, type: {info.Type}");
            });
            _client.MessageReceived.Subscribe(msg => HandleMessage(msg));
        }

        private void HandleMessage(ResponseMessage msg)
        {
            Log.Debug($"Sync Message: {msg}");
            try
            {
                SyncMessage? syncMessage = JsonConvert.DeserializeObject<SyncMessage>(msg.Text);
                switch (syncMessage?.OpCode)
                {
                    case SyncOpCode.AddGuild:
                        {
                            var guildVoiceChannels = AddGuild(syncMessage.GuildId ?? 0);
                            var response = new SyncMessage(guildVoiceChannels, syncMessage.GuildId ?? 0);
                            var responseJson = JsonConvert.SerializeObject(response);
                            Console.WriteLine(responseJson);
                            _client.Send(responseJson);
                            break;
                        }
                    case SyncOpCode.RemoveGuild:
                        {
                            RemoveGuild(syncMessage.GuildId ?? 0);
                            break;
                        }
                    default:
                        {
                            for (var i = 0; i < 10; i++)
                                Log.Error("IMPLEMENT MISSING OP CODES!");
                            System.Environment.Exit(1);
                            break;
                        }
                }
            }
            catch (WebSocketException e)
            {
                Log.Error(e.ToString());
            }
            catch (Exception e)
            {
                Log.Error(e.ToString());
            }
        }

        private GuildVoiceChannels AddGuild(ulong guildId)
        {
            DiscordGuild? guild;
            DiscordBot.Instance.DiscordClient.Guilds.TryGetValue(guildId, out guild);
            if (guild == null)
            {
                throw new GuildNotFoundException();
            }
            var guildVoiceChannels = new GuildVoiceChannels(guild);
            TrackedGuilds.Add(guild, guildVoiceChannels);
            return guildVoiceChannels;
        }

        private void RemoveGuild(ulong guildId)
        {
            DiscordGuild? guild;
            DiscordBot.Instance.DiscordClient.Guilds.TryGetValue(guildId, out guild);
            if (guild == null)
            {
                throw new GuildNotFoundException();
            }
            TrackedGuilds.Remove(guild);
        }
    }
}
