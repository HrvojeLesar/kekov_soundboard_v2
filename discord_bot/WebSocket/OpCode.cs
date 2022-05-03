namespace KekovBot
{
    public enum OpCode
    {
        Connection,
        Play,
        Stop,
        Skip,
        GetQueue,
        PlayResponse,
        StopResponse,
        SkipResponse,
        GetQueueResponse,
        Error,
        UpdateUserCache,
    }

    public static class OpCodeConverter
    {
        public static OpCode? ToResponse(OpCode opCode) => opCode switch
        {
            OpCode.Play => OpCode.PlayResponse,
            OpCode.Stop => OpCode.StopResponse,
            OpCode.Skip => OpCode.SkipResponse,
            OpCode.GetQueue => OpCode.GetQueueResponse,
            _ => null,
        };

    }
}
