package io.github.marchliu.jse;

import java.util.Map;
import java.util.HashMap;

public abstract class FunctorBox implements Functor {
    private Map<String, JseValue> meta = new HashMap<>();

    public Map<String, JseValue> getMeta() {
        return this.meta;
    }


    public void putMeta(String key, JseValue value) {
        this.meta.put(key, value);
    }

    public void setMeta(Map<String, JseValue> meta) {
        this.meta = meta;
    }

    @Override
    public abstract Object apply(Env env, Object... args);


    public static FunctorBox box(Functor functor) {
        return new FunctorBox() {
            @Override
            public Object apply(Env env, Object... args) {
                return functor.apply(env, args);
            }
        };
    }
}
