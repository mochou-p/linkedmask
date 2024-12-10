// linkedmask/examples/usage.rs

use core::iter::repeat;

use linkedmask::LinkedMask;


fn main() {
    // every LinkedMask<T> grows to fit the biggest 1 bit
    {
        let mut lm1 = LinkedMask::<u8>::default();
        let mut lm2 = LinkedMask::<u16>::default();
        let mut lm3 = LinkedMask::<u32>::default();
        let mut lm4 = LinkedMask::<u8>::default();

        lm1 |= (1 << 26) + (1 << 9) + 1;
        lm2 |= (1 << 26) + (1 << 9) + 1;
        lm3 |= (1 << 26) + (1 << 9) + 1;
        lm4 |= ((1 << 9) + 1) << 1;

        println!("{lm1}\n{lm2}\n{lm3}\n{lm4}\n");
    }

    // even any standard integer overflowing bit
    {
        let mut lm1 = LinkedMask::<u8>::default();
        let mut lm2 = LinkedMask::<u8>::default();

        // functional equivalent of `lm |= 1 << 128` if it was possible
        lm1.bitor_assign_one_shl(128);
        lm2.bitor_assign_n_shl((1 << 9) + 1, 0);

        println!("{lm1}\n{lm2}\n");
    }

    // new -> empty, default -> one 0
    {
        let lm1 = LinkedMask::<u8>::new();
        let lm2 = LinkedMask::<u8>::default();

        println!("{lm1}\n{lm2}\n");
    }

    // from array, or iterator
    {
        let lm1 = LinkedMask::<u8>::from([]);
        let lm2 = LinkedMask::<u8>::from([1, 0, 1 << 2]);
        let lm3 = repeat(1 << 1).take(3).collect::<LinkedMask<u8>>();

        println!("{lm1}\n{lm2}\n{lm3}\n");
    }

    // to and into Vec<T>
    {
        let lm1 = LinkedMask::<u8>::new();
        let lm2 = LinkedMask::<u8>::from([1 << 0, 1 << 1, 1 << 2, 1 << 3]);

        println!("{:?}\n{:?}", lm1.to_vec(), lm2.into_vec());

        // lm2 already dropped here
    }
}

