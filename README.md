# rust-http-sandbox
Playing around with some rust web frameworks

### From command line
```
cargo run
```
### from Dockerfile
These are the basics but really you'll want to use docker-compose.
And then you'll need to manually build the schema and table (otherwise it won't
work)

```
docker build -t http-sandbox .
docker run -t -p 3000 http-sandbox
```

# Routes

You can find the routes in the routes.rs file but generally there are three
types of routes:

1. GET /hello/:word
1. POST /json -d '{"hello":"string value","world":1234}'
1. Resourse /persons/:id

Each was testing a different concept and building on the previous

1. Parsing url params
1. Processing json
1. A RESTful route

# Database

```sql
create schema http_sandbox.persons;
create table http_sandbox.persons (
  id SERIAL,
  first_name text,
  last_name text
);
```

# Performance

run as: cargo run --release

Run on a MBP 15"

### hello world route

```
$ ab -c 4 -n 10000 http://localhost:3000/hello/world
This is ApacheBench, Version 2.3 <$Revision: 1528965 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 1000 requests
Completed 2000 requests
Completed 3000 requests
Completed 4000 requests
Completed 5000 requests
Completed 6000 requests
Completed 7000 requests
Completed 8000 requests
Completed 9000 requests
Completed 10000 requests
Finished 10000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            3000

Document Path:          /hello/world
Document Length:        12 bytes

Concurrency Level:      4
Time taken for tests:   2.136 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      1140000 bytes
HTML transferred:       120000 bytes
Requests per second:    4682.04 [#/sec] (mean)
Time per request:       0.854 [ms] (mean)
Time per request:       0.214 [ms] (mean, across all concurrent requests)
Transfer rate:          521.24 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.0      0       0
Processing:     0    1   1.2      0      12
Waiting:        0    1   1.2      0      12
Total:          0    1   1.2      0      12

Percentage of the requests served within a certain time (ms)
  50%      0
  66%      0
  75%      1
  80%      1
  90%      2
  95%      4
  98%      5
  99%      6
 100%     12 (longest request)
```

### json route

```
$ ab -p test.json -c 4 -n 10000 http://localhost:3000/json                    
This is ApacheBench, Version 2.3 <$Revision: 1528965 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 1000 requests
Completed 2000 requests
Completed 3000 requests
Completed 4000 requests
Completed 5000 requests
Completed 6000 requests
Completed 7000 requests
Completed 8000 requests
Completed 9000 requests
Completed 10000 requests
Finished 10000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            3000

Document Path:          /json
Document Length:        46 bytes

Concurrency Level:      4
Time taken for tests:   2.178 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      1480000 bytes
Total body sent:        1720000
HTML transferred:       460000 bytes
Requests per second:    4592.09 [#/sec] (mean)
Time per request:       0.871 [ms] (mean)
Time per request:       0.218 [ms] (mean, across all concurrent requests)
Transfer rate:          663.70 [Kbytes/sec] received
                        771.33 kb/s sent
                        1435.03 kb/s total

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.0      0       1
Processing:     0    1   1.2      0      14
Waiting:        0    1   1.2      0      14
Total:          0    1   1.2      0      14

Percentage of the requests served within a certain time (ms)
  50%      0
  66%      1
  75%      1
  80%      1
  90%      2
  95%      4
  98%      5
  99%      6
 100%     14 (longest request)
```

### Index route for persons
There are a dozen persons in the db:

```
$ ab -c 4 -n 10000 http://localhost:3000/persons                                    
This is ApacheBench, Version 2.3 <$Revision: 1528965 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 1000 requests
Completed 2000 requests
Completed 3000 requests
Completed 4000 requests
Completed 5000 requests
Completed 6000 requests
Completed 7000 requests
Completed 8000 requests
Completed 9000 requests
Completed 10000 requests
Finished 10000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            3000

Document Path:          /persons
Document Length:        431 bytes

Concurrency Level:      4
Time taken for tests:   2.866 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      5340000 bytes
HTML transferred:       4310000 bytes
Requests per second:    3489.60 [#/sec] (mean)
Time per request:       1.146 [ms] (mean)
Time per request:       0.287 [ms] (mean, across all concurrent requests)
Transfer rate:          1819.77 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.1      0       3
Processing:     0    1   1.3      1      20
Waiting:        0    1   1.3      1      20
Total:          0    1   1.3      1      20

Percentage of the requests served within a certain time (ms)
  50%      1
  66%      1
  75%      1
  80%      1
  90%      2
  95%      4
  98%      6
  99%      7
 100%     20 (longest request)
```
