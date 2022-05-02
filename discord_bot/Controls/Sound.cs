using dotenv.net;

namespace KekovBot
{
    public class Sound
    {
        private static string _soundFileDir = DotEnv.Read()["SOUNDFILE_DIR"];
        public ulong FileId;
        public FileInfo FileInfo;

        public Sound(ulong file_id)
        {
            FileId = file_id;
            FileInfo = new FileInfo($"{_soundFileDir}{FileId}");
        }
    }
}
