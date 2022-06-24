using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace KekovBot
{
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class SyncMessage
    {
        [JsonProperty("op")]
        [JsonConverter(typeof(StringEnumConverter))]
        public SyncOpCode OpCode { get; set; }

        [JsonProperty("user_id")]
        [JsonConverter(typeof(ToStringConverter))]
        public ulong? UserId { get; set; }

        [JsonProperty("guild_id")]
        [JsonConverter(typeof(ToStringConverter))]
        public ulong? GuildId { get; set; }

        [JsonProperty("guild_voice_channels")]
        public GuildVoiceChannels? GuildVoiceChannels { get; set; }

        public SyncMessage() { }

        public SyncMessage(SyncOpCode opCode, Nullable<ulong> userId, Nullable<ulong> guildId)
        {
            OpCode = opCode;
            UserId = userId;
            GuildId = guildId;
        }

        public SyncMessage(GuildVoiceChannels guildVoiceChannels, ulong guildId)
        {
            OpCode = SyncOpCode.UpdateGuildChannels;
            GuildId = guildId;
            GuildVoiceChannels = guildVoiceChannels;
        }

        public SyncMessage(SyncOpCode code, SyncMessage other)
        {
            OpCode = code;
            UserId = other.UserId;
        }
    }
}
