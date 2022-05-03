namespace KekovBot
{
    public class WebSocketException : Exception
    {
        public WebSocketException() { }
        public WebSocketException(string message) : base(message) { }
    }

    public class InvalidGuildIdException : WebSocketException
    {
        public InvalidGuildIdException() : base("Provided Guild id is invalid") { }
    }

    public class GuildNotFoundException : WebSocketException
    {
        public GuildNotFoundException() : base("Guild not found") { }
    }

    public class ChannelNotFoundException : WebSocketException
    {
        public ChannelNotFoundException() : base("Channel not found") { }
    }

    public class ChannelsEmptyException : WebSocketException
    {
        public ChannelsEmptyException() : base("All channels are empty") { }
    }

    public class InvalidFileIdException : WebSocketException
    {
        public InvalidFileIdException() : base("Invalid file id") { }
    }

    public class LavalinkConnectionNotEstablishedException : WebSocketException
    {
        public LavalinkConnectionNotEstablishedException() : base("Lavalink connection is not established") { }
    }

    public class InvalidVoiceChannelException : WebSocketException
    {
        public InvalidVoiceChannelException() : base("Not a valid voice channel") { }
    }

    public class FileLoadingFailedException : WebSocketException
    {
        public FileLoadingFailedException() : base("Failed to load file") { }
    }

    public class NotPlayingExpcetion : WebSocketException
    {
        public NotPlayingExpcetion() : base("Nothing is playing") { }
    }
}
