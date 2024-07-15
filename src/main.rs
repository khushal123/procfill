mod dirutil;
mod procutils;

fn main() {
    dirutil::create_dirs();
    procutils::stop_all_processes();
    for port in procutils::PORT_LIST {
        println!("Starting process on port {}", port);
        procutils::create_command(port)
    }
}
