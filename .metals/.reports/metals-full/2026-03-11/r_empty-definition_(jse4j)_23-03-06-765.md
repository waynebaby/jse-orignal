error id: file://<WORKSPACE>/java/src/main/java/io/github/marchliu/jse/functors/BuiltinFunctors.java:io/github/marchliu/jse/FunctorBox#functor#
file://<WORKSPACE>/java/src/main/java/io/github/marchliu/jse/functors/BuiltinFunctors.java
empty definition using pc, found symbol in pc: io/github/marchliu/jse/FunctorBox#functor#
empty definition using semanticdb
empty definition using fallback
non-local guesses:

offset: 3779
uri: file://<WORKSPACE>/java/src/main/java/io/github/marchliu/jse/functors/BuiltinFunctors.java
text:
```scala
package io.github.marchliu.jse.functors;

import io.github.marchliu.jse.Functor;
import io.github.marchliu.jse.FunctorBox;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;


/**
 * Built-in functors for JSE.
 *
 * <p>Includes basic operators:
 * <ul>
 *   <li>$quote: Return argument unevaluated</li>
 *   <li>$eq: Equality comparison</li>
 *   <li>$cond: Multi-way conditional</li>
 *   <li>$cons: Prepend element to list</li>
 * </ul>
 */
public final class BuiltinFunctors {

    private BuiltinFunctors() {}

    /**
     * $quote functor - Return argument unevaluated.
     */
    public static final FunctorBox QUOTE = FunctorBox.box((env, args) -> {
        if (args.length == 0) {
            return null;
        }
        return args[0];  // Return unevaluated
    });

    /**
     * $eq functor - Equality comparison.
     */
    public static final FunctorBox EQ = FunctorBox.box((env, args) -> {
        if (args.length <= 1) {
            return true;
        }
        if (args.length == 2) {
            return eq(args[0], args[1]);
        }
        // Check all adjacent pairs are equal
        for (int i = 0; i < args.length - 1; i++) {
            if (!eq(args[i], args[i + 1])) {
                return false;
            }
        }
        return true;
    });

    /**
     * $cond functor - Multi-way conditional.
     * Form: [$cond, [pred1, expr1], [pred2, expr2], ..., [else_expr]]
     */
    public static final FunctorBox COND = FunctorBox.box((env, args) -> {
        for (Object arg : args) {
            if (!(arg instanceof List<?> clause) || clause.size() < 1) {
                throw new IllegalArgumentException("$cond clauses must be non-empty lists");
            }

            // If this is the last clause and has only one element, it's the else clause
            if (arg == args[args.length - 1] && clause.size() == 1) {
                return env.eval(clause.get(0));
            }

            if (clause.size() < 2) {
                throw new IllegalArgumentException("$cond clauses must have at least 2 elements except the last");
            }

            Object predResult = env.eval(clause.get(0));
            if (toBoolean(predResult)) {
                return env.eval(clause.get(1));
            }
        }

        return null;
    });

    /**
     * $cons functor - Prepend element to list.
     */
    public static final FunctorBox CONS = FunctorBox.box((env, args) -> {
        if (args.length != 2) {
            throw new IllegalArgumentException("$cons requires exactly 2 arguments");
        }

        Object elem = env.eval(args[0]);
        Object lst = env.eval(args[1]);

        if (!(lst instanceof List<?>)) {
            throw new IllegalArgumentException("$cons second argument must be a list");
        }

        @SuppressWarnings("unchecked")
        List<Object> list = (List<Object>) lst;
        List<Object> result = new ArrayList<>(list.size() + 1);
        result.add(elem);
        result.addAll(list);
        return result;
    });

    /**
     * Convert value to boolean.
     */
    private static boolean toBoolean(Object value) {
        if (value instanceof Boolean b) {
            return b;
        }
        return value != null;
    }

    /**
     * Check equality of two objects.
     */
    private static boolean eq(Object a, Object b) {
        if (a == null) return b == null;
        if (b == null) return false;
        return a.equals(b);
    }

    /**
     * Dictionary of all built-in functors for registration.
     */
    public static final Map<String, ? extends Functor> BUILTIN_FUNCTORS;

    static {
        BUILTIN_FUNCTORS = new LinkedHashMap<>();
        BUILTIN_FUNCTORS.put("$quote", QUOTE.functo@@r());
        BUILTIN_FUNCTORS.put("$eq", EQ.functor());
        BUILTIN_FUNCTORS.put("$cond", COND.functor());
        BUILTIN_FUNCTORS.put("$cons", CONS.functor());
    }
}

```


#### Short summary: 

empty definition using pc, found symbol in pc: io/github/marchliu/jse/FunctorBox#functor#