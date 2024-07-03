# ref with ref vs params! macro with value vs params! macro with ref 
From reading the run results:
- For Strings: faster is with ref with ref (short and long)
- For Arrays: faster is with ref with ref (short and long)
- For Multiple parameters of different kinds: params! macro with values

### First run
```shell
params_macro_array/params_macro_value_array
time:   [310.56 ps 311.09 ps 311.79 ps]
change: [-3.2501% -1.4881% -0.4428%] (p = 0.03 < 0.05)
Change within noise threshold.
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe
params_macro_array/params_macro_ref_array
time:   [312.26 ps 314.46 ps 318.30 ps]
change: [+0.3286% +0.7218% +1.1417%] (p = 0.00 < 0.05)
Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
params_macro_array/ref_macro_ref_array
time:   [311.54 ps 312.79 ps 314.10 ps]
change: [-0.0663% +0.2675% +0.6204%] (p = 0.13 > 0.05)
No change in performance detected.
Found 19 outliers among 100 measurements (19.00%)
  4 (4.00%) high mild
  15 (15.00%) high severe

params_macro_array_long/params_macro_value_array_long
time:   [310.63 ps 311.84 ps 314.08 ps]
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) high mild
  13 (13.00%) high severe
params_macro_array_long/params_macro_ref_array_long
time:   [310.41 ps 310.60 ps 310.85 ps]
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) high mild
  10 (10.00%) high severe
params_macro_array_long/ref_macro_ref_array_long
time:   [310.62 ps 311.03 ps 311.52 ps]
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe

params_macro_short_string/params_macro_value_short_string
time:   [314.71 ps 316.01 ps 317.82 ps]
change: [+0.8158% +1.2197% +1.6845%] (p = 0.00 < 0.05)
Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
params_macro_short_string/params_macro_ref_short_string
time:   [314.74 ps 315.15 ps 315.56 ps]
change: [+1.2899% +1.4715% +1.6281%] (p = 0.00 < 0.05)
Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
params_macro_short_string/ref_macro_ref_short_string
time:   [313.14 ps 313.94 ps 314.73 ps]
change: [+1.0914% +1.3803% +1.7362%] (p = 0.00 < 0.05)
Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

params_macro_long_string/params_macro_value_long_string
time:   [314.16 ps 315.64 ps 318.32 ps]
change: [+0.9949% +1.4849% +2.3167%] (p = 0.00 < 0.05)
Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
params_macro_long_string/params_macro_ref_long_string
time:   [312.70 ps 313.45 ps 314.31 ps]
change: [+1.0065% +1.1996% +1.4245%] (p = 0.00 < 0.05)
Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
params_macro_long_string/ref_macro_ref_long_string
time:   [310.40 ps 311.10 ps 312.58 ps]
change: [-0.8887% -0.4936% +0.0240%] (p = 0.03 < 0.05)
Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

params_all/params_macro_value_all
time:   [310.34 ps 310.53 ps 310.84 ps]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe
params_all/params_macro_ref_all
  time:   [310.39 ps 310.99 ps 311.80 ps]
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
params_all/ref_macro_ref_all
  time:   [311.47 ps 312.20 ps 313.07 ps]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
```


