fn main() {
    // loop
    let mut a: isize = 0;

    loop {
        a += 1;
        println!("Merhaba Ben tam bir loop döngüsüyüm {}", a);
        if a == 5 {
            break;
        }
    }

    //-----------------------------------------------------------------------
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    //-----------------------------------------------------------------------
    // while
    let mut sayi = 3;

    while sayi != 0 {
        println!("Ben bir while döngüsüyüm {}", sayi);

        sayi -= 1;
    }

    //-----------------------------------------------------------------------
    // for
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("Ben dizili for'um: {}", element);
    }
    //-----------------------------------------------------------------------
    for number in (1..4).rev() {
        println!("Bu da geri for : {}", number);
    }
    //-----------------------------------------------------------------------
    for number in 1..4 {
        println!("Bu da düz for 2 : {}", number);
    }
}
