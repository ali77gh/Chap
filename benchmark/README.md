
# Chap benchmark

## Chap vs python

Chap:

```chp
0 -> $counter
@loop
    $counter -> increase
@loop , $counter, 30000000  -> jump_if_not_equal
$counter
```

Python:

```chp
count = 0
while count!=30000000:
    count+=1;
print(count)
```

| Language       | Time | overhead    |
|----------------|------|-------------|
| Python 3.11.3  | 3.2s | N/A         |
| Chap 2.0.0     | 9.0s | 5.8s slower |
