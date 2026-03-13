using System.Collections.ObjectModel;

namespace Jse.Runtime;

public sealed class OperatorRegistry
{
    private readonly Dictionary<string, List<OperatorBinding>> _operators = new(StringComparer.Ordinal);

    public OperatorRegistry Register<TResult>(string name, Func<TResult> implementation)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(name);
        ArgumentNullException.ThrowIfNull(implementation);

        return RegisterBinding(name, implementation, Array.Empty<Type>(), typeof(TResult));
    }

    public OperatorRegistry Register<T1, TResult>(string name, Func<T1, TResult> implementation)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(name);
        ArgumentNullException.ThrowIfNull(implementation);

        return RegisterBinding(name, implementation, new[] { typeof(T1) }, typeof(TResult));
    }

    public OperatorRegistry Register<T1, T2, TResult>(string name, Func<T1, T2, TResult> implementation)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(name);
        ArgumentNullException.ThrowIfNull(implementation);

        return RegisterBinding(name, implementation, new[] { typeof(T1), typeof(T2) }, typeof(TResult));
    }

    public OperatorRegistry Register<T1, T2, T3, TResult>(string name, Func<T1, T2, T3, TResult> implementation)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(name);
        ArgumentNullException.ThrowIfNull(implementation);

        return RegisterBinding(name, implementation, new[] { typeof(T1), typeof(T2), typeof(T3) }, typeof(TResult));
    }

    private OperatorRegistry RegisterBinding(string name, Delegate implementation, IReadOnlyList<Type> parameterTypes, Type returnType)
    {
        if (!_operators.TryGetValue(name, out var overloads))
        {
            overloads = new List<OperatorBinding>();
            _operators[name] = overloads;
        }

        overloads.Add(new OperatorBinding(name, implementation, parameterTypes, returnType));
        return this;
    }

    public OperatorBinding Resolve(string name, IReadOnlyList<Type> argumentTypes)
    {
        if (!_operators.TryGetValue(name, out var overloads))
        {
            throw new InvalidOperationException($"Operator '{name}' is not registered.");
        }

        var candidates = overloads
            .Where(binding => IsApplicable(binding, argumentTypes))
            .ToList();

        if (candidates.Count == 0)
        {
            throw new InvalidOperationException(
                $"No applicable overload found for operator '{name}' with argument types ({string.Join(", ", argumentTypes.Select(static t => t.Name))}).");
        }

        candidates.Sort((x, y) => CompareSpecificity(x, y, argumentTypes));

        if (candidates.Count > 1 && CompareSpecificity(candidates[0], candidates[1], argumentTypes) == 0)
        {
            throw new InvalidOperationException(
                $"Ambiguous overload for operator '{name}' with argument types ({string.Join(", ", argumentTypes.Select(static t => t.Name))}).");
        }

        return candidates[0];
    }

    public IReadOnlyDictionary<string, IReadOnlyList<OperatorBinding>> Snapshot() =>
        new ReadOnlyDictionary<string, IReadOnlyList<OperatorBinding>>(
            _operators.ToDictionary(static kvp => kvp.Key, static kvp => (IReadOnlyList<OperatorBinding>)kvp.Value.AsReadOnly()));

    public static OperatorRegistry CreateDefault()
    {
        var registry = new OperatorRegistry();

        registry.Register<decimal, decimal, decimal>("add", static (a, b) => a + b);
        registry.Register<long, long, long>("add", static (a, b) => a + b);
        registry.Register<double, double, double>("add", static (a, b) => a + b);
        registry.Register<object?, object?, bool>("eq", static (a, b) => Equals(a, b));
        registry.Register<bool, bool, bool>("and", static (a, b) => a && b);
        registry.Register<bool, bool, bool>("or", static (a, b) => a || b);
        registry.Register<bool, object?, object?, object?>("cond", static (c, t, f) => c ? t : f);

        return registry;
    }

    private static bool IsApplicable(OperatorBinding binding, IReadOnlyList<Type> argumentTypes)
    {
        if (binding.ParameterTypes.Count != argumentTypes.Count)
        {
            return false;
        }

        for (var i = 0; i < argumentTypes.Count; i++)
        {
            var parameterType = binding.ParameterTypes[i];
            var argumentType = argumentTypes[i];

            if (!CanAssign(argumentType, parameterType))
            {
                return false;
            }
        }

        return true;
    }

    private static int CompareSpecificity(OperatorBinding x, OperatorBinding y, IReadOnlyList<Type> argumentTypes)
    {
        var xBetter = 0;
        var yBetter = 0;

        for (var i = 0; i < argumentTypes.Count; i++)
        {
            var xDistance = TypeDistance(argumentTypes[i], x.ParameterTypes[i]);
            var yDistance = TypeDistance(argumentTypes[i], y.ParameterTypes[i]);

            if (xDistance < yDistance)
            {
                xBetter++;
            }
            else if (yDistance < xDistance)
            {
                yBetter++;
            }
        }

        if (xBetter > 0 && yBetter == 0)
        {
            return -1;
        }

        if (yBetter > 0 && xBetter == 0)
        {
            return 1;
        }

        return 0;
    }

    private static bool CanAssign(Type sourceType, Type targetType)
    {
        if (sourceType == targetType)
        {
            return true;
        }

        if (sourceType == typeof(object))
        {
            return !targetType.IsValueType || Nullable.GetUnderlyingType(targetType) is not null;
        }

        return targetType.IsAssignableFrom(sourceType);
    }

    private static int TypeDistance(Type sourceType, Type targetType)
    {
        if (sourceType == targetType)
        {
            return 0;
        }

        if (targetType == typeof(object))
        {
            return 100;
        }

        if (sourceType == typeof(object))
        {
            return 90;
        }

        if (targetType.IsInterface)
        {
            return 50;
        }

        if (targetType.IsAssignableFrom(sourceType))
        {
            var distance = 1;
            var current = sourceType.BaseType;
            while (current is not null && current != targetType)
            {
                distance++;
                current = current.BaseType;
            }

            return distance;
        }

        return int.MaxValue;
    }
}
