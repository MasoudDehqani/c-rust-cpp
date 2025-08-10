#[derive(Debug)]
enum IpAddrVer {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct ProgrammerInfo {
    name: String,
    age: u8,
}

#[derive(Debug)]
enum Occupation {
    Programmer(ProgrammerInfo),
    Developer,
}

impl Occupation {
    fn get_programmer_info(&self) -> &str {
        match self {
            Occupation::Programmer(info) => &info.name,
            Occupation::Developer => "DEVELOPER",
        }
    }

    fn new_programmer(name: String) -> Occupation {
        Occupation::Programmer(ProgrammerInfo { name, age: 36 })
    }
}

fn main() {
    let v4 = IpAddrVer::V4;
    let v6 = IpAddrVer::V6;

    if let IpAddrVer::V4 = v6 {
        println!("Version FOUR");
    }

    match v6 {
        IpAddrVer::V4 => println!("version four"),
        IpAddrVer::V6 => println!("VERSION SIX"),
    }

    println!("{:?}", v4);
    println!("{:?}", v6);

    let ip_addr_v4 = IpAddr::V4(String::from("172.0.0.1"));
    let ip_addr_v6 = IpAddr::V6(String::from("20a3:5404:50d4:3742:6752:62c7:d4e2:471f"));
    println!("{:?}", ip_addr_v4);

    if let IpAddr::V6(ref addr) = ip_addr_v6 {
        println!("{addr}");
    }

    match ip_addr_v6 {
        IpAddr::V4(addr) => {
            if addr == String::from("172.0.0.1") {
                println!("YES")
            } else {
                println!("NO")
            }
        }
        IpAddr::V6(addr) => println!("{addr}"),
    }

    let arr = [1, 2, 3, 4, 5];
    let el: Option<&i32> = arr.get(2);
    match el {
        Some(&v) => println!("{v}"),
        None => println!("HICHI"),
    }

    let occ = Occupation::Programmer(ProgrammerInfo {
        name: String::from("A Khafan Programmer"),
        age: 35,
    });

    if let Occupation::Programmer(ProgrammerInfo { name, age }) = &occ {
        println!("{name}, {age}");
    }

    let inf = occ.get_programmer_info();
    println!("{inf}");

    let n_programmer = Occupation::new_programmer(String::from("TOPOLI"));
    let _alaki = Occupation::Developer;
    match n_programmer {
        Occupation::Developer => println!("INJI"),
        _ => println!("HICHI"),
    }
    println!("{:?}", n_programmer);
}
