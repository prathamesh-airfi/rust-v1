enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

fn main() {
    let my_computer = OperatingSystem::Linux;
    let age = years_since_releases(my_computer);
    println!("My computer's operating system is {age} years old");
}

fn years_since_releases(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => 39,
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}
