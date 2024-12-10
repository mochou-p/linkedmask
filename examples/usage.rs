// linkedmask/examples/usage.rs

use linkedmask::LinkedMask;


fn main() {
    // every LinkedMask<T> grows to fit the biggest 1 bit
    {
        let mut lm = LinkedMask::<u8>::default();

        lm |= (1 << 26) + (1 << 9) + 1;

        println!("{lm}");
    }
    {
        let mut lm = LinkedMask::<u16>::default();

        lm |= (1 << 26) + (1 << 9) + 1;

        println!("{lm}");
    }
    {
        let mut lm = LinkedMask::<u32>::default();

        lm |= (1 << 26) + (1 << 9) + 1;

        println!("{lm}");
    }
    {
        let mut lm = LinkedMask::<u8>::default();

        lm |= ((1 << 9) + 1) << 1;

        println!("{lm}");
    }

    // even any standard integer overflowing bit
    {
        let mut lm = LinkedMask::<u8>::default();

        // functional equivalent of `lm |= 1 << 128` if it was possible
        lm.bitor_assign_one_shl(128);

        lm.bitor_assign_n_shl((1 << 9) + 1, 0);

        println!("{lm}");
    }

    {
        let lm1 = LinkedMask::<u8>::new();
        let lm2 = LinkedMask::<u8>::default();

        println!("{lm1}\n{lm2}");
    }
}

