# Performance Metrics Calculation

- `pidstat` cli utility is used to calculate the performance metrics

- Receive Stats

```
$ pidstat  -d -r -s -u 1 -e ./target/debug/tcp-ft recv

Average:      UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
Average:     1000    435348    7.15    6.79    0.00    0.06   13.95     -  tcp-ft

Average:      UID       PID  minflt/s  majflt/s     VSZ     RSS   %MEM  Command
Average:     1000    435348      9.52      0.00    6742    3971   0.00  tcp-ft

Average:      UID       PID StkSize  StkRef  Command
Average:     1000    435348     132      75  tcp-ft

Average:      UID       PID   kB_rd/s   kB_wr/s kB_ccwr/s iodelay  Command
Average:     1000    435348      0.00  27654.59      0.00       0  tcp-ft
```

- Transmit Stats

```
$ pidstat  -d -r -s -u 1 -e ./target/debug/tcp-ft send --recv-addr 127.0.0.1:8080 --file ./tmp/c.txt

Average:      UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
Average:     1000    435369   98.59    1.31    0.00    0.06   99.91     -  tcp-ft

Average:      UID       PID  minflt/s  majflt/s     VSZ     RSS   %MEM  Command
Average:     1000    435369      8.93      0.00    6592    4540   0.00  tcp-ft

Average:      UID       PID StkSize  StkRef  Command
Average:     1000    435369     308     308  tcp-ft

Average:      UID       PID   kB_rd/s   kB_wr/s kB_ccwr/s iodelay  Command
Average:     1000    435369      0.00     80.20      0.00       0  tcp-ft
```

<!-- end of file -->
