# Ownership

Rust'ta bir garbage collection mekanizması yoktur. Ama, kullandığınız alanları geri bırakma zorunluluğu da yoktur. Farklı bir mekanizması vardır. Bu sayede daha hızlı ve güvenilir yazılımlar çıkartmaktadır.

Stack : son giren ilk çıkar mantığı ile çalışır. Boyutu sabittir. Heap'tan saha hızlıdır çünkü yeni bir alan aramak zorunda olmaz.
Heap : Boyhutu değişkendir. Verilere ulaşmak daha fazla zaman alır. Geri bir işaretçi döndürür. Veriler eulaşmak için işaretçiyi takip eder.

## Ownership Kuralları

* Rust'taki her değerin sahibi olarak adlandırılan bir değişkeni vardır.
* Bir seferde yalnızca bir sahip olabilir.
* Kapsam dışına çıktığında, değer düşecektir.

### Değişken Kapsamı

Var olduğu "{}" içinde bulunduğu sürece kapsam dahilindedir.