### Second run
```shell
params_macro_array/params_macro_value_array
  time:   [310.40 ps 311.01 ps 311.96 ps]
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) high mild
  10 (10.00%) high severe
params_macro_array/params_macro_ref_array
  time:   [310.42 ps 310.95 ps 311.61 ps]
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe
params_macro_array/ref_macro_ref_array
  time:   [310.41 ps 310.74 ps 311.17 ps]
Found 17 outliers among 100 measurements (17.00%)
  3 (3.00%) high mild
  14 (14.00%) high severe

params_macro_array_long/params_macro_value_array_long
  time:   [310.34 ps 310.71 ps 311.20 ps]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe
params_macro_array_long/params_macro_ref_array_long
  time:   [310.38 ps 310.90 ps 311.63 ps]
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe
params_macro_array_long/ref_macro_ref_array_long
  time:   [310.28 ps 310.42 ps 310.63 ps]
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe

params_macro_short_string/params_macro_value_short_string
  time:   [312.78 ps 313.60 ps 314.55 ps]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
params_macro_short_string/params_macro_ref_short_string
  time:   [314.15 ps 314.76 ps 315.29 ps]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
params_macro_short_string/ref_macro_ref_short_string
  time:   [310.49 ps 310.74 ps 311.09 ps]
Found 15 outliers among 100 measurements (15.00%)
  8 (8.00%) high mild
  7 (7.00%) high severe

params_macro_long_string/params_macro_value_long_string
  time:   [311.65 ps 311.85 ps 312.13 ps]
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe
params_macro_long_string/params_macro_ref_long_string
  time:   [310.98 ps 311.19 ps 311.49 ps]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe
params_macro_long_string/ref_macro_ref_long_string
  time:   [310.76 ps 310.96 ps 311.25 ps]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

params_all/params_macro_value_all
  time:   [312.38 ps 313.42 ps 314.57 ps]
params_all/params_macro_ref_all
  time:   [312.05 ps 313.07 ps 314.21 ps]
Found 18 outliers among 100 measurements (18.00%)
  6 (6.00%) high mild
  12 (12.00%) high severe
params_all/ref_macro_ref_all
  time:   [312.62 ps 314.21 ps 316.85 ps]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
```

### Third run
```shell
params_macro_array/params_macro_value_array
  time:   [313.80 ps 314.60 ps 315.42 ps]
  change: [+0.8696% +1.1577% +1.4245%] (p = 0.00 < 0.05)
  Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
params_macro_array/params_macro_ref_array
  time:   [313.16 ps 313.82 ps 314.55 ps]
  change: [+0.5970% +0.8789% +1.1180%] (p = 0.00 < 0.05)
  Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe
params_macro_array/ref_macro_ref_array
  time:   [313.77 ps 316.19 ps 319.48 ps]
  change: [+3.2380% +6.9023% +10.529%] (p = 0.00 < 0.05)
  Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe

params_macro_array_long/params_macro_value_array_long
  time:   [314.61 ps 316.50 ps 319.09 ps]
  change: [+0.5589% +1.0316% +1.6476%] (p = 0.00 < 0.05)
  Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
params_macro_array_long/params_macro_ref_array_long
  time:   [313.82 ps 314.22 ps 314.67 ps]
  change: [+1.3851% +1.7242% +2.0640%] (p = 0.00 < 0.05)
  Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
params_macro_array_long/ref_macro_ref_array_long
  time:   [312.77 ps 313.36 ps 314.00 ps]
  change: [+0.9188% +1.1576% +1.3822%] (p = 0.00 < 0.05)
  Change within noise threshold.

params_macro_short_string/params_macro_value_short_string
  time:   [310.78 ps 311.27 ps 311.89 ps]
  change: [-1.1137% -0.7856% -0.4513%] (p = 0.00 < 0.05)
  Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
params_macro_short_string/params_macro_ref_short_string
  time:   [313.91 ps 314.45 ps 314.97 ps]
  change: [+0.2387% +0.4873% +0.7451%] (p = 0.00 < 0.05)
  Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
params_macro_short_string/ref_macro_ref_short_string
  time:   [312.90 ps 314.05 ps 315.73 ps]
  change: [-6.8949% -1.9036% +1.8514%] (p = 0.56 > 0.05)
  No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

params_macro_long_string/params_macro_value_long_string
  time:   [310.46 ps 310.95 ps 311.71 ps]
  change: [-0.5117% -0.1712% +0.2991%] (p = 0.42 > 0.05)
  No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe
params_macro_long_string/params_macro_ref_long_string
  time:   [315.29 ps 316.14 ps 317.07 ps]
  change: [+1.4569% +1.8648% +2.3132%] (p = 0.00 < 0.05)
  Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
params_macro_long_string/ref_macro_ref_long_string
  time:   [311.09 ps 311.69 ps 312.36 ps]
  change: [+0.3317% +0.6024% +0.8807%] (p = 0.00 < 0.05)
  Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

params_all/params_macro_value_all
  time:   [312.75 ps 314.68 ps 317.71 ps]
  change: [-1.6888% -1.1073% -0.4913%] (p = 0.00 < 0.05)
  Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
params_all/params_macro_ref_all
  time:   [310.61 ps 310.91 ps 311.31 ps]
  change: [-0.3430% +0.0174% +0.3658%] (p = 0.93 > 0.05)
  No change in performance detected.
Found 19 outliers among 100 measurements (19.00%)
  10 (10.00%) high mild
  9 (9.00%) high severe
params_all/ref_macro_ref_all
  time:   [312.43 ps 313.08 ps 313.80 ps]
  change: [-0.8375% -0.3270% +0.0993%] (p = 0.19 > 0.05)
  No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```

