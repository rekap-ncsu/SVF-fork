fn swap<'arg>(conspicuous_p: &'arg mut char, conspicuous_q: &'arg mut char)
{
    // println!("p: {}, q: {}", p, q);
    let conspicuous_t = *conspicuous_p; 
    *conspicuous_p = *conspicuous_q; 
    *conspicuous_q = conspicuous_t;
    // println!("p: {}, q: {}", p, q);
}

fn main()
{
    let mut conspicuous_a1 = 'a';
    let mut conspicuous_b1 = 'b';

    // let mut a = &a1;
    // let mut b = &b1;
    swap(&mut conspicuous_a1,&mut conspicuous_b1);

    // println!("a: {}, b: {}", a1, b1);
}
