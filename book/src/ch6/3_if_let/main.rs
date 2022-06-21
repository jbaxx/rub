#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Provincia {
    BocasDelToro,
    Cocle,
    // ...
}

#[allow(dead_code)]
enum Moneda {
    Penny,
    Nickel,
    Dime,
    Quarter(Provincia),
}


fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // We could write this in a shorter way
    // Basically it provides a quick way to do a single match arm
    let config_max = Some(3u8);
    if let Some(maximo) = config_max {
        println!("The maximum is configured to be {}", maximo)
    }

    // let coin = Moneda::Quarter(Provincia::Cocle);
    let coin = Moneda::Dime;
    let mut count = 0;
    match &coin {
        Moneda::Quarter(provin) => println!("Provincia de {:?}", provin),
        _ => count += 1,
    }
    println!("count is: {}", count);

    if let Moneda::Quarter(provin) = &coin {
        println!("Provincia de {:?}", provin);
    } else {
        count += 1;
    }
    println!("count is: {}", count);
}
