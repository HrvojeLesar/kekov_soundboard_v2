using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace KekovBot
{
    public class SyncMessage
    {
        [JsonProperty("op")]
        [JsonConverter(typeof(StringEnumConverter))]
        public OpCode OpCode { get; set; }

        [JsonProperty("user_id")]
        public ulong? UserId { get; set; }

        public SyncMessage() { }

        public SyncMessage(ulong userId) {
            OpCode = OpCode.UpdateUserCache;
            UserId = userId;
        }

        public SyncMessage(OpCode code, SyncMessage other)
        {
            OpCode = code;
            UserId = other.UserId;
        }
    }
}
