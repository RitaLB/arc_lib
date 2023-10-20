pub mod datatypes {
    
    pub fn variables() {
    // by defout, all variables declared in rust are immutable
    // pu "mut" after "let" to turn the variable mutable
        // impliciting declaring a variable:
        let mut x = 8;
        println!("{}",x);
    
        // expliciting declaring a unsined int  32 type variable:
        let mut x2: u32 = 4;
        println!("{}",x2);
    
        x2 = 32;
        println!("{}",x2);
    
        // if you don't want the variable to be set as mutable, you can change its value by redeclaring it:
        let x3 = 4;
        println!("x3  ={}", x3);
    
        let x3 = 44;
        println!("x3 = {}", x3);
        
        // you can change the type of a vriable if you redifine it
       
    }   

    pub fn constants(){
        // a constant is something whose value and type can not change through the entirety of the program once it's defined
        // constants cannot be redefined
        const SECONDS_IN_MINUTE: u32 = 60;
        println!("{}", SECONDS_IN_MINUTE);
    }
}