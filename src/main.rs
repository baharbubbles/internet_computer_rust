use std::io;

enum Operations{
    Add{x: f64, y: f64},
    Subtract{x: f64, y: f64},
    Multiply{x: f64, y: f64},
    Divide{x: f64, y: f64},
}
impl Operations {
fn calculate (&self) -> f64 {
    match self {
        Operations::Add {x, y} => {
            return x+y;
        }
        Operations::Subtract {x, y} => {
            return x-y;
        }
        Operations::Multiply {x, y} => {
            return x*y;
        }
        Operations::Divide {x, y} => {
            return x/y;
        }
    }
}
}

fn main() {
    let mut x = String::new();
    let mut opr = String::new(); 
    let mut y = String::new();

    println!("Input First Number");
    io::stdin().read_line(&mut x).expect("failed to read input");
    let x: f64 = x.trim().parse().expect("invalid input");

    println!("Input Operation(+,-,*,/)");
    io::stdin().read_line(&mut opr).expect("failed to read input");
    let opr: char = opr.trim().parse().expect("invalid input");

    println!("Input Second Number");
    io::stdin().read_line(&mut y).expect("failed to read input");
    let y: f64 = y.trim().parse().expect("invalid input");

    println!("{:?} {:?} {:?} = ",x,opr,y);

     match opr {
        '+' => {
        let islem=  Operations::Add {x:x, y: y};
         println!("{:?}",islem.calculate());

        }

        '-' => {
        let islem= Operations::Subtract {x:x, y: y};
        println!("{:?}",islem.calculate());
        }

        '*' => {
        let islem= Operations::Multiply {x:x, y: y};
        println!("{:?}",islem.calculate());
        }

        '/' => {
        let islem=   Operations::Divide {x:x, y: y};
        println!("{:?}",islem.calculate());
        }
        _ => unreachable!()
    }


}