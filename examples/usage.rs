// linkedmask/examples/usage.rs

use linkedmask::LinkedMask;


fn main() {
    // every LinkedMask<T> grows to fit the biggest 1 bit
    {
        let mut lm = LinkedMask::<u8>::new();

        lm |= 1 << 26;

        println!("{lm}");
    }
    {
        let mut lm = LinkedMask::<u16>::new();

        lm |= 1 << 26;

        println!("{lm}");
    }
    {
        let mut lm = LinkedMask::<u32>::new();

        lm |= 1 << 26;

        println!("{lm}");
    }
}

