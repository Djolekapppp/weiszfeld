use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

struct Config {
    input: String,
    epsilon: f32,
    max_iterations: usize,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let input = args[1].clone();

        let epsilon = if let Some(index) = args.iter().position(|x| x == "--epsilon") {
            if index + 1 < args.len() {
                match args[index + 1].parse() {
                    Ok(val) => val,
                    Err(_) => return Err("Invalid value for --epsilon"),
                }
            } else {
                return Err("Missing value for --epsilon");
            }
        } else {
            0.01
        };

        let max_iterations = if let Some(index) = args.iter().position(|x| x == "--max-iter") {
            if index + 1 < args.len() {
                match args[index + 1].parse() {
                    Ok(val) => val,
                    Err(_) => return Err("Invalid value for --max-iter"),
                }
            } else {
                return Err("Missing value for --max-iter");
            }
        } else {
            100
        };

        Ok(Config {
            input,
            epsilon,
            max_iterations,
        })
    }
}

struct WeiszfeldResult {
    x: f32,
    y: f32,
    iterations: usize,
    converged: SolveStatus,
}

#[derive(Debug)]
enum SolveStatus {
    Converged,
    MaxIterationsReached,
    InitialPointFound,
}

struct Point {
    x: f32,
    y: f32,
    w: f32,
}

fn parse_dat(path: &str) -> Result<Vec<Point>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut points = Vec::new();

    let mut skip_next = false;
    let mut reading = false;

    for line in reader.lines() {
        let line = line?;
        if skip_next {
            skip_next = false;
            reading = true;
            continue;
        }
        if line.contains("param:") {
            skip_next = true;
            continue;
        }
        let parse_error_msg = format!("Failed to parse line: {}", line);

        if reading {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let x: f32 = parts[1].parse().map_err(|_| parse_error_msg.clone())?;
                let y: f32 = parts[2].parse().map_err(|_| parse_error_msg.clone())?;
                let w: f32 = parts[3].parse().map_err(|_| parse_error_msg.clone())?;
                let point = Point { x, y, w };
                points.push(point);
            }
        }
        if line.contains(";") && reading {
            break;
        }
    }
    Ok(points)
}

fn weiszfeld(points: &Vec<Point>, eps: f32, max_iter: usize) -> WeiszfeldResult {
    if let Some(p) = init_check(points) {
        return WeiszfeldResult {
            x: points[p].x,
            y: points[p].y,
            iterations: 0,
            converged: SolveStatus::InitialPointFound,
        };
    }
    else {
        let mut n = 0;
        
        let sum_w: f32 = points.iter().map(|p| p.w).sum();
        let mut x = points.iter().map(|p| p.w * p.x).sum::<f32>() / sum_w;
        let mut y = points.iter().map(|p| p.w * p.y).sum::<f32>() / sum_w;

        loop {
            let mut num_x = 0.0;
            let mut num_y = 0.0;
            let mut denom = 0.0;

            for p in points {
                let dx = p.x - x;
                let dy = p.y - y;
                let dist = (dx * dx + dy * dy).sqrt().max(1e-6); // avoid division by zero
                num_x += (p.w * p.x) / dist;
                num_y += (p.w * p.y) / dist;
                denom += p.w / dist;
            }

            let pom_x = num_x / denom;
            let pom_y = num_y / denom;

            if (pom_x - x).abs().max((pom_y - y).abs()) < eps || n >= max_iter {
                x = pom_x;
                y = pom_y;
                break;
            }

            x = pom_x;
            y = pom_y;
            n += 1;
        }
        
        return WeiszfeldResult {
            x,
            y,
            iterations: n,
            converged: if n >= max_iter { SolveStatus::MaxIterationsReached } else { SolveStatus::Converged },
        };
    }
}

fn init_check(points: &Vec<Point>) -> Option<usize> {
    for (r_idx, r) in points.iter().enumerate() {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        for (i_idx, i) in points.iter().enumerate() {
            if i_idx == r_idx {
                continue;
            }
            let dx = r.x - i.x;
            let dy = r.y - i.y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist != 0.0 {
                sum_x += i.w * dx / dist;
                sum_y += i.w * dy / dist;
            }
        }
        let result = (sum_x * sum_x + sum_y * sum_y).sqrt();
        if result <= r.w {
            return Some(r_idx);
        }
    }

    return None;
}

fn objective_function(points: &Vec<Point>, x: f32, y: f32) -> f32 {
    points.iter().map(|p| p.w * ((p.x - x).powi(2) + (p.y - y).powi(2)).sqrt()).sum()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args();
    let config = Config::new(&args.collect::<Vec<String>>())?;

    let start = Instant::now();

    let points = parse_dat(&config.input)?;
    let load_time = start.elapsed();

    if points.is_empty() {
        return Err("Input file contains no points".into());
    }

    let algorithm_start = Instant::now();

    let result = weiszfeld(
        &points,
        config.epsilon,
        config.max_iterations,
    );

    let algorithm_time = algorithm_start.elapsed();

    let objective = objective_function(&points, result.x, result.y);

    let total_time = start.elapsed();

    println!();
    println!("========== Weber Problem ==========");
    println!("Input file:       {}", config.input);
    println!("Points loaded:    {}", points.len());
    println!("Epsilon:          {}", config.epsilon);
    println!("Max iterations:   {}", config.max_iterations);
    println!("Iterations:       {}", result.iterations);
    println!("Status:           {:?}", result.converged);

    println!("-----------------------------------");
    println!("Optimal point:    ({:.6}, {:.6})", result.x, result.y);
    println!("Objective value:  {:.6}", objective);
    println!("-----------------------------------");
    println!("File loading:     {:.3} ms", load_time.as_secs_f64() * 1000.0);
    println!("Algorithm time:   {:.3} ms", algorithm_time.as_secs_f64() * 1000.0);
    println!("Total time:       {:.3} ms", total_time.as_secs_f64() * 1000.0);
    println!("===================================");

    Ok(())
}
