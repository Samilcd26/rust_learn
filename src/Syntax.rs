//Function
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn Variables() {
    let x: i32 = 42;
    
    //! Bellekte her değişken birtane tutulur eğer aydı değişkenden ikitane yapmak istersen clon yada ödüç al mak için & işaretini kullan
    let message: &str = "Hello, world!";
    //! Değerler rusta değiiştirilemez eğer sonradan deşitirilen bir veri istiyorsan maşına mut(arable) ekle
    let mut name = String::from("Alice");

    let numbers: [i32; 3] = [1, 2, 3];
    let second_number = numbers[1];

    let min_i32 = i32::MIN;
    let max_i32 = i32::MAX;

    let pi: f64 = 3.14159;

    let is_rust_fun: bool = true;

    let letter_a: char = 'a';
}

fn selecteds() {
    // Else if
    if x >= 0 {
        println!("x is non-negative");
    } else {
        println!("x is negative");
    }
}

fn loops() {
    // While
    let mut i = 1;
    while i <= 5 {
        println!("{}", i);
        i += 1;
    }



    let mut count = 0u32;
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }
    }
    //? OR
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };



    //For 
    for n in 1..=100 {
        
    }

}

fn str_functions() {
    let slice = &numbers[1..3];

}