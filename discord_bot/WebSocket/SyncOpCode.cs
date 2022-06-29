namespace KekovBot.WebSocket
{
    public enum SyncOpCode
    {
        UpdateUserCache,
        InvalidateGuildsCache,
        UpdateGuildChannels,
        AddGuild,
        RemoveGuild,
    }
}
