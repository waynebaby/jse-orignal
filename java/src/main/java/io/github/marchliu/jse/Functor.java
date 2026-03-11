package io.github.marchliu.jse;

import java.util.Map;
import java.util.HashMap;

/**
 * Functional interface for JSE functors (operators).
 *
 * <p>A functor takes an environment and variable arguments,
 * and returns a result value.</p>
 */
@FunctionalInterface
public interface Functor {

    /**
     * Apply this functor with the given environment and arguments.
     *
     * @param env Execution environment
     * @param args Arguments (already evaluated)
     * @return Result value
     */
    Object apply(Env env, Object... args);

}
