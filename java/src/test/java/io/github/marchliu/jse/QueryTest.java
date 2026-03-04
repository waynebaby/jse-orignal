package io.github.marchliu.jse;

import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertTrue;
import static io.github.marchliu.jse.Sql.QUERY_FIELDS;

class QueryTest {

    @Test
    void basicQuery() {
        Map<String, Object> query = Map.of(
                "$expr", List.of("$pattern", "$*", "author of", "$*")
        );

        Engine engine = new Engine(new Env.ExpressionEnv());
        Object result = engine.execute(query);
        String sql = (String) result;

        assertTrue(sql.contains("select"));
        assertTrue(sql.contains("subject, predicate, object, meta"));
        assertTrue(sql.contains("from statement as s"));
        assertTrue(sql.contains("author of"));
        assertTrue(sql.contains("triple"));
        assertTrue(sql.contains("offset 0"));
        assertTrue(sql.contains("limit 100"));
    }

    @Test
    void combinedQuery() {
        Map<String, Object> query = Map.of(
                "$query", List.of(
                        "$and",
                        List.of(
                                List.of("$pattern", "Liu Xin", "author of", "$*"),
                                List.of("$pattern", "$*", "author of", "$*")
                        )
                )
        );

        Engine engine = new Engine(new Env.ExpressionEnv());
        String result = (String) engine.execute(query);

        assertTrue(result.contains("select " + QUERY_FIELDS));
        assertTrue(result.contains("from statement"));
        assertTrue(result.contains("Liu Xin"));
        assertTrue(result.contains("author of"));
        assertTrue(result.contains(" and "));
        assertTrue(result.contains("offset 0"));
        assertTrue(result.contains("limit 100"));
    }
}

