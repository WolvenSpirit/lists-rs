
struct N<'a> {
    v: Option<&'a i32>,
    next: Option<&'a N<'a>>,
}

pub fn fiddle() {
    let i = 3;
    let z: i32 = 5 ;
    let q: i32 = 7 ;
    let no_next= None;
    let mut foo = N{v: Some(&i),next: no_next};
    let mut bar = N{v: Some(&z), next: no_next};
    let  baz = N{v: Some(&q), next: no_next};
    //bar.next = Some(&foo);
    bar.next = Some(&baz);
    foo.next = Some(&bar);
    
    let mut curr = Some(&foo);
    let mut j = 0;
    while j < 10 {
        match curr {
            None => println!("No val"),
            Some(p) => {
                match p.v {
                    None => println!("No val inside"),
                    Some(g) => println!("{}", g)
                }
        }
    }
        
        curr = curr.unwrap().next;
        j = j+1;
    }
}