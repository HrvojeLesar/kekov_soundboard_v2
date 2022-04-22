using Newtonsoft.Json;

namespace KekovBot
{
    public class ControlMessage
    {
        [JsonProperty("op")]
        public OpCode OpCode { get; set; }

        [JsonProperty("guild_id")]
        public ulong? GuildId { get; set; }

        [JsonProperty("file_id")]
        public ulong? FileId { get; set; }

        public override string ToString()
        {
            return $"OpCode: {OpCode.ToString()}\nGuildId: {GuildId}\nFileId: {FileId}";
        }
    }
}
