fn test() {
    let v = vec![1, 2, 3];

    let v2 = v;

    //! Error
    println!("v[0] is: {} ", v[0]);


    let t1 = "Test Data";
    let t2 = t1.clone();
    print!("Method One : {}",t2);


    let m1 = String::from("Test Data");
    let m2 = &m1;
    print!("Method One : {}",t2);
}
