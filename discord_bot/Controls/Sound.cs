using dotenv.net;
using Newtonsoft.Json;

namespace KekovBot
{
    public class Sound
    {
        private static string _soundFileDir = DotEnv.Read()["SOUNDFILE_DIR"];

        [JsonProperty("id")]
        [JsonConverter(typeof(ToStringConverter))]
        public ulong FileId;

        [JsonIgnore]
        public FileInfo FileInfo;

        public Sound(ulong file_id)
        {
            FileId = file_id;
            FileInfo = new FileInfo($"{_soundFileDir}{FileId}");
        }
    }
}
