fn main(){
    // let test_b: bool = true;
    // let test_c: char = 's';
    
    print!("{} ", test(10));
    println!("hello");
}   

fn test(i: usize) -> usize {
    i*2
}