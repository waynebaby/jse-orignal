package io.github.marchliu.jse;

import java.util.ArrayList;
import java.util.List;

/**
 * SQL helpers for JSE query expressions.
 */
public final class Sql {
    private Sql() {}

    /** Query field list for SQL SELECT. */
    public static final String QUERY_FIELDS = "subject, predicate, object, meta";

    /**
     * Convert $pattern arguments to PostgreSQL jsonb containment triple.
     * <ul>
     *   <li>["$pattern", "$*", "author of", "$*"] -&gt; triple: ["author of"]</li>
     *   <li>["$pattern", "Liu Xin", "author of", "$*"] -&gt; triple: ["Liu Xin", "author of", "$*"]</li>
     * </ul>
     */
    public static List<String> patternToTriple(String subject, String predicate, String object) {
        if ("$*".equals(subject) && "$*".equals(object)) {
            return List.of(predicate);
        }
        List<String> triple = new ArrayList<>(3);
        triple.add(subject);
        triple.add(predicate);
        triple.add(object);
        return triple;
    }

    /**
     * Build SQL WHERE clause for a triple pattern: {@code meta @> '{"triple": [...]}'}.
     */
    public static String tripleToSqlCondition(List<String> triple) {
        String json = toJson(triple);
        String escaped = json.replace("'", "''");
        return "meta @> '" + escaped + "'";
    }

    private static String toJson(List<String> triple) {
        StringBuilder sb = new StringBuilder();
        sb.append("{\"triple\":[");
        for (int i = 0; i < triple.size(); i++) {
            if (i > 0) {
                sb.append(',');
            }
            sb.append('"').append(escapeJson(triple.get(i))).append('"');
        }
        sb.append("]}");
        return sb.toString();
    }

    private static String escapeJson(String value) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < value.length(); i++) {
            char c = value.charAt(i);
            switch (c) {
                case '\\' -> sb.append("\\\\");
                case '"' -> sb.append("\\\"");
                case '\b' -> sb.append("\\b");
                case '\f' -> sb.append("\\f");
                case '\n' -> sb.append("\\n");
                case '\r' -> sb.append("\\r");
                case '\t' -> sb.append("\\t");
                default -> {
                    if (c < 0x20) {
                        sb.append(String.format("\\u%04x", (int) c));
                    } else {
                        sb.append(c);
                    }
                }
            }
        }
        return sb.toString();
    }
}

