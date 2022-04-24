namespace KekovBot
{
    public enum OpCode
    {
        Connection,
        Play,
        Stop,
        PlayResponse,
        StopResponse,
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
}
