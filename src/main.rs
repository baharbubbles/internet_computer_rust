fn main() {
    let message="Hello, World!";
    // x-değişken_adı: i32-değişken_tipi = 42-değişkenin_değeri
    let x: i32 = 42;
    let pi: f64 = 3.14;
    let is_rust_fun: bool = true;
    let letter_a: char = 'a';

    // fonksiyon oluşturma
    // add isimli fonksiyona i32 titpnde iki parametre eklendi, "-> i32" de fonksiyonun return'ü
    fn add(x: i32, y: i32) -> i32 {
        x+y
    }
    
    // karşılaştırma yapma
    let x = 42; //x'e değer verildi
    if x >= 0 {
        println!("x is non-negative");  // println! şeklinde yazılıyor dikkat!!!
    }
    else{
        println!("x is negative");
    }

    // döngü yapma
    let mut i = 1; //mutable yani güncellenebilen değişken old. belirTiyor
    while i <= 5 {
        println!("{}", i);
        i += 1;
    }

}
// ** genel olarak syntax yapısı javascript ve c diline çok benzer **
