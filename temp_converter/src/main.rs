fn main() {
    convert_to_fran(0);
    convert_to_celc(-10);
}

fn convert_to_fran(degree: i32) {
    let franhereit = (degree * 9 / 5) + 32;

    println!("{}", franhereit)
}

fn convert_to_celc(degree: i32) {
    let celsius = (degree - 32) * 5 / 9;

    println!("{}", celsius)
}


//LEFT TO DO 
// ACCEPT USER INPUT