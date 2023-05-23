use std::time::Instant;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn lagrange_interpolation(x: f64, points: &[Point]) {
    let start_iterative = Instant::now();
    let n = points.len();
    let mut result = 0.0;

    for i in 0..n {
        let mut term = points[i].y;
        for j in 0..n {
            if i != j {
                term *= (x - points[j].x) / (points[i].x - points[j].x);
            }
        }
        result += term;
    }
    println!("iterative result: {}, time consumed ms: {}", result, start_iterative.elapsed().as_millis());
}

pub fn parallel_lagrange_interpolation(x: f64, points: &[Point]) {
    let n = points.len();
    let num_threads = 2;
    let chunk_size = n / num_threads;

    crossbeam::thread::scope(|s| {
        let start_paralel = Instant::now();
        let mut handlers = Vec::with_capacity(num_threads);

        for step in 0..num_threads {
            let handle : crossbeam::thread::ScopedJoinHandle<f64> = s.spawn(move |_| {
                let mut result = 0.0;
                for i in step * chunk_size..(step + 1) * chunk_size {
                    let mut term = points[i].y;
                    for j in 0..n {
                        if i != j {
                            term *= (x - points[j].x) / (points[i].x - points[j].x);
                        }
                    }
                    result += term;        
                }
                result
            });
            handlers.push(handle);
        }    
        let result : f64 = handlers.into_iter().map(|x| x.join().unwrap()).sum();
        println!("result: {}, time consumed ms: {}", result, start_paralel.elapsed().as_millis());

    }).unwrap();
}

fn main() {
    let n = 1 << 14;
    println!("N: {}", n);
    let mut points : Vec<Point> = Vec::new();
    for i in 0..n {
        points.push(Point {x: i as f64, y: (i as f64).cos()});
    }

    let x = 0.0f64;
    lagrange_interpolation(x, &points);
    parallel_lagrange_interpolation(x, &points);
}