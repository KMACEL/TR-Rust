fn main() {
    println!("Merhaba, dünya!");

    // Değişkenleri Yazdırma
    // let değişken oluşturmak için kullanılır
    // println makrosunun içindeki {}, değişkeni ekrana yazdırmak için kullanılır
    // "_" ile kelimelerin arasını ayırın
    let benim_guzel_degiskenim = 7;
    println!(
        "Benim Güzel Değişkenim İçindeki Sayı : : {}",
        benim_guzel_degiskenim
    );

    // let sabit bir değişken oluşturur aşağadıki komutu çalıştırırsanız şöyle bir hata alırsınız;
    // * cannot assign twice to immutable variable
    // benim_guzel_degiskenim = 12;

    // Eğer sabit değişkeni tekrar kullanmak isterseniz aşağıdaki gibi tekrardan "let" ile tanımlamanız gerekiyor
    let benim_guzel_degiskenim = 12;
    println!(
        "Benim Güzel Değişkenimi Tekrar Tanımladıktan Sonra İçindeki Sayı : : {}",
        benim_guzel_degiskenim
    );

    // Eğer sabit değilde, değişken bir değişken :) tanımlamak isterseniz let yanına mut ta eklemek gerek
    // fakat mut ekleyince eğer gerek yoksa "variable does not need to be mutable" uyarısı verir, ama kod çalışır. "Yabancı olsaydım 'It's cool' deerdim :D"
    let mut gercekten_degisken = 8;
    println!(
        "Gerçekten Değişen Değişkenimin İçindeki Sayı : : {}",
        gercekten_degisken
    );
    gercekten_degisken = 18;
    println!(
        "Gerçekten Değişen Değişkenimin İçindeki Yeni Sayı : : {}",
        gercekten_degisken
    );

    // Şimdi diyeceksiniz ki sabitler yokmu. Evet var ama kullanım amacı farklı. Sabitler gerçekten sabit.
    // Bir daha tanımlanamazlar, değiştirilemezler, tanımlarken tipi belli olması gerekir.
    const MAX_POINTS: u32 = 100_000;
    println!("Sabitin İçindeki Sayı : : {}", MAX_POINTS);

    // Birde Shadowing kavramı var (ismi çok iyi değil mi?)
    // Bu da "let" ile tanımlı, değişmeyen bir değişkeni, "let" ile tekrar tekrar kullanabilmeye imkan sağlıyor. Nasıl mı?
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("Shadowlu x : {}", x);

    // "let" ile tanımlamanın bir avantajı, tip kontrolü gerekmez. Mesela aşağıda aynı ismi hem string hem integer türünde tanımlanıyor.
    // Eğer mut ile değişken oluşturulsaydı hata alacaktı
    let spaces = "   ";
    let spaces = spaces.len();
}
