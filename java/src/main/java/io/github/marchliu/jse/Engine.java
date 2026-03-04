package io.github.marchliu.jse;

import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.Objects;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import static io.github.marchliu.jse.Sql.QUERY_FIELDS;
import static io.github.marchliu.jse.Sql.patternToTriple;
import static io.github.marchliu.jse.Sql.tripleToSqlCondition;

/**
 * JSE (JSON Structural Expression) execution engine for Java.
 *
 * <p>This mirrors the semantics of the Python and TypeScript implementations.</p>
 */
public final class Engine {
    private final Env env;

    public Engine(Env env) {
        this.env = Objects.requireNonNull(env, "env");
    }

    /**
     * Execute a JSE expression.
     */
    public Object execute(Object expr) {
        // literals
        if (expr == null || expr instanceof Number || expr instanceof Boolean) {
            return expr;
        }
        if (expr instanceof String s) {
            return unescapeSymbol(s);
        }

        // list: s-expression if first element is symbol
        if (expr instanceof List<?> list) {
            if (list.isEmpty()) {
                return list;
            }
            Object first = list.get(0);
            if (first instanceof String s && isSymbol(s)) {
                List<?> tail = list.subList(1, list.size());
                return evalSExpr(s, tail);
            }
            List<Object> result = new ArrayList<>(list.size());
            for (Object e : list) {
                result.add(execute(e));
            }
            return result;
        }

        // object
        if (expr instanceof Map<?, ?> rawMap) {
            @SuppressWarnings("unchecked")
            Map<String, Object> map = (Map<String, Object>) rawMap;
            String sym = getSExprKey(map);
            if (sym != null) {
                Object tail = map.get(sym);
                List<?> args;
                if ("$expr".equals(sym)) {
                    args = List.of(tail);
                } else if (tail instanceof List<?> l) {
                    args = l;
                } else {
                    args = List.of(tail);
                }
                return evalSExpr(sym, args);
            }

            Map<String, Object> result = new LinkedHashMap<>();
            for (Map.Entry<String, Object> entry : map.entrySet()) {
                String key = unescapeSymbol(entry.getKey());
                result.put(key, execute(entry.getValue()));
            }
            return result;
        }

        return expr;
    }

    private static boolean isSymbol(String s) {
        return s.startsWith("$") && !s.startsWith("$$");
    }

    private static String unescapeSymbol(String s) {
        if (s.startsWith("$$")) {
            return s.substring(1);
        }
        return s;
    }

    private static String getSExprKey(Map<String, Object> obj) {
        String found = null;
        for (String k : obj.keySet()) {
            if (k.startsWith("$") && !k.startsWith("$$")) {
                if (found != null) {
                    return null; // more than one symbol key
                }
                found = k;
            }
        }
        return found;
    }

    private Object evalSExpr(String symbol, List<?> args) {
        // $quote: do not evaluate the argument
        if ("$quote".equals(symbol)) {
            return args.isEmpty() ? null : args.get(0);
        }

        List<Object> evaluated = new ArrayList<>(args.size());
        for (Object a : args) {
            evaluated.add(execute(a));
        }

        return switch (symbol) {
            case "$and" -> evalAnd(evaluated);
            case "$or" -> evalOr(evaluated);
            case "$not" -> evalNot(evaluated);
            case "$expr" -> evaluated.isEmpty() ? null : evaluated.get(0);
            case "$pattern" -> evalPattern(evaluated);
            case "$query" -> evalQuery(evaluated);
            default -> {
                Object resolved = env.resolve(symbol);
                if (resolved != null) {
                    yield resolved;
                }
                throw new IllegalArgumentException("Unknown symbol: " + symbol);
            }
        };
    }

    private static boolean toBoolean(Object value) {
        if (value instanceof Boolean b) {
            return b;
        }
        return value != null;
    }

    private static boolean evalAnd(List<Object> evaluated) {
        for (Object v : evaluated) {
            if (!toBoolean(v)) {
                return false;
            }
        }
        return true;
    }

    private static boolean evalOr(List<Object> evaluated) {
        for (Object v : evaluated) {
            if (toBoolean(v)) {
                return true;
            }
        }
        return false;
    }

    private static boolean evalNot(List<Object> evaluated) {
        if (evaluated.isEmpty()) {
            return true;
        }
        return !toBoolean(evaluated.get(0));
    }

    private static String evalPattern(List<Object> evaluated) {
        if (evaluated.size() < 3) {
            throw new IllegalArgumentException("$pattern requires (subject, predicate, object)");
        }
        Object subj = evaluated.get(0);
        Object pred = evaluated.get(1);
        Object obj = evaluated.get(2);
        if (!(subj instanceof String s && pred instanceof String p && obj instanceof String o)) {
            throw new IllegalArgumentException("$pattern requires string arguments");
        }
        List<String> triple = patternToTriple(s, p, o);
        String cond = tripleToSqlCondition(triple);
        return "select \n" +
               "    subject, predicate, object, meta \n" +
               "from statement as s \n" +
               "where " + cond + " \n" +
               "offset 0\n" +
               "limit 100 \n";
    }

    private static final Pattern WHERE_PATTERN =
            Pattern.compile("where\\s+(.+?)\\s+offset", Pattern.CASE_INSENSITIVE | Pattern.DOTALL);

    private static String evalQuery(List<Object> evaluated) {
        if (evaluated.size() < 2 || !(evaluated.get(1) instanceof List<?> patterns)) {
            throw new IllegalArgumentException("$query expects [op, patterns array]");
        }
        List<String> conditions = new ArrayList<>();
        for (Object sqlObj : patterns) {
            if (!(sqlObj instanceof String sql)) {
                throw new IllegalArgumentException("Pattern must evaluate to SQL string");
            }
            Matcher m = WHERE_PATTERN.matcher(sql);
            if (m.find()) {
                conditions.add("(" + m.group(1).trim() + ")");
            } else {
                conditions.add(sql);
            }
        }
        String whereClause = String.join(" and \n    ", conditions);
        return "select " + QUERY_FIELDS + " \n" +
               "from statement \n" +
               "where \n" +
               "    " + whereClause + " \n" +
               "offset 0\n" +
               "limit 100 \n";
    }
}

