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
        PlayResponseQueued,
        StopResponse,
        SkipResponse,
        GetQueueResponse,
        Error,
        UpdateUserCache,
    }

    public static class OpCodeConverter
    {
        public static OpCode? ToResponse(OpCode opCode, bool isQueued = false)
        {
            if (opCode == OpCode.Play && isQueued)
            {
                return OpCode.PlayResponseQueued;
            }

            return opCode switch
            {
                OpCode.Play => OpCode.PlayResponse,
                OpCode.Stop => OpCode.StopResponse,
                OpCode.Skip => OpCode.SkipResponse,
                OpCode.GetQueue => OpCode.GetQueueResponse,
                _ => null,
            };
        }

    }
}
