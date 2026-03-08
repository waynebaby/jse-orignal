"""SQL extension functors for JSE.

Migrated from the original engine.py implementation:
- $pattern: Generate SQL for triple pattern matching
- $query: Generate SQL for multi-pattern queries
"""


from typing import Callable, TYPE_CHECKING
from pyjse.types import JseValue
import json
from pyjse.ast.nodes import SymbolNode
from pyjse.env import Env
from pyjse.ast.parser import Parser

class PatternNode(SymbolNode):
    """Pattern node for SQL query.
    """
    def __init__(self, name: str, env: 'Env') -> None:
        super().__init__(name, env)
        self._name = name
        self._env = env

QUERY_FIELDS = "subject, predicate, object, meta"

if TYPE_CHECKING:
    from pyjse.env import Env


# Type alias for functors
Functor = Callable[['Env', ...], JseValue]

def pattern_to_triple(subject: str, predicate: str, object: str) -> list[str]:
    """Generate triple for a pattern.
    """
    return [subject, predicate, object]

def triple_to_sql_condition(triple: list[str]) -> str:
    """Generate SQL condition for a triple pattern.
    """
    return "meta @> '" + json.dumps({"triple": triple}) + "'"


def _pattern(env: 'Env', *args: JseValue) -> JseValue:
    """Generate SQL for triple pattern matching.

    Form: [$pattern, subject, predicate, object]

    Args:
        env: Environment
        *args: (subject, predicate, object) - all must be strings

    Returns:
        SQL query string

    Raises:
        ValueError: If wrong arguments or non-string types
    """
    if len(args) < 3:
        raise ValueError("$pattern requires (subject, predicate, object)")

    subj = env.eval(args[0]) if hasattr(env, 'eval') else args[0]
    pred = env.eval(args[1]) if hasattr(env, 'eval') else args[1]
    obj = env.eval(args[2]) if hasattr(env, 'eval') else args[2]

    if all(x == "*" for x in (subj, pred, obj)):
        raise ValueError("subject, predicate, and object must not be all '*'")

    triple = pattern_to_triple(subj, pred, obj)
    cond = triple_to_sql_condition(triple)

    return (
        "select \n    subject, predicate, object, meta \n"
        f"from statement as s \nwhere {cond} \noffset 0\nlimit 100 \n"
    )

def _and(env: 'Env', *args: JseValue) -> JseValue:
    """Generate SQL for AND query.
    """
    tokens = [env.eval(e) for e in args]
    return " and ".join(tokens)

def _wildcard(env: 'Env', *args: JseValue) -> JseValue:
    """Generate SQL for wildcard query.
    """
    return "*"


def _query(env: 'Env', *args: JseValue) -> JseValue:
    """Generate SQL for multi-pattern query.

    Form: [$query, condition]
    where condition is a AST quote

    Args:
        env: Environment
        *args: query condition expression

    Returns:
        Combined SQL query string

    Raises:
        ValueError: If wrong arguments or invalid patterns
    """
    # First arg is operator (currently ignored, assumes "and")
    local = Env(env)
    local.load({
            "$pattern": _pattern,
            "$and": _and,
            "$*": _wildcard,
        })
    parser = Parser(local)
    condition = parser.parse(args)
    where = condition.apply(local)

    sql = f"select {QUERY_FIELDS} \nfrom statement \nwhere \n    {where} \n"
    return sql

# Dict of all SQL functors for registration
SQL_FUNCTORS: dict[str, Functor] = {
    "$query": _query,
}
