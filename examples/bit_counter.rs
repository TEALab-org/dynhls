pub fn main() {
    for i in 0..32 {
        let mut r1 = i & 0b1_000_0;
        let mut r2 = i & 0b0_111_0;
        let r3 = i & 0b0_000_1;
        r1 >>= 4;
        r2 >>= 1;
        //println!("i: {}, {:b}, {:03b}, {:03b}, {:03b}", i, i, r1, r2, r3);
        println!("// -{:01b}-", r1);
        println!("// {:03b}", r2);
        println!("// -{:01b}-", r3);
        println!("0b{:05b} => (false, State::IncX)", i);
    }
    /*
    for i in 0..512 {
        println!("i: {}, {:b}", i, i);
        let mut r1 = i & 0b111_000_000;
        let mut r2 = i & 0b000_111_000;
        let r3 = i & 0b000_000_111;
        r1 >>= 6;
        r2 >>= 3;
        //println!("i: {}, {:b}, {:03b}, {:03b}, {:03b}", i, i, r1, r2, r3);
        println!("---");
        println!("i: {}", i);
        println!("{:03b}", r1);
        println!("{:03b}", r2);
        println!("{:03b}", r3);
    }
    */
}
