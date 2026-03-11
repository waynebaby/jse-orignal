error id: file://<WORKSPACE>/java/src/main/java/io/github/marchliu/jse/FunctorBox.java:io/github/marchliu/jse/FunctorBox#`<init>`().
file://<WORKSPACE>/java/src/main/java/io/github/marchliu/jse/FunctorBox.java
empty definition using pc, found symbol in pc: 
found definition using semanticdb; symbol io/github/marchliu/jse/FunctorBox#`<init>`().
empty definition using fallback
non-local guesses:

offset: 107
uri: file://<WORKSPACE>/java/src/main/java/io/github/marchliu/jse/FunctorBox.java
text:
```scala
package io.github.marchliu.jse;

import java.util.Map;
import java.util.HashMap;

abstract class FunctorBox@@ implements Functor {
    private Map<String, JseValue> meta = new HashMap<>();
    public abstract Object apply(Env env, Object... args);
    public Map<String, JseValue> getMeta() {
        return this.meta;
    }

    public void putMeta(String key, JseValue value) {
        this.meta.put(key, value);
    }

}

```


#### Short summary: 

empty definition using pc, found symbol in pc: 