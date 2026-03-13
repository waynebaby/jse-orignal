using System.Text.Json;
using Jse.Runtime;
using Xunit;

namespace Jse.Tests;

public class JseEngineTests
{
    private readonly JseEngine _engine = new();

    [Theory]
    [InlineData("42", 42L)]
    [InlineData("\"hello\"", "hello")]
    [InlineData("true", true)]
    public void Execute_LiteralValues_ReturnsOriginal(string json, object expected)
    {
        var result = _engine.Execute(json);

        if (expected is long && result is not null)
        {
            Assert.Equal(Convert.ToInt64(expected), Convert.ToInt64(result));
            return;
        }

        Assert.Equal(expected, result);
    }

    [Fact]
    public void Execute_SimpleExpression_Addition()
    {
        var result = _engine.Execute("[\"$add\",1,2]");
        Assert.Equal(3.JsonValue(), result);
    }

    [Fact]
    public void Execute_NestedExpression_Addition()
    {
        var result = _engine.Execute("[\"$add\",1,[\"$add\",2,3]]");
        Assert.Equal(6.JsonValue(), result);
    }

    [Fact]
    public void Execute_NamedForm_Addition()
    {
        var json = """
                   {
                     "$add": [1, 2],
                     "source": "user"
                   }
                   """;

        var result = _engine.Execute(json);
        Assert.Equal(3.JsonValue(), result);
    }

    [Fact]
    public void Execute_Quote_PreventsEvaluation()
    {
        var result = _engine.Execute("[\"$quote\",[\"$add\",1,2]]");
        var quoted = Assert.IsType<List<object?>>(result);

        Assert.Equal("$add", quoted[0]);
        Assert.Equal(1.JsonValue(), quoted[1]);
        Assert.Equal(2.JsonValue(), quoted[2]);
    }

    [Fact]
    public void Execute_SymbolEscape_ReturnsLiteralDollarString()
    {
        var result = _engine.Execute("\"$$add\"");
        Assert.Equal("$add", result);
    }

    [Fact]
    public void Execute_WithJsonElementInput_Works()
    {
        using var doc = JsonDocument.Parse("[\"$eq\", 1, 1]");
        var result = _engine.Execute(doc.RootElement);
        Assert.Equal(true, result);
    }
}
