fn main() {
    let mut s = String::from("merhaba");
    s.push_str(", dünya!");
    println!("{}", s);
    // Eğer yukarıdaki gibi kullanılırsa main bitene kadar s yer kaplar. Fakat aşağıdaki gibi kullanılırsa işini yapar ve bitirir.

    {
        let mut s2 = String::from("merhaba");
        s2.push_str(", dünya!");
        println!("{}", s2);
    }

    //-------------------------------------------------------------------
    let s3 = String::from("merhaba");
    let s4 = s3;

    println!("{}", s4);

    // s3 ü, s4 e atadıktan sonra verilerini kopyalamaz. Ona aktarır. Eğer aşağıdaki gibi bir kullanım yaparsanız,
    //println!("{}", s3);
    // value borrowed here after move hatası alır. Bunun sebebi, verileri aktardığı için birinin kapsam alanı bittiğinde ve veri silindiğinde
    // diğeri ulaşmak istediği zaman öyle bir veri olmayacağı için hata alır. Rust bunu önlemek için derleme esnasında siler

    //-------------------------------------------------------------------
    // Eğer siz taşımak değilde, clone yapmak isterseniz, aşağıdaki gibi bir kullanım yapmanız gerekir
    let s5 = String::from("merhaba");
    let s6 = s5.clone();

    println!("s5 = {}, s6 = {}", s5, s6);

    //-------------------------------------------------------------------
    let s7 = String::from("merhaba");

    takes_ownership(s7); // burada taşındı.
                         // Yani eğer bu fonksiyondan sonra println!("{}",s); dersek yine hata verecektir.

    let s8 = 12;
    makes_copy(s8); // burada int türünde olduğu için kopyalandı. yani kullanılabilir
    println!("{}", s8);

    //-------------------------------------------------------------------
    // burada fonsiyondan döndüpğü için kulanabilir.
    let s9 = gives_ownership();
    println!("Gelen : {}", s9);

    let s10 = String::from("merhaba");
    let s11 = takes_and_gives_back(s10);
    println!("Gelen : {}", s10)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("merhaba");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
