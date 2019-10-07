fn main() {
    // Değişkenler tip belirtmedende kullnaılabilir

    let a = 5;
    let b = 5.12;
    let c = true;
    let d = "Merhaba";

    println!(
        "Değişken Tipleri \n\tIntger : {}, \n\tFloat : {} \n\tBoolean : {}, \n\tString : {} \n\n",
        a, b, c, d
    );

    // Integer
    // 8-bit	i8  	u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64  	u64
    // 128-bit	i128	u128
    // arch	    isize	usize eğer bilgisayarınız 32 bit se 32, 64 bitse 64

    // Tam sayıların aldığı değer tipleri
    // Decimal 	        98_222
    // Hex  	        0xff
    // Octal        	0o77
    // Binary	        0b1111_0000
    // Byte (u8 only)	b'A'

    // _ görsellik için yazılır. yani 1_000 ile 1000 aynıdır

    let x: i64 = 32_000_000_000_000_000;
    println!("i64 Türünde değişken içindeki veri : {}", x);

    //---------------------------------------------------------------------------

    // Float
    // f32
    // f64
    let ondalik: f32 = 5.12;
    println!("f32 Türünde değişken içindeki veri : {}", ondalik);

    //---------------------------------------------------------------------------
    let bool_test: bool = false; // with explicit type annotation
    println!("bool Türünde değişken içindeki veri : {}", bool_test);

    //---------------------------------------------------------------------------
    // Eğer ' kullanılırsa karakter, " kullanırsa karakter dizi
    let char_1 = 'z';
    let char_2 = 'ℤ';
    let char_3 = '😻';
    println!("Character Test\n\t{}\n\t{}\n\t{}", char_1, char_2, char_3);

    //---------------------------------------------------------------------------
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup Test\n\t{:?}", tup);
    let (x, y, z) = tup;
    println!("tup aktarma\n\t{}\n\t{}\n\t{}", x, y, z);
    println!(
        "tup tek tek kullanımı\n\t{}\n\t{}\n\t{}",
        tup.0, tup.1, tup.2
    );

    //---------------------------------------------------------------------------
    // Diziler
    let a = [1, 2, 3, 4, 5];
    println!("dizi Test\n\t{:?}", a);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("dizi tanımlamalı Test\n\t{:?}", a);

    let a = [3; 7];
    println!("dizi başlangıç değerli Test\n\t{:?}", a);

    let a = [3; 7];
    println!("dizi ilk değeri değerli Test\n\t{}", a[0]);
}
