
// fn swap<'arg>(mut p: &'arg &'arg char, mut q: &'arg &'arg char)
// {
//     // let mut t = p; 
//     // p = q; 
//     // q = t;
//     // println!("p: {}, q: {}", p, q);
//     // *(*p) = 'p';
//     // *(*q) = 'q';
// }

fn main()
{
    let a1 = 'a';
    let b1 = 'b';

    let mut a = &a1;
    let mut b = &b1;
    // swap(&mut a,&mut b);

    let t = a; 
    a = b;
    b = t;

    // println!("a: {}, b: {}", *a, *b);
}
