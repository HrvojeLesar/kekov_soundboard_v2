using System.Numerics;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace KekovBot
{
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

        public ControlMessage() { }

        public ControlMessage(OpCode code, ControlMessage other)
        {
            OpCode = code;
            GuildId = other.GuildId;
            FileId = other.FileId;
            VoiceChannelId = other.VoiceChannelId;
            MessageId = other.MessageId;
        }

        public ControlMessage(ClientError error, ControlMessage other) : this(OpCode.Error, other)
        {
            ClientError = error;
        }

        public override string ToString()
        {
            return $"OpCode: {OpCode.ToString()}\nGuildId: {GuildId}\nFileId: {FileId}\nVoiceChannelId: {VoiceChannelId}\nMessageId: {MessageId}";
        }
    }
}
