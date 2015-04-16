# pglinearmodel

This utility runs a query on a postgres database once a minute. The
query should return a single row with two floating point numbers. The
first number is taken as an X value and the second as a Y. As the
program runs it accumulates a random sampling of these X and Y
values and finds a line of best to the sample, printing out
information about the line in a tab delimited format:

```
time    recent-x        recent-y        x-intercept     y-intercept    m
2015-04-16T18:19:05Z    1429208345.232872963    0.0    1429197370.2938756943   28658.7076769402        -0.0000200523
2015-04-16T18:20:05Z    1429208405.2388079166   0.0    1429197370.2938756943   28658.7076769402        -0.0000200523
2015-04-16T18:21:05Z    1429208465.2451839447   0.0    1429197370.2938756943   28658.7076769402        -0.0000200523
2015-04-16T18:22:05Z    1429208525.2512218952   0.0    1429197370.2938756943   28658.7076769402        -0.0000200523
2015-04-16T18:23:05Z    1429208585.257461071    0.0    1429197370.2938756943   28658.7076769402        -0.0000200523
```

The headers are reprinted every 20 lines.

## Usage

An example usage would look something like this:

```
PG_QUERY="SELECT CAST(date_part('epoch', NOW()) AS float) AS date, CAST(COUNT(id) AS float) AS number FROM some_table"
PG_URL="postgresql://kevin@localhost/somedatabase"
./pglinearmodel
```

With this example usage X is going to be time values, and Y is going
to be the count of rows in `some_table`. Then the best fit line can be
used to estimate future counts of rows in some_table. The estimates
will be all over the place if the the relationship between X and Y is
non-linear.

The url in PG_URL follows the url syntax used by
[rust-postgres](https://github.com/sfackler/rust-postgres).
