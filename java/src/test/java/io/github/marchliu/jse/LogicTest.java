package io.github.marchliu.jse;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class LogicTest {
    private final Engine engine = new Engine(new Env.ExpressionEnv());

    @Test
    void andBasic() {
        assertEquals(true, engine.execute(List.of("$and", true, true, true)));
        assertEquals(false, engine.execute(List.of("$and", true, false, true)));
    }

    @Test
    void orBasic() {
        assertEquals(true, engine.execute(List.of("$or", false, false, true)));
        assertEquals(false, engine.execute(List.of("$or", false, false, false)));
    }

    @Test
    void notBasic() {
        assertEquals(false, engine.execute(List.of("$not", true)));
        assertEquals(true, engine.execute(List.of("$not", false)));
    }

    @Test
    void nestedLogic() {
        List<Object> expr = List.of(
                "$or",
                List.of("$and", true, List.of("$not", false)),
                List.of("$and", false, true)
        );
        assertEquals(true, engine.execute(expr));
    }

    @Test
    void deepNesting() {
        List<Object> expr = List.of(
                "$not",
                List.of(
                        "$or",
                        List.of("$and", false, List.of("$not", false)),
                        List.of("$not", true)
                )
        );
        assertEquals(true, engine.execute(expr));
    }
}

