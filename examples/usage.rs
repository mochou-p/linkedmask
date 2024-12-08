// linkedmask/examples/usage.rs

use linkedmask::LinkedMask;


fn main() {
    {
        let lm = LinkedMask::<u8>::new();

        println!("{lm}");
    }
    {
        let mut lm = LinkedMask::<u8>::new();

        lm.add(0);
        lm.add(1);
        lm.add(3);

        println!("{lm}");
    }
    {
        let mut lm = LinkedMask::<u8>::new();

        lm.add( 1);
        lm.add( 2);
        lm.add( 5);
        lm.add(14);

        println!("{lm}");
    }
    {
        let mut lm = LinkedMask::<u16>::new();

        lm.add(23);

        println!("{lm}");
    }
}

