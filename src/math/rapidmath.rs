extern crate num;

use num_traits::NumOps;

/**
Maps a given number of a range onto another range.

# Arguments
* `value` - The original value to be mapped.
* `start1` - The original start value of the number range.
* `end1` - The original end value of the number range.
* `start2` - The new start value of the number range.
* `end2` - The new start value of the number range.

# Returns
A number containing the new mapped value.

# Examples
```
use lib_rapid::math;

let result: i32 = math::rapidmath::map(5., 0., 10., 0., 1.); // Original value 5 in the range from 0-10
std::println!("{}", result.to_string()) // Prints "0.5"
```
*/
pub fn map_to<T: NumOps + Copy>(value: T, start1: T, end1: T, start2: T, end2: T) -> T {

    (start2 + (end2 - start2)) * ((value - start1) / end1 - start1)
}

/**
Multiplies by 10 (shifts the decimal places to the left by 1) while being more efficient.

# Arguments
* `n` - The number to be multiplied by 10.

# Returns
The new shifted number.
*/
pub fn dec_lshift<T: std::ops::Add<Output = T> + std::ops::Shl<usize, Output = T> + Copy >(n: T) -> T {
    (n << 1) + (n << 3)
}