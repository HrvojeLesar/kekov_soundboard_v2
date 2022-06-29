using dotenv.net;
using KekovBot.WebSocket;
using Newtonsoft.Json;

namespace KekovBot.Control
{
    public class Sound
    {
        private static string _soundFileDir = DotEnv.Read()["SOUNDFILE_DIR"];

        [JsonProperty("id")]
        [JsonConverter(typeof(ToStringConverter))]
        public ulong FileId;

        [JsonProperty("display_name")]
        public string DisplayName;

        [JsonIgnore]
        public FileInfo FileInfo;

        public Sound(ulong fileId, string displayName)
        {
            FileId = fileId;
            DisplayName = displayName;
            FileInfo = new FileInfo($"{_soundFileDir}{FileId}");
        }
    }
}
