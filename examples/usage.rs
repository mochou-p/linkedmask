// linkedmask/examples/usage.rs

use linkedmask::LinkedMask;


fn main() {
    // every LinkedMask<T> grows to fit the biggest 1 bit
    {
        let mut lm = LinkedMask::<u8>::new();

        lm |= (1 << 26) + (1 << 9) + 1;

        println!("{lm}");
    }
    {
        let mut lm = LinkedMask::<u16>::new();

        lm |= (1 << 26) + (1 << 9) + 1;

        println!("{lm}");
    }
    {
        let mut lm = LinkedMask::<u32>::new();

        lm |= (1 << 26) + (1 << 9) + 1;

        println!("{lm}");
    }
    {
        let mut lm = LinkedMask::<u8>::new();

        lm |= ((1 << 9) + 1) << 1;

        println!("{lm}");
    }

    // even any standard integer overflowing bit
    {
        let mut lm = LinkedMask::<u8>::new();

        // functional equivalent of `lm |= 1 << 128` if it was possible
        lm.bitor_assign_one_shl(128);

        lm.bitor_assign_n_shl((1 << 9) + 1, 0);

        println!("{lm}");
    }
}

