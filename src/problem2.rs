pub fn fibonacci_seq(bounds:i32)
{
    let mut y = 0;
    let mut z = 1;
    let mut sum = 0;

    for x in 0..bounds
    {
        sum = y + z;

        println!("{}",sum);
        y = z;
        z = sum;
    }

}

        
    
