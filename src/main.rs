use std::io;

fn to_celcius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.) * (5. / 9.);
}

fn to_fahrenheit(celcius: f64) -> f64 {
    return celcius * (9. / 5.) + 32.;
}

const SCALES: [&'static str; 2] = ["celcius", "fahrenheit"];

fn main() {
    let scale = loop {
        println!("Select temperature scale to convert from.");

        let mut scale = String::new();
        io::stdin()
            .read_line(&mut scale)
            .expect("Couldn't select temperature scale.");
        scale = scale.trim().to_lowercase();

        if SCALES.contains(&scale.as_str()) {
            break scale;
        }
    };

    let value = loop {
        println!("Enter temperature value.");

        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Couldn't get temperature value.");

        match value.trim().parse() {
            Ok(value) => break value,
            Err(_) => continue,
        }
    };

    match scale.as_str() {
        "celcius" => println!("{value} °C is {} °F", to_fahrenheit(value)),
        "fahrenheit" => println!("{value} °F is {} °C", to_celcius(value)),
        &_ => panic!(),
    };
}
