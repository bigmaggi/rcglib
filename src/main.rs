use std::fs::File;
use std::io::Write;

static BACKGROUND_COLOR: [u8; 3] = [88, 26, 188];
static FOREGROUND_COLOR: [u8; 3] = [26, 188, 88];

//fn write_to_file(width: u32, height: u32, color: [u8; 3]) {
//    let file = File::create("output.ppm").expect("Unable to create file");
//    let mut writer = std::io::BufWriter::new(file);
//
//    // Write the PPM header
//    write!(writer, "P6\n{} {}\n255\n", width, height).expect("Unable to write to file");
//
//    // Write pixel data for each pixel
//    for i in 0..height {
//        for j in 0..width {
//            // Determine the color based on the current pixel's position
//            let pixel_color = if (i % 2 == 0 && j % 2 == 0) || (i % 2 != 0 && j % 2 != 0) {
//                FOREGROUND_COLOR // Green color for even positions
//            } else {
//                BACKGROUND_COLOR // Purple color for odd positions
//            };
//
//            // Write RGB values for each pixel
//            writer.write_all(&pixel_color).expect("Unable to write to file");
//        }
//    }
//}

static WIDTH: u32 = 256;
static HEIGHT: u32 = 256; 
fn put_pixel(p_image_matrix: &mut Vec<Vec<[u8; 3]>>, x: u32, y: u32, color: [u8; 3]) {
    // Write pixel data for each pixel
    let pixel_color = color;
    p_image_matrix[x as usize][y as usize] = pixel_color;

}

fn write_to_file(p_image_matrix: &mut Vec<Vec<[u8; 3]>>) {
    let file = File::create("output.ppm").expect("Unable to create file");
    let mut writer = std::io::BufWriter::new(file);

    // Write the PPM header
    write!(writer, "P6\n{} {}\n255\n", WIDTH, HEIGHT).expect("Unable to write to file");
    for x in 0..p_image_matrix.len()  {
        for y in 0..p_image_matrix[x].len() {
            writer.write_all(&p_image_matrix[x][y]).expect("Unable to write to file");
        }
    }
}

fn draw_line(p_image_matrix: &mut Vec<Vec<[u8; 3]>>) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if x == y {
                put_pixel(p_image_matrix, x, y, FOREGROUND_COLOR);
            }
        }
    }
}

fn draw_rectangle(p_image_matrix: &mut Vec<Vec<[u8; 3]>>, pixels_width: u32, pixels_height: u32,
                  x0: u32, y0: u32, color: [u8; 3]) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if x >= x0 && x <= x0 + pixels_width && y >= y0 && y <= y0 + pixels_height {
                put_pixel(p_image_matrix, x, y, color);
            }
        }
    }
}

fn draw_circle(p_image_matrix: &mut Vec<Vec<[u8; 3]>>, radius: u32, center_x: u32, center_y: u32, color: [u8; 3]) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            let distance = (dx * dx + dy * dy) as f32;

            if distance <= (radius * radius) as f32 {
                put_pixel(p_image_matrix, x, y, color);
            }
        }
    }
}

fn main() {
    //write_to_file(255, 255, [255, 0, 127]);
    let mut image_matrix: Vec<Vec<[u8; 3]>> = vec![vec![BACKGROUND_COLOR; WIDTH as usize]; HEIGHT as usize];

    // draw a circle
    //draw_circle(&mut image_matrix, 100, WIDTH/2, HEIGHT/2, FOREGROUND_COLOR);

    // draw a line
    //draw_line(&mut image_matrix);

    // draw a rectangle
    
    let rw = 100;
    let rh = 100;

    draw_rectangle(&mut image_matrix, rw, rh, WIDTH/2 - rw/2, HEIGHT/2-rh/2, FOREGROUND_COLOR);

    write_to_file(&mut image_matrix);
    println!("Image created successfully.");
}
