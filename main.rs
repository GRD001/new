use std::io;
fn main(){
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();
   
       
    loop {
    println!("\nenter a");
    match io::stdin().read_line(&mut a_str){
        Ok(_)=>{}
        Err(e) =>{
            panic!("wrong value {}", e)
        }
    }
    
    println!("enter b");
    
    match io::stdin().read_line(&mut b_str){
        Ok(_)=>{}
        Err(e) =>{
            panic!("wrong value {}", e)
        }
    }
    
    println!("enter c");
    
    match io::stdin().read_line(&mut c_str){
        Ok(_)=>{}
        Err(e) =>{
            panic!("wrong value {}", e)
        }
    }

    let a:f64 = a_str.trim().parse().unwrap();
    let b:f64 = b_str.trim().parse().unwrap();
    let c:f64 = c_str.trim().parse().unwrap();

    let d: f64 = (b*b) - 4.0*(a*c);

    if d >0.0{
        let x1 = ((-b) + d.sqrt()) / (2.0 *a);
        let x2 = ((-b)-d.sqrt()) / (2.0*a);
        print!("решено\nесть два корня\nD={}\nКорень1={}\nКорень2={}\n", d,x1,x2)
    }
    if d == 0.0{
        let x = (-b)/ (2.0*a);
        print!("решено\nесть один корень \nD=0\nКорень={} \n",x);
    }
    if d <0.0{
        println!("Net korney,D<0 \nD= {} \n", d);
    }
}
}




