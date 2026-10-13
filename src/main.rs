use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Point {
    x: f32,
    y: f32,
    w: f32,
}

#[derive(Debug)]
enum DataReadError {
    FileOpenError(()),
    ReadError(()),
}


fn parse_dat(path: &str) -> Result<Vec<Point>, DataReadError> {
    println!("Unesite ime .dat fajla iz {:?} ", path);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Greska pri citanju");

    let path_dat = path.to_string() + input.trim();
    
    println!("{:?}", path_dat);


    let file = File::open(path_dat).map_err(|_| DataReadError::FileOpenError(()))?;
    let reader = BufReader::new(file);

    let mut points = Vec::new();

    let mut skip_next = false;
    let mut reading = false;

    for line in reader.lines() {
        let line = line.map_err(|_| DataReadError::ReadError(()))?;
        if skip_next {
            skip_next = false;
            reading = true;
            continue;
        }
        if line.contains("param") {
            skip_next = true;
            continue;
        }
        if line.contains(";") && reading {
            break;
        }

        if reading {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let x: f32 = parts[1].parse().unwrap_or(0.0);
                let y: f32 = parts[2].parse().unwrap_or(0.0);
                let w: f32 = parts[3].parse().unwrap_or(0.0);
                let point = Point { x, y, w };
                points.push(point);
            }
        }
    }
    Ok(points)
}

fn weiszfeld(points: &Vec<Point>, eps: f32) -> (f32, f32) {
    if let Some(p) = init_check(points) {
        return (points[p].x, points[p].y);
    }
    else {
        
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
                let dist = (dx * dx + dy * dy).sqrt().max(eps); // avoid division by zero
                num_x += (p.w * p.x) / dist;
                num_y += (p.w * p.y) / dist;
                denom += p.w / dist;
            }

            let pom_x = num_x / denom;
            let pom_y = num_y / denom;

            if (pom_x - x).abs().max((pom_y - y).abs()) < eps {
                x = pom_x;
                y = pom_y;
                break;
            }

            x = pom_x;
            y = pom_y;
        }
        return (x, y);
    }
}

fn weiszfeld_n(points: &Vec<Point>, n: usize) -> (f32, f32) {
    if let Some(p) = init_check(points) {
        return (points[p].x, points[p].y);
    } else {
        let sum_w: f32 = points.iter().map(|p| p.w).sum();
        let mut x = points.iter().map(|p| p.w * p.x).sum::<f32>() / sum_w;
        let mut y = points.iter().map(|p| p.w * p.y).sum::<f32>() / sum_w;

        for _ in 0..n {
            let mut num_x = 0.0;
            let mut num_y = 0.0;
            let mut denom = 0.0;

            for p in points {
                let dx = p.x - x;
                let dy = p.y - y;
                let dist = (dx * dx + dy * dy).sqrt().max(1e-6); // ili neka mala vrednost
                num_x += (p.w * p.x) / dist;
                num_y += (p.w * p.y) / dist;
                denom += p.w / dist;
            }

            x = num_x / denom;
            y = num_y / denom;
        }
        return (x, y);
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


fn main() -> Result<(), DataReadError> {
    let points = parse_dat("/home/djolekap/Desktop/ampl/Modeli/")?;

    for point in &points {
        println!("Point: x={}, y={}, w={:?}", point.x, point.y, point.w);
    }

    let (x, y) = weiszfeld(&points, 0.02);
    println!("Prihvatamo tacku ({},{}) kao resenje", x, y);

    Ok(())
}
