/*
The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143 ?
*/

fn main() {
    let m = fact(6008514751);
    println!("{}", m);
}

fn fact(num: usize) -> usize {
    let m: u32 = 0;
    let k = num as u32;
    if k%2 == 0 {
        m = k/2;
    }
    // let mut max_num: usize = 0;
    // //num_usize: u32 = num as u32;
    // for i in 1..num {
    //     if num % i == 0 {
    //         if i>max_num {
    //             max_num = i;
    //         }
    //     }
    // }
    // max_num
}
