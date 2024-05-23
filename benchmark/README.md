
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
| Chap 2.0.0     | 8.4s | 5.2s slower |

## Chap vs Chap (finding slow parts)(version 2.0.0)

### Time Chart

| Function       | Time range        | Time avg |
|----------------|-------------------|----------|
| nothing        | 8.838s  - 9.200s  | 9.018s   |
| pass           | 10.261s - 10.482s | 10.371s  |
| increase       | 13.700s - 13.884s | 13.792s  |
| increase(opt)  | 10.200s - 10.500s | 11.350s  |
| var = add(1,2) | 13.370s - 14.300s | 13.835s  |
| var = add(a,2) | 14.346s - 15.363s | 14.854s  |
| var = add(a,b) | 15.984s - 16.845s | 16.414s  |
| jump           | 12.103s - 12.816s | 12.459s  |

### Technical Analyis

| Function       | Overhead | Operations |
|----------------|----------|------------|
| nothing        | N/A      | N/A        |
| pass           | 1.352s   | rel        |
| increase       | 4.773s   | rel + cpe + cpt + vg + cve + cpdt + mo + vs |
| increase(opt)  | 1.332s   | rel + cpe + cpt + vg + cve + cpdt + mo |
| var = add(1,2) | 4.817s   | rel + (cpe + cpt) + (cpe + cpt) + cpdt + cpdt + mo + cope + vs |
| var = add(a,2) | 5.835s   | rel + (cpe + cpt + vg + cve) + (cpe + cpt) + cpdt + cpdt + mo + cope + vs |
| var = add(a,b) | 7.396s   | rel + (cpe + cpt + vg + cve) + (cpe + cpt + vg + cve) + cpdt + cpdt + mo + cope + vs |
| jump           | 3.400s   | rel + cpe + cpt + tg + cte + ts |

### Operations

| Short-form | Actual meaning            |
|------------|---------------------------|
| rel        | runtime execute line      |
| cpe        | check param exist         |
| cpt        | check param type          |
| cpdt       | check param datatype      |
| vg         | var-get                   |
| mo         | math operation            |
| vs         | var-set                   |
| cve        | check variable exist      |
| cope       | check output param exists |
| tg         | tag-get                   |
| cte        | check tag exists          |
| ts         | current line set          |

### Results

| Short-form                            | time overhead (calculated) |
|---------------------------------------|----------------------------|
| rel                                   | 1.3s                       |
| cpe + cpt + vg + cve + cpdt + mo + vs | 3.4s                       |
| vg + cve                              | 1.1s                       |
| cpe + cpt + cpdt + mo + vs            | 2.3s (3.4 - 1.1)           |
| cpe + cpt + tg + cte + ts             | 2.1s                       |
| vs                                    | 2.4s (4.7 - 2.3)           |
