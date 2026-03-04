# JSE

JSON Structural Expression (JSE) is a JSON-based structural expression format.
It turns plain JSON from a passive data format into a medium that can carry structured intent and computation semantics.

## Features

- **Pure JSON**: always valid JSON, easy to generate and parse
- **S-expression style**: `$`-prefixed symbols and list/object forms
- **Composable**: can express logical and query-like structures
- **Multi-language runtimes**: Python, TypeScript/JavaScript, Java

## Installation

### Python (PyPI)

```bash
pip install pyjse
```

### TypeScript / JavaScript (npm)

```bash
npm install jse-engine
```

### Java (Maven Central)

Maven:

```xml
<dependency>
  <groupId>io.github.marchliu</groupId>
  <artifactId>jse4j</artifactId>
  <version>0.1.0</version>
</dependency>
```

Gradle Groovy:

```groovy
implementation 'io.github.marchliu:jse4j:0.1.0'
```

Gradle Kotlin:

```kotlin
implementation("io.github.marchliu:jse4j:0.1.0")
```

## Usage

Basic idea: use `$`-prefixed symbols inside JSON to express structure.

Python example:

```python
from pyjse import Engine, ExpressionEnv

engine = Engine(ExpressionEnv())

engine.execute(42)                          # 42
engine.execute(["$and", True, True, False])  # False
engine.execute({"$expr": ["$pattern", "$*", "author of", "$*"]})  # SQL string
```

## Development

This repo contains implementations in multiple languages:

- `python/` – Python runtime (`pyjse`)
- `typescript/` – TypeScript runtime (`jse-engine`)
- `java/` – Java runtime (`jse4j`)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

Mars Liu - [@MarchLiu](https://github.com/MarchLiu)

Project Link: [https://github.com/MarchLiu/jse](https://github.com/MarchLiu/jse)
