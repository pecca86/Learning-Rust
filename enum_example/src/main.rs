// Enumarate all IP-address types
#[derive(Debug)]
enum IpAddrType {
    V4,
    V6,
}

// Create a Ip-address struct
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrType,
    address: String,
}

// Putting data straight into an enum
#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Method for enum IpAddress
impl IpAddress {
    fn dial_up(&self) {
        println!("Dialing from ip: {:?}", self)
    }
}


fn main() {

    // Create instances of our enums
    let ip_four = IpAddrType::V4;
    let ip_six = IpAddrType::V6;

    route(ip_six);

    // Create a new ip-address struct
    let my_home_ip = IpAddr { kind: ip_four, address: String::from("127.0.0.1") };
    println!("Home ip: {:#?}", my_home_ip);

    // Putting data into enum instead of enum inside struct
    let cottage_ip = IpAddress::V4(127, 0, 0, 1);
    println!("Cottage: {:#?}", cottage_ip);

    // Testing enum method
    cottage_ip.dial_up();

}

// Create a function that takes in a IpAddrType type
fn route(ip_type: IpAddrType) {
    println!("The type {:?}", ip_type);
}
