// This is better approach (Counting Method)
fn main() {
    let mut nums = [2, 0, 2, 1, 1, 0];

    sort_arr(&mut nums);
    
    // Debug ({:?}) vs Display ({})
    println!("{:?}", nums);
}

fn sort_arr(nums: &mut [i32]) {

    let l = nums.len();
    
    // let mut zeroes, ones, twos :i32; syntax error 
    let (mut zeroes, mut ones, mut twos) = (0, 0, 0);
    

    // for &n in nums.iter() {
    //     match n {
    //         0 => zeroes += 1,
    //         1 => ones += 1,
    //         2 => twos += 1,
    //         _ => panic!("Unexpected number"),
    //     }
    // }

    for i in 0..l {
        if nums[i] == 0 {
            zeroes = zeroes + 1;
        } else if nums[i] == 1 {
            ones = ones + 1;
        } else {
            twos = twos + 1;
        }
    }
    
    ones = zeroes + ones;
    twos = ones  + twos;

    for i in 0..zeroes {
        nums[i] = 0;
    }
    for i in zeroes..ones {
        nums[i] = 1;
    }
    for i in ones..twos {
        nums[i] = 2;
    }


}