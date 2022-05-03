namespace KekovBot
{
    public enum OpCode
    {
        Connection,
        Play,
        Stop,
        Skip,
        PlayResponse,
        StopResponse,
        SkipResponse,
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
            _ => null,
        };

    }
}
