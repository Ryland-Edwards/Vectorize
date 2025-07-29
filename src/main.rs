use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse_lidar_data(file_path: &str) -> io::Result<Vec<f64>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    // Process each line
    for line in reader.lines() {
        let line = line?;

        // Extract the number from the line by removing brackets and commas
        // Format is like: "[4.5563],"
        if let Some(value_str) = line
            .replace("[", "")
            .replace("]", "")
            .replace(",", "")
            .trim()
            .parse::<String>()
            .ok()
        {
            // Parse the string to f64
            if let Ok(value) = value_str.parse::<f64>() {
                data.push(value);
            }
        }
    }

    Ok(data)
}
//might have issue with chunking setting points off by 5 degrees
fn grouped_averages(scans: &Vec<f64>, group_size: usize) -> Vec<f64> {
        scans
        .chunks(group_size)
        .map(|chunk| {
            let sum: f64 = chunk.iter().sum();
            let average = sum / chunk.len() as f64;
            (average * 10000.0).round() / 10000.0
        })
        .collect()
}


fn generate_lidar_points(distances: &Vec<f64>) -> Vec<(f64, f64)> {
    let start_angle = 225.0; // Starting angle in degrees
    let end_angle = -45.0;   // Ending angle in degrees
    let increment = 5.0;     // Increment in degrees

    let mut points = Vec::new();
    let mut angle:f64 = start_angle;
    let mut distance_index = 0;

    while angle >= end_angle && distance_index < distances.len() {
        // Convert angle to radians for trigonometric functions
        let angle_rad = angle.to_radians();

        // Get the distance for this angle
        let distance = distances[distance_index];

        // Calculate x and y coordinates
        // x = distance * cos(angle), y = distance * sin(angle)
        let x = distance * angle_rad.cos();
        let y = distance * angle_rad.sin();

        // Round to 4 decimal places for cleaner output
        let x_rounded = (x * 10000.0).round() / 10000.0;
        let y_rounded = (y * 10000.0).round() / 10000.0;

        points.push((x_rounded, y_rounded));

        angle -= increment;
        distance_index += 1;
    }

    points
}




fn main() {
    match parse_lidar_data("src/scans") {
        Ok(data) => {
            println!("Successfully parsed {} data points", data.len());
            //println!("{:?}", data);

            // Example: Print the first 10 values
            // println!("First 10 values:");
            // for (i, &value) in data.iter().take(10).enumerate() {
            //     println!("  {}: {}", i, value);
            // }

            // Now you can process this data for your original vector field calculation
            let group_size = 10;
            let groups = grouped_averages(&data, group_size);
            //vectors need to be (f64,f64,f64,f64) :: (x1,y1,x2,y2), for first point in vector to second point in vector since origin is not (0,0) as that is the center of the robot
            //can use the angle of lidar scanner to determine direction of vector
            println!("Created groups {:?}", groups);

            let points = generate_lidar_points(&groups);
            println!("Created points {:?}", points);

            for (i, (x, y)) in points.iter().enumerate() {
                println!("  Point {}: ({}, {})", i, x, y);
            }

        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

//-0.81915204
//-0.57357644


