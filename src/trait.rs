trait Fall {
    fn hit_ground(&self);
}

struct Vase;
impl Fall for Vase {
    fn hit_ground(&self) {
        println!("the vase broke")
    }
}

struct Cat;
impl Fall for Cat {
    fn hit_ground(&self) {
        println!("the cat casually walked away")
    }
}

fn fall(thing: impl Fall) {
    thing.hit_ground();
}

trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}

struct Square {
    side: i32
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        self.side * 4
    }
}

struct Triangle {
    side_1: i32,
    side_2: i32,
    side_3: i32,
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        self.side_1 + self.side_2 + self.side_3
    }
}

fn print_perimeter(shape: impl Perimeter) {
    let perimeter = shape.calculate_perimeter();
    println!("perimeter = {:?}", perimeter)
}

fn main() {
    fall(Vase{});
    fall(Cat{});

    let square = Square{ side: 5 };
    let triangle = Triangle{ side_1: 2, side_2: 3, side_3: 4 };
    print_perimeter(square);
    print_perimeter(triangle);
}
