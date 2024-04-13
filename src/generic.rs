trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("checked in as pilot")
    }

    fn process(&self) {
        println!("pilot enters the cockpit")
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("checked in as passenger")
    }

    fn process(&self) {
        println!("passenger sit down")
    }
}
struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("cargo checked in")
    }

    fn process(&self) {
        println!("cargo moved to storage")
    }
}

fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}

#[derive(Debug)]
enum ServicePriority {
    High,
    Standard
}

#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug)]
struct Guest;
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

fn print_guest_priority<T>(guest: T) where T: Priority + std::fmt::Debug {
    println!("{:?} is {:?} priority", guest, guest.get_priority())
}

fn main(){
    process_item(Pilot);
    process_item(Passenger{});
    process_item(Cargo{});

    print_guest_priority(Guest);
    print_guest_priority(ImportantGuest);
}