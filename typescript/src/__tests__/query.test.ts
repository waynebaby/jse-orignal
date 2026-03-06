import { describe, it, expect } from "vitest";
import { Engine, Env, UTILS_FUNCTORS, SQL_FUNCTORS } from "../index.js";

function createEngine() {
  const env = new Env();
  env.load(UTILS_FUNCTORS, SQL_FUNCTORS);
  return new Engine(env);
}

describe("query expressions", () => {
  const engine = createEngine();

  it("basic $expr $pattern query", () => {
    const query = {
      $expr: ["$pattern", "$*", "author of", "$*"],
    };

    const result = engine.execute(query) as string;

    expect(result).toContain("select");
    expect(result).toContain("subject, predicate, object, meta");
    expect(result).toContain("from statement as s");
    expect(result).toContain("author of");
    expect(result).toContain("triple");
    expect(result).toContain("offset 0");
    expect(result).toContain("limit 100");
  });

  it("combined $query with $and patterns", () => {
    const query = {
      $query: [
        "$and",
        [
          ["$pattern", "Liu Xin", "author of", "$*"],
          ["$pattern", "$*", "author of", "$*"],
        ],
      ],
    };

    const result = engine.execute(query) as string;

    expect(result).toContain("select subject, predicate, object, meta");
    expect(result).toContain("from statement");
    expect(result).toContain("Liu Xin");
    expect(result).toContain("author of");
    expect(result).toContain(" and ");
    expect(result).toContain("offset 0");
    expect(result).toContain("limit 100");
  });
});
