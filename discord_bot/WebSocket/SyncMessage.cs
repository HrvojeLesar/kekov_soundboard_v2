using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace KekovBot
{
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

        public SyncMessage() { }

        public SyncMessage(SyncOpCode opCode, Nullable<ulong> userId, Nullable<ulong> guildId)
        {
            OpCode = opCode;
            UserId = userId;
            GuildId = guildId;
        }

        public SyncMessage(SyncOpCode code, SyncMessage other)
        {
            OpCode = code;
            UserId = other.UserId;
        }
    }
}
