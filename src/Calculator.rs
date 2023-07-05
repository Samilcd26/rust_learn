pub mod taskOne{
    
    enum Operator {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    pub fn calculator(){
        println!("Hesap Makinesine Hoş Geldiniz!");
        println!("İşlem Seçenekleri:");
        println!("1. Toplama");
        println!("2. Çıkarma");
        println!("3. Çarpma");
        println!("4. Bölme");


        let operator = loop {
            let choice: u32 = match get_user_input("İşlem seçeneğini girin:").trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Geçersiz bir seçenek girdiniz. Lütfen tekrar deneyin.");
                    continue;
                }
            };


            
        if choice >= 1 && choice <= 4 {
            break match choice {
                1 => Operator::Add,
                2 => Operator::Subtract,
                3 => Operator::Multiply,
                4 => Operator::Divide,
                _ => unreachable!(),
            };
        } else {
            println!("Geçersiz bir seçenek girdiniz. Lütfen tekrar deneyin.");
        }


        let num1: f64 = loop {
            match get_user_input("Birinci sayıyı girin:").trim().parse() {
                Ok(num) => break num,
                Err(_) => println!("Geçersiz bir sayı girdiniz. Lütfen tekrar deneyin."),
            }
        };
    
        let num2: f64 = loop {
            match get_user_input("İkinci sayıyı girin:").trim().parse() {
                Ok(num) => break num,
                Err(_) => println!("Geçersiz bir sayı girdiniz. Lütfen tekrar deneyin."),
            }
        };
    
        if let Some(result) = calculate(operator, num1, num2) {
            println!("Sonuç: {}", result);
        } else {
            println!("Geçersiz bir işlem gerçekleştirdiniz!");
        }
    };

        
    }
    
    fn  calculate(operator: Operator, num1: f64, num2: f64) -> Option<f64> {
        match operator {
            Operator::Add => Some(num1 + num2),
            Operator::Subtract => Some(num1 - num2),
            Operator::Multiply => Some(num1 * num2),
            Operator::Divide => {
                if num2 != 0.0 {
                    Some(num1 / num2)
                } else {
                    None
                }
            }
        }
    }


    fn get_user_input(prompt: &str) -> String {
        use std::io::{self, Write};
        print!("{} ", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input
    }
    
}
   