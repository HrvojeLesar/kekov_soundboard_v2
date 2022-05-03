using System.Numerics;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace KekovBot
{
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class ControlMessage
    {
        [JsonProperty("op")]
        [JsonConverter(typeof(StringEnumConverter))]
        public OpCode OpCode { get; set; }

        [JsonProperty("guild_id")]
        public ulong? GuildId { get; set; }

        [JsonProperty("file_id")]
        public ulong? FileId { get; set; }

        [JsonProperty("voice_channel_id")]
        public ulong? VoiceChannelId { get; set; }

        [JsonProperty("message_id")]
        public BigInteger MessageId { get; set; }

        [JsonProperty("client_error")]
        [JsonConverter(typeof(StringEnumConverter))]
        public ClientError? ClientError { get; set; }

        [JsonProperty("queue")]
        public List<Sound>? Queue { get; set; }

        public ControlMessage() { }

        public ControlMessage(OpCode code, List<Sound>? queue, ControlMessage other)
        {
            OpCode = code;
            GuildId = other.GuildId;
            FileId = other.FileId;
            VoiceChannelId = other.VoiceChannelId;
            MessageId = other.MessageId;
            Queue = queue;
        }

        public ControlMessage(ClientError error, ControlMessage other) : this(OpCode.Error, null, other)
        {
            ClientError = error;
        }
    }
}
