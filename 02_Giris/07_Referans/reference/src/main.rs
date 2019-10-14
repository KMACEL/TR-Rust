fn main() {
    let s1 = String::from("merhaba");

    let len = calculate_length(&s1);

    println!("Girilen değer : '{}' \nUzunluğu : {}.", s1, len);

    //------------------------------------------------------------------------------

    let mut s2 = String::from("merhaba");
    println!("Girilen İlk Değer : '{}'", s2);
    change(&mut s2);
    println!("Reference Gönderilip değiştikten sonraki değer : '{}'", s2);

    //------------------------------------------------------------------------------
    // Ancak değişken referansların büyük bir kısıtlaması vardır:
    //  belirli bir kapsamdaki belirli bir veri parçasına yalnızca bir değişken referansınız olabilir.
    /*
    Bu hata alır
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
    */

    let mut s3 = String::from("merhaba");

    {
        s3.push_str(", dünya 1");
        let r1 = &mut s3;
        println!("{}", r1);
        s3.push_str(", dünya 2");
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    s3.push_str(", dünya 3");
    let r2 = &mut s3;
    println!("{}", r2);
    //------------------------------------------------------------------------------

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 //let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}", r1, r2);

    //------------------------------------------------------------------------------
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    // eğer değeri değiştirmeye çalışırsak hata verir. Çünkü let ile tanımladığımız için buna hakkı yok.
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", dünya");
}
