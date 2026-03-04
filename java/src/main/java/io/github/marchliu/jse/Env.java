package io.github.marchliu.jse;

/**
 * Base environment for JSE execution.
 * Can be extended to mount knowledge/statement data.
 */
public interface Env {

    /**
     * Resolve a symbol to a value. Default implementation returns {@code null}.
     *
     * @param symbol JSE symbol (e.g. {@code "$and"}).
     * @return resolved value, or {@code null} if not bound.
     */
    default Object resolve(String symbol) {
        return null;
    }

    /**
     * Expression-only environment for basic and logic evaluation.
     * No query/SQL capabilities.
     */
    final class ExpressionEnv implements Env {
        // uses default resolve implementation
    }
}

