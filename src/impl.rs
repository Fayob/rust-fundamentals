struct Temperature {
    degree_c: f64,
}

impl Temperature {
    fn show_temp(&self) -> f64 {
        self.degree_c
    }

    fn freezing() -> Self {
        Self {
            degree_c: -1.0,
        }
    }
}

enum Color {
    Blue,
    Brown,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Blue => println!("blue"),
            Color::Brown => println!("brown"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn another_new(&self, dimensions: Dimensions) -> Self {
        Self {
            weight: self.weight,
            color: Color::Brown,
            dimensions: dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {}", self.weight)
    }
}

fn main() {
    let hot_temp = Temperature {
        degree_c: 100.0
    };
    let res = hot_temp.show_temp();
    println!("{} degree celcius", res);

    let cold_temp = Temperature::freezing();
    println!("{} degree celcius", cold_temp.show_temp());

    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let small_box = ShippingBox::new(5.0, Color::Blue, small_dimensions);
    small_box.print();
    println!("-------------------------------------");

    let small_dimensions1 = Dimensions {
        width: 2.0,
        height: 4.0,
        depth: 6.0,
    };
    // another_new method can be called like this because it return itself(that is the ShippingBox struct)
    let another_box = ShippingBox::another_new(&small_box, small_dimensions1);
    another_box.print();
    println!("-------------------------------------");

    let small_dimensions2 = Dimensions {
        width: 4.0,
        height: 8.0,
        depth: 12.0,
    };
    // another_new method can also be called like this because it accepts self as a parameter
    let another_box2 = small_box.another_new(small_dimensions2);
    another_box2.print();
}
