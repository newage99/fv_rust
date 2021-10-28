pub struct FVID;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

impl FVID {
    pub fn check(fvid: &str) {
        println!("{}", fvid);
        //println!("{}", symbols.len());
        
    }
}