namespace KekovBot
{
    public enum OpCode
    {
        Connection,
        Play,
        Stop,
        PlayResponse,
        StopResponse,
        Error,
    }

    public enum ClientError
    {
        InvalidGuildId,
        GuildNotFound,
        ChannelNotFound,
        ChannelsEmpty,
        Unknown,
    }

    public static class OpCodeConverter
    {
        public static OpCode? ToResponse(OpCode opCode) => opCode switch
        {
            OpCode.Play => OpCode.PlayResponse,
            OpCode.Stop => OpCode.StopResponse,
            _ => null,
        };

    }

    public static class ClientErrorConverter
    {
        public static ClientError ToClientError(WebSocketException e) => e.GetBaseException() switch
        {
            InvalidGuildIdException => ClientError.InvalidGuildId,
            GuildNotFoundException => ClientError.GuildNotFound,
            ChannelNotFoundException => ClientError.ChannelNotFound,
            ChannelsEmptyException => ClientError.ChannelsEmpty,
            _ => ClientError.Unknown,
        };
    }
}
