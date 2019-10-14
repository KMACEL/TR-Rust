fn main() {
    //  String Slices
    let s = String::from("merhaba dunya"); // eğer "ü" kulanırsan 1 byte daha artar. böylece 13 değil 14 yazman gerekecek

    let merhaba = &s[0..7]; // &s[..7]

    //let len = s.len();
    let dunya = &s[8..13]; // &s[8..]  &s[8..len]

    println!("{} {}", merhaba, dunya);

    //------------------------------------------------
    // Kelime bölme

    let get_first_word = first_word(&s);
    println!("İlk Kelime {}", get_first_word);

    //------------------------------------------------

    let a = [1, 2, 3, 4, 5];
    println!("Int Array {:#?}", a);

    let slice = &a[1..3];

    println!("Int Slice {:#?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
