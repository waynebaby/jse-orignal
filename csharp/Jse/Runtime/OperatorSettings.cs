namespace Jse.Runtime;

public sealed class OperatorSettings
{
    public OperatorSettings(OperatorRegistry operators)
    {
        Operators = operators;
    }

    public OperatorRegistry Operators { get; }
}
