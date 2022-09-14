fn main() {
    let height = 256;
    let width = 256;

    println!("P3\n {width} {height}\n256");

    for j in 0..height {
        eprint!("\rScanlines remaining: {j}");
        for i in 0..width {
            let red = i as f64 / (width - 1) as f64;
            let green = j as f64 / (height - 1) as f64;
            let blue = 0.25;

            let red = (255.999 * red) as i32;
            let green = (255.999 * green) as i32;
            let blue = (255.999 * blue) as i32;

            println!("{red} {green} {blue}");
        }
    }
    eprintln!("\rDone!                       ");
}
