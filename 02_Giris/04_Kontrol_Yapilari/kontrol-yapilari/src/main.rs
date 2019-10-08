fn main() {
    // if - else
    // ==, !=,<=,>=
    let number = 3;

    if number < 5 {
        println!("Sayı 5 ten küçük");
    } else if number == 5 {
        println!("Sayı 5 e eşit ");
    } else {
        println!("Sayı 5 ten  büyük");
    }

    //-----------------------------------------------------------------------
    // Let ile kullanım
    let durum = true;
    let number = if durum { 5 } else { 6 }; // geri dönüş değeri aynı tip olması gerekir

    println!("Sayı : {}", number);
}
