fn main() {
    println!("Merhaba, dünya!");
    ilk_fonksiyonum();
    degerli_fonksiyon(5, true);

    //-----------------------------------------------------
    // ifadeler : bakılacak
    let _x = 5;

    let y = {
        let _x = 3;
        _x + 1 // noktalı virgül olrusa ifade dönmez
    };

    println!("The value of y is: {}", y);

    //-----------------------------------------------------
    println!("Döndü valla : {}", donerse_bizimdir(7));
}

fn ilk_fonksiyonum() {
    println!("Bu benim ilk fonksiyonum");
}

fn degerli_fonksiyon(x: isize, y: bool) {
    println!("Gelen değerler : {} - {}", x, y);
}

fn donerse_bizimdir(x: isize) -> isize {
    x + 1
}
