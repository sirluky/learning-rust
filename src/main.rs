fn main() {
    // let mut x = 5;
    // x += 5;
    // let a: char = 'f';
    // let test: (i16, char) = (15, 'a');
    // let (_, text) = test;

    // println!("Hello, world {}! {:#?} {} {}", x, test, test.1, text);
    let cislo = 3;
    println!("Faktorial cisla {} je {}", cislo, faktorial(cislo))
}

fn faktorial(cislo: u16) -> u16 {
    let mut fcislo = 1;
    for x in 1..cislo + 1 {
        fcislo = fcislo * x;
    }
    return fcislo;
}
