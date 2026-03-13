using System.Text.Json;
using System.Text.Json.Serialization;
using Jse.Ast;
using Jse.Utils;

namespace Jse.Parser;

public sealed class JseParser
{
    public static readonly JseNodeJsonConverter NodeConverter = new();
    public static readonly JsonSerializerOptions SerializerOptions = CreateSerializerOptions();

    public JseNode Parse(string json)
    {
        return JsonSerializer.Deserialize<JseNode>(json, SerializerOptions)
            ?? throw new InvalidOperationException("Failed to parse JSE JSON into AST.");
    }

    public JseNode Parse(JsonElement element)
    {
        return JsonSerializer.Deserialize<JseNode>(element.GetRawText(), SerializerOptions)
            ?? throw new InvalidOperationException("Failed to parse JSE JSON element into AST.");
    }

    private static JsonSerializerOptions CreateSerializerOptions()
    {
        var options = new JsonSerializerOptions();
        options.Converters.Add(NodeConverter);
        return options;
    }

    private static JseNode ParseArray(JsonElement array)
    {
        var items = array.EnumerateArray().ToList();
        if (items.Count > 0 && items[0].ValueKind == JsonValueKind.String)
        {
            var head = items[0].GetString() ?? string.Empty;
            if (IsSymbol(head))
            {
                var op = NormalizeSymbol(head);
                var args = items.Skip(1).Select(ParseElement).ToList();
                return new JseCall(op, args);
            }
        }

        var literalList = items.Select(i => JsonHelpers.JsonElementToObject(i)).ToList();
        return new JseLiteral(literalList);
    }

    private static JseNode ParseObject(JsonElement obj)
    {
        var props = obj.EnumerateObject().ToList();
        var symbolProps = props.Where(p => IsSymbol(p.Name)).ToList();

        if (symbolProps.Count == 1)
        {
            var opProp = symbolProps[0];
            var op = NormalizeSymbol(opProp.Name);

            var args = opProp.Value.ValueKind == JsonValueKind.Array
                ? opProp.Value.EnumerateArray().Select(ParseElement).ToList()
                : new List<JseNode> { ParseElement(opProp.Value) };

            var meta = props
                .Where(p => p.Name != opProp.Name)
                .ToDictionary(p => p.Name, p => JsonHelpers.JsonElementToObject(p.Value));

            return new JseCall(op, args, meta.Count == 0 ? null : meta);
        }

        var literalObj = props.ToDictionary(p => p.Name, p => JsonHelpers.JsonElementToObject(p.Value));
        return new JseLiteral(literalObj);
    }

    private static JseNode ParseString(string value)
    {
        if (value.StartsWith("$$", StringComparison.Ordinal))
        {
            return new JseLiteral(value[1..]);
        }

        if (IsSymbol(value))
        {
            return new JseSymbol(NormalizeSymbol(value));
        }

        return new JseLiteral(value);
    }

    private static bool IsSymbol(string value) =>
        value.StartsWith('$') && !value.StartsWith("$$", StringComparison.Ordinal);

    private static string NormalizeSymbol(string value) =>
        value.StartsWith('$') ? value[1..] : value;

    private static JseNode ParseElement(JsonElement element)
    {
        return element.ValueKind switch
        {
            JsonValueKind.Array => ParseArray(element),
            JsonValueKind.Object => ParseObject(element),
            JsonValueKind.String => ParseString(element.GetString() ?? string.Empty),
            _ => new JseLiteral(JsonHelpers.JsonElementToObject(element))
        };
    }

    public sealed class JseNodeJsonConverter : JsonConverter<JseNode>
    {
        public override JseNode Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            using var doc = JsonDocument.ParseValue(ref reader);
            return ParseElement(doc.RootElement);
        }

        public override void Write(Utf8JsonWriter writer, JseNode value, JsonSerializerOptions options)
        {
            throw new NotSupportedException("JseNode serialization is not supported by this converter.");
        }
    }
}
