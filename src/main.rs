extern crate postgres;
extern crate rand;
extern crate time;
extern crate csv;

use postgres::{Connection, SslMode};
use std::env::{var_os};
use time::{precise_time_ns, now_utc};
use rand::{random};
use std::cmp::{min};
use std::thread::{sleep_ms};
use std::io::{stdout,Write};
use csv::Writer;

struct RingBuffer {
    d: Vec<(f64, f64)>,
    size: usize,
    pos:usize
}

fn update (r: RingBuffer, value: (f64, f64)) -> RingBuffer {
    let p = r.pos % r.size;
    if r.d.len() > p {
        let mut d = r.d.clone();
        d[p] = value;
        let d1 = d;
        RingBuffer{d:d1,size:r.size,pos:p+1}
    } else {
        let mut d = r.d.clone();
        d.push(value);
        let d1 = d;
        RingBuffer{d:d1,size:r.size,pos:p+1}
    }
}

fn pair_wise_mean (v: &Vec<(f64, f64)>) -> (f64, f64) {
    let mut xsum = 0.0f64;
    let mut ysum = 0.0f64;
    for i in 0..v.len() {
        xsum = xsum + v[i].0;
        ysum = ysum + v[i].1;
    }
    (xsum/(v.len() as f64), ysum/(v.len() as f64))
}

fn least_squares_line_of_best_fit (pairs: Vec<(f64, f64)>) -> (f64, f64) {
    let ref p = pairs;
    let means = pair_wise_mean(p);
    let mut t = 0f64;
    let mut u = 0f64;
    let l = p.len();
    for i in 0..l {
        t = ((pairs[i].0 - means.0) * (pairs[i].1 - means.1)) + t;
        u = ((pairs[i].0 - means.0) * (pairs[i].0 - means.0)) + u;
    }
    let m = t/u;
    let b = means.1 - (m * means.0);
    (m, b)
}

fn x_intercept (m: f64, b: f64) -> f64 {
    (0f64 - b) / m
}

fn main () {
    match (var_os("PG_URL"), var_os("PG_QUERY"))  {
        (Some(url), Some(query)) => {
            let conn = Connection::connect(url.to_str().unwrap(), &SslMode::None).unwrap();
            let stmt = conn.prepare(query.to_str().unwrap()).unwrap();
            let mut r = RingBuffer{d:Vec::new(), size: 100, pos: 0};
            let mut i = 0;
            loop {
                let mut w = Writer::from_memory().delimiter(b'\t');
                if (i % 20) == 0 {
                    w.encode(("time", "recent-x", "recent-y", "x-intercept", "y-intercept", "m")).ok().unwrap();
                }
                for row in stmt.query(&[]).unwrap() {
                    let x = row.get(0);
                    let y = row.get(1);
                    r = update(r, (x,y));
                    if i > 2 {
                        let (m, b) = least_squares_line_of_best_fit(r.d.clone());
                        w.encode((now_utc().rfc3339().to_string(), x, y, x_intercept(m, b), b, m)).ok().unwrap();
                    }
                };
                stdout().write_all(w.as_bytes()).ok().unwrap();
                sleep_ms(1000*60);
                i = i + 1
            }
        },
        _ => println!("Be sure to define PG_URL and PG_QUERY")
    }
}
