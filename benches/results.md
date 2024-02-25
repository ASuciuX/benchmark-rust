# for_loop_with_new vs for_loop_with_vec_capacity vs iterator
## 1 word benchmarking

### First run
```shell
for_loop_vec_new        
time:   [118.01 ns 118.29 ns 118.60 ns]
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe

for_loop_vec_with_capacity
time:   [116.50 ns 116.92 ns 117.40 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

map                     
time:   [119.93 ns 120.26 ns 120.64 ns]
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe

flat_map                
time:   [109.86 ns 110.36 ns 110.87 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

append                  
time:   [99.150 ns 99.339 ns 99.542 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

append_with_capacity    
time:   [99.708 ns 100.12 ns 100.56 ns]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
```

### Second run
```shell
for_loop_vec_new        
time:   [118.74 ns 119.02 ns 119.30 ns]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

for_loop_vec_with_capacity                        
time:   [119.22 ns 119.66 ns 120.11 ns]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild

map                     
time:   [120.63 ns 121.10 ns 121.59 ns]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

flat_map                
time:   [109.19 ns 109.52 ns 109.86 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

append                  
time:   [99.723 ns 100.13 ns 100.54 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

append_with_capacity    
time:   [100.23 ns 100.47 ns 100.72 ns]
```

## 1000 words benchmarking

### First run
```shell
for_loop_vec_new        
time:   [110.50 µs 111.01 µs 111.54 µs]
Found 2 outliers among 100 measurements (2.00%)
1 (1.00%) high mild
1 (1.00%) high severe

for_loop_vec_with_capacity
time:   [106.11 µs 106.95 µs 107.86 µs]
Found 3 outliers among 100 measurements (3.00%)
2 (2.00%) high mild
1 (1.00%) high severe

map                     
time:   [115.94 µs 116.42 µs 116.96 µs]
Found 3 outliers among 100 measurements (3.00%)
3 (3.00%) high mild

flat_map                
time:   [114.31 µs 114.70 µs 115.11 µs]

append                  
time:   [105.00 µs 105.75 µs 106.61 µs]
Found 5 outliers among 100 measurements (5.00%)
3 (3.00%) high mild
2 (2.00%) high severe
```


### Second run

```shell
for_loop_vec_new        
time:   [108.69 µs 109.27 µs 109.89 µs]

for_loop_vec_with_capacity
time:   [106.17 µs 106.72 µs 107.43 µs]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high severe

map                     
time:   [116.43 µs 116.72 µs 117.03 µs]
Found 3 outliers among 100 measurements (3.00%)
2 (2.00%) high mild
1 (1.00%) high severe

flat_map                
time:   [113.22 µs 113.47 µs 113.72 µs]
Found 4 outliers among 100 measurements (4.00%)
4 (4.00%) high mild

append                  
time:   [104.97 µs 105.33 µs 105.73 µs]
```

### Third run

```shell
for_loop_vec_new       
time:   [107.29 µs 107.73 µs 108.19 µs]
Found 4 outliers among 100 measurements (4.00%)
4 (4.00%) high mild

for_loop_vec_with_capacity
time:   [104.93 µs 105.38 µs 105.87 µs]
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild

map
time:   [112.79 µs 113.33 µs 113.87 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) low mild

flat_map
time:   [112.97 µs 113.22 µs 113.49 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

append
time:   [104.31 µs 104.48 µs 104.67 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

append_with_capacity
time:   [104.04 µs 104.41 µs 104.86 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
```


### Fourth run
```shell
for_loop_vec_new        
time:   [109.22 µs 109.86 µs 110.47 µs]

for_loop_vec_with_capacity
time:   [105.30 µs 105.73 µs 106.22 µs]
Found 8 outliers among 100 measurements (8.00%)
7 (7.00%) high mild
1 (1.00%) high severe

map
time:   [117.87 µs 118.57 µs 119.38 µs]
Found 5 outliers among 100 measurements (5.00%)
4 (4.00%) high mild
1 (1.00%) high severe

flat_map
time:   [114.99 µs 115.53 µs 116.10 µs]
Found 4 outliers among 100 measurements (4.00%)
3 (3.00%) high mild
1 (1.00%) high severe

append                  
time:   [104.74 µs 104.96 µs 105.20 µs]
Found 4 outliers among 100 measurements (4.00%)
4 (4.00%) high mild

append_with_capacity    
time:   [104.19 µs 104.53 µs 104.89 µs]
Found 2 outliers among 100 measurements (2.00%)
2 (2.00%) high mild
```