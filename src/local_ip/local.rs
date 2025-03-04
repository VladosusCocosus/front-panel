use local_ip_address::local_ip;

pub fn get_ip() -> String {
    let my_local_ip = local_ip().unwrap().to_string();

    my_local_ip
}