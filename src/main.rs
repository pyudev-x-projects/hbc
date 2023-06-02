use pyu_rust_util as pyu;

fn main() {
    
    let mut var = String::from("nil");    

    loop {
        
        
        let v = pyu::input("; ");

        let split = v.split_whitespace();

        let collection = split.collect::<Vec<&str>>();
        // dbg!(collection);


        
        let cmd = collection.get(0).unwrap().trim();

        // let arg1 = collection[1].trim();
        // let n2: f32 = collection[2].trim().parse().expect("Could not convert.");
        // let operator = collection[1].trim();

  
        match cmd {
            "print" => {
                for v in collection {
                    if v != cmd {
                        print!("{} ", v);
                    }
                }

                println!();
            }

            "set" => {
                let v = String::from(collection[1].trim());
                
                var = v;
                
            }

            "var" => {
                println!("{}", var);
            }


            "printc" => {
                let operator = collection[2].trim();
                let n1: f64 = collection[1].parse().expect("fail");
                let n2: f64 = collection[3].parse().expect("fail");

                match operator {
                    "+" => println!("{}", n1+n2),
                    "-" => println!("{}", n1-n2),
                    "*" => println!("{}", n1*n2),
                    "/" => println!("{}", n1/n2),
                    _ => println!("invalid operator.")
                }

            }


            "exit" => {
                return;
            }

            _ => {

            }
        }
    }
}