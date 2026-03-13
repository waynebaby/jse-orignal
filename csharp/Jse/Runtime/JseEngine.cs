using System.Text.Json;
using Jse.Execution;
using Jse.Serialization;

namespace Jse.Runtime;

public sealed class JseEngine
{
    private readonly ExpressionCompiler _compiler;
    private readonly OperatorEnvironment _environment;

    public JseEngine()
        : this(OperatorRegistry.CreateDefault())
    {
    }

    public JseEngine(OperatorRegistry registry)
    {
        _compiler = new ExpressionCompiler();
        _environment = new OperatorEnvironment(registry);
    }

    public object? Execute(string json)
    {
        using var doc = JsonDocument.Parse(json);
        return Execute(doc.RootElement);
    }

    public object? Execute(JsonElement element)
    {
        var ast = JseRuntimeSerializer.DeserializeNode(element);
        var executable = _compiler.Compile(ast, _environment);
        return executable();
    }
}
