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
}
