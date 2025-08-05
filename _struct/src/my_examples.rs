// tuple struct
struct Token(
   String, u64
);

// C,C++ style struct

struct Employee {
    name: String,
    salary: i32,
    id: u32,
    token: Token,
}

// impl Employee {
//     fn mark_attendance(&self) {
//         println!(
//             "{} {} (ID: {}) scanned token: {} ({})",
//             self.name,self.salary, self.id, self.token.0, self.token.1  // Access tuple fields
//             /*
//             self.token.0 → Refers to the first field of the tuple (String).
//             self.token.1 → Refers to the second field of the tuple (u64).
//             */
//         );
//     }
// }

// fn main() {
//     let charlie = Employee {
//         name: String::from("Charlie"),
//         id: 103,
//         salary: 70000,
//         token: Token(String::from("Password"), 555555555),
//     };

//     charlie.mark_attendance();
//     // Output: Charlie (ID: 103) used token: Password (555555555)
// }



/*  #Unit struct with a tuple struct.
        => Unit Struct as a "Marker" for a Tuple Struct
*/
struct AdminToken;

// impl Token{
//     fn validate(&self, _marker: &AdminToken)->bool {
//         self.0=="admin" && self.1<1
//     }
// }

// fn main() {
//     // let token = Token(String::from("admin"), 123456789);
//     //let token = Token(String::from("user"), 123456789); 
    
//     let marker= AdminToken;// Create an instance of the unit struct
//     if token.validate(&marker) {
//         println!("Token is valid for {}",token.0);
//     } else {
//         println!("Token is invalid for {}",token.0);
//     }
    
// }


/*
unit struct + C-style struct 
*/

struct FullTime;
struct PartTime;

// C-style struct with generics
struct EmployeeDetails <T> {
    name: String,
    id: u32,
    working_day: u32,
    employment_type: T, // Uses the unit struct (compile-time only)
}

impl <T> EmployeeDetails <T>{
    fn new(name:String, id:u32, working_day:u32, employment_type:T) -> Self {
        EmployeeDetails {
            name,
            id,
            working_day,
            employment_type,
        }
    }
}

impl EmployeeDetails<FullTime> {
    // Method for full-time employees and it is not dynamic use enum for dynamicly choosing employment type
    fn fulltimer_salary(&self) {
        println!(
            "Full-time employee: {} ",
            self.name
        );
        // Example salary calculation
         println!{"{}", self.working_day * 2500 }

    }
}
impl EmployeeDetails<PartTime> {
    // Method for part-time employees and it is not dynamic use enum for dynamicly choosing employment type
    fn parttimer_salary(&self) {
        println!(
            "Part-time employee: {}",
            self.name
        );
        println!{"{}", self.working_day * 2000 }
        // Example hourly wage calculation
    }
}

fn main() {
    let full_time_emp = EmployeeDetails::<FullTime>::new(
        String::from("Alice"),
        101,
        25,
        FullTime,
    );
    full_time_emp.fulltimer_salary();

    let part_time_emp = EmployeeDetails::<PartTime>::new(
        String::from("Bob"),
        102,
        25,
        PartTime,
    );
    part_time_emp.parttimer_salary();
}
