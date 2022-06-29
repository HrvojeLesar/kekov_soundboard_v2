using KekovBot.Exceptions;

namespace KekovBot.WebSocket
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
        InvalidFileId,
        NotPlaying,
        QueueFull,
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
            InvalidFileIdException => ClientError.InvalidFileId,
            NotPlayingExpcetion => ClientError.NotPlaying,
            QueueFullException => ClientError.QueueFull,
            _ => ClientError.Unknown,
        };
    }
}
