namespace KekovBot
{
    public enum ClientError
    {
        InvalidGuildId,
        GuildNotFound,
        ChannelNotFound,
        ChannelsEmpty,
        LavalinkConnectionNotEstablished,
        InvalidVoiceChannel,
        FileLoadingFailed,
        Unknown,
    }

    public static class ClientErrorConverter
    {
        public static ClientError ToClientError(WebSocketException e) => e.GetBaseException() switch
        {
            InvalidGuildIdException => ClientError.InvalidGuildId,
            GuildNotFoundException => ClientError.GuildNotFound,
            ChannelNotFoundException => ClientError.ChannelNotFound,
            ChannelsEmptyException => ClientError.ChannelsEmpty,
            LavalinkConnectionNotEstablishedException => ClientError.LavalinkConnectionNotEstablished,
            InvalidVoiceChannelException => ClientError.InvalidVoiceChannel,
            FileLoadingFailedException => ClientError.FileLoadingFailed,
            _ => ClientError.Unknown,
        };
    }
}
