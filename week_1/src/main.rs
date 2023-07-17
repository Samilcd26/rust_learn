
//? This function calculate to text length
fn calculate_length(s: &String) -> usize {
    s.len()
}


fn main() {
    let test_data = String::from("Test Data");
   
    let len: usize = calculate_length(&test_data); 

    println!("The length of {} is {}.", test_data, len);

}


