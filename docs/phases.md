
# 1. Normalizer

1. Bind line numbers to lines of code
2. Remove all comments
3. Convert ";" to "/n"
4. Expand one line to many if needed (for example on Chaining).
5. Normalize function names (To String -> tostring and "جمع" -> add)

# 2. Parser

1. Parse syntax to Operation
2. Throw syntax error

# 3. Static Analyzer

1. Function existant and static type checker
2. Finding undefind used variables

# 4. Optimizer

1. Garbage Collector

# 5-1 REPL

Read Evaluate Print Loop

# 5-2. Runtime

Runs iter of Operations
