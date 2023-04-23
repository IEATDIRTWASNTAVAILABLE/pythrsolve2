use text_io::read;

fn main() {
    println!("Enter the value for A");
    let a: f64 = read!();
    println!("Enter the value for B");
    let b: f64 = read!();
    println!("enter the operation you want to do on A and B, valid operations are 'add | sub | pythr'");
    let typ: String = read!();
    if typ == "add" {
        let c: f64 = a+b;
        println!("C is equal to {c}");
    }
        else if typ == "sub" {
            let c: f64 = a-b;
            println!("C is equal to {c}");
        }
        else if typ == "pythr" {
        let asqr: f64 = a.powi(2);
        let bsqr: f64 = b.powi(2);
        let csqr: f64 = asqr+bsqr;
        let sqrtc: f64 = csqr;
        let sqrtc: f64 = sqrtc.sqrt();
        println!("{}", sqrtc);
        println!("Thank you for using pythrsolve!")


        }

}
