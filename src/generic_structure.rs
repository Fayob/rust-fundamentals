struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

trait Convey {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimensions;
}

struct ConveyorBelt<T: Convey> {
    pub items: Vec<T>,
}

impl<T: Convey> ConveyorBelt<T> {
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}

impl Default for CarPart {
    fn default() -> Self {
        Self {
            width: 5.0,
            height: 2.0,
            depth: 4.0,
            weight: 6.0,
            part_number: "abc".to_owned()
        }
    }
}

impl Convey for CarPart {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            depth: self.depth,
            height: self.height,
            width: self.width,
        }
    }

    fn weight(&self) -> f64 {
        self.weight
    }
}

trait Body {}
trait Color {}

#[derive(Debug)]
struct Vehicle<B: Body, C:Color> {
    body: B,
    color: C,
}

impl <B: Body, C: Color> Vehicle<B, C> {
    pub fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}

#[derive(Debug)]
struct Car {}
impl Body for Car{}

#[derive(Debug)]
struct Truck {}
impl Body for Truck{}

#[derive(Debug)]
struct Yellow {}
impl Color for Yellow{}

#[derive(Debug)]
struct Blue {}
impl Color for Blue{}

fn main(){
    let mut belt = ConveyorBelt{ items: vec![] };
    belt.add(CarPart::default());

    let yellow_truck = Vehicle::new(Truck{}, Yellow{});
    let blue_car = Vehicle::new(Car{}, Blue{});

    println!("{:?}", yellow_truck);
    println!("{:?}", blue_car);
}