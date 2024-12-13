// linkedmask/examples/usage.rs

use {
    core::{iter::repeat, panic::UnwindSafe},
    std::panic::{catch_unwind, set_hook, take_hook}
};

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

        println!("{:?}\n{:?}\n", lm1.to_vec(), lm2.into_vec());

        // lm2 already dropped here
    }

    // get, get_mut, Index, IndexMut
    {
        let mut lm1 = LinkedMask::<u8>::default();
        let mut lm2 = LinkedMask::<u8>::default();

        println!("{lm1}, {lm2}");

        assert!(lm1.get(1).is_none());
        *lm1.get_mut(0).unwrap() |= 1 << 2;

        assert!(ensure_panic(|| { let _: u8 = lm2[1]; }));
        lm2[0] |= 1 << 2;

        println!("{lm1}, {lm2}\n");
    }

    // binary helpers
    {
        let lm1 = LinkedMask::<u8>::from([0, 15 << 2, 0]);
        let lm2 = LinkedMask::<u8>::from([u8::MAX,   (1 << 7) + (1 << 3) + 3,  u8::MAX]);
        let lm3 = !lm2.clone();

        println!("{lm1}: ones: {}, zeros: {}", lm1.count_ones(), lm1.count_zeros());
        println!("{lm2}: leading ones:  {}, trailing_ones:  {}", lm2.leading_ones(),  lm2.trailing_ones());
        println!("{lm3}: leading zeros: {}, trailing_zeros: {}", lm3.leading_zeros(), lm3.trailing_zeros());
    }
}

fn ensure_panic<F>(f: F) -> bool
where
    F: FnOnce() + UnwindSafe
{
    let original_hook = take_hook();
    set_hook(Box::new(|_| {}));

    let panicked = catch_unwind(f).is_err();

    set_hook(original_hook);

    panicked
}

