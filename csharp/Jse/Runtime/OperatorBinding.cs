namespace Jse.Runtime;

public sealed class OperatorBinding
{
    public OperatorBinding(string name, Delegate implementation, IReadOnlyList<Type> parameterTypes, Type returnType)
    {
        Name = name;
        Implementation = implementation;
        ParameterTypes = parameterTypes;
        ReturnType = returnType;
    }

    public string Name { get; }

    public Delegate Implementation { get; }

    public IReadOnlyList<Type> ParameterTypes { get; }

    public Type ReturnType { get; }
}