### Fourth run
```shell
params_macro_array/params_macro_value_array
  time:   [315.87 ps 317.85 ps 320.92 ps]
  change: [-0.3224% +0.1552% +0.6697%] (p = 0.59 > 0.05)
  No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
params_macro_array/params_macro_ref_array
  time:   [316.04 ps 317.80 ps 320.04 ps]
  change: [+0.8901% +1.4458% +1.9684%] (p = 0.00 < 0.05)
  Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
params_macro_array/ref_macro_ref_array
  time:   [310.69 ps 310.99 ps 311.36 ps]
  change: [-9.8401% -6.2984% -3.1236%] (p = 0.00 < 0.05)
  Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

params_macro_array_long/params_macro_value_array_long
  time:   [316.22 ps 318.59 ps 322.53 ps]
  change: [-0.0984% +0.7830% +1.5201%] (p = 0.06 > 0.05)
  No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
params_macro_array_long/params_macro_ref_array_long
  time:   [315.89 ps 318.58 ps 322.37 ps]
  change: [-0.1982% +0.4155% +1.1526%] (p = 0.27 > 0.05)
  No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
params_macro_array_long/ref_macro_ref_array_long
  time:   [312.06 ps 314.50 ps 318.05 ps]
  change: [-0.3992% +0.2590% +1.2792%] (p = 0.58 > 0.05)
  No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

params_macro_short_string/params_macro_value_short_string
  time:   [310.99 ps 312.42 ps 314.81 ps]
  change: [-0.3894% -0.0715% +0.3183%] (p = 0.71 > 0.05)
  No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) high mild
  11 (11.00%) high severe
params_macro_short_string/params_macro_ref_short_string
  time:   [310.33 ps 310.51 ps 310.80 ps]
  change: [-1.5052% -1.3747% -1.2334%] (p = 0.00 < 0.05)
  Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
params_macro_short_string/ref_macro_ref_short_string
  time:   [310.29 ps 310.44 ps 310.69 ps]
  change: [-2.3809% -1.6136% -1.1278%] (p = 0.00 < 0.05)
  Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

params_macro_long_string/params_macro_value_long_string
  time:   [310.50 ps 310.70 ps 310.99 ps]
  change: [-0.4890% -0.0132% +0.3919%] (p = 0.95 > 0.05)
  No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe
params_macro_long_string/params_macro_ref_long_string
  time:   [310.30 ps 310.44 ps 310.65 ps]
  change: [-2.5754% -2.1690% -1.8011%] (p = 0.00 < 0.05)
  Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
params_macro_long_string/ref_macro_ref_long_string
  time:   [310.70 ps 311.66 ps 313.24 ps]
  change: [-0.8085% -0.4447% -0.0658%] (p = 0.02 < 0.05)
  Change within noise threshold.
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) high mild
  11 (11.00%) high severe

params_all/params_macro_value_all
  time:   [310.30 ps 310.38 ps 310.47 ps]
  change: [-1.2428% -0.7855% -0.4388%] (p = 0.00 < 0.05)
  Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
params_all/params_macro_ref_all
  time:   [310.38 ps 310.97 ps 311.98 ps]
  change: [-0.8548% -0.5560% -0.2362%] (p = 0.00 < 0.05)
  Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe
params_all/ref_macro_ref_all
  time:   [312.23 ps 313.52 ps 315.10 ps]
  change: [-0.6688% -0.2509% +0.1931%] (p = 0.26 > 0.05)
  No change in performance detected.
Found 21 outliers among 100 measurements (21.00%)
  17 (17.00%) high mild
  4 (4.00%) high severe
```