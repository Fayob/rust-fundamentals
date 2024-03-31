trait Sale {
    fn amount(&self) -> f64;
}

struct FullSale(f64);
impl Sale for FullSale {
    fn amount(&self) -> f64 {
        self.0
    }
}

struct OneDollarOffCoupon(f64);
impl Sale for OneDollarOffCoupon {
    fn amount(&self) -> f64 {
        self.0 - 1.0
    }
}

struct TenPercentOffPromo(f64);
impl Sale for TenPercentOffPromo {
    fn amount(&self) -> f64 {
        self.0 * 0.9
    }
}

fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f64 {
    sales.iter().map(|sale| sale.amount()).sum()
}

trait Material {
    fn cost_per_sq_meter(&self) -> f64;
    fn square_meters(&self) -> f64;
    fn total_cost(&self) -> f64 {
        self.cost_per_sq_meter() * self.square_meters()
    }
}

struct Carpet(f64);
impl Material for Carpet {
    fn cost_per_sq_meter(&self) -> f64 {
        10.0
    }

    fn square_meters(&self) -> f64 {
        self.0
    }
}

struct Tile(f64);
impl Material for Tile {
    fn cost_per_sq_meter(&self) -> f64 {
        15.0
    }

    fn square_meters(&self) -> f64 {
        self.0
    }
}

struct Wood(f64);
impl Material for Wood {
    fn cost_per_sq_meter(&self) -> f64 {
        20.0
    }

    fn square_meters(&self) -> f64 {
        self.0
    }
}

fn total_cost(material: &Vec<Box<dyn Material>>) -> f64 {
    material.iter().map(|mat| mat.total_cost()).sum()
}

fn main() {
    let price = 20.0;
    let regular = Box::new(FullSale(price));
    let coupon = Box::new(OneDollarOffCoupon(price));
    let promo = Box::new(TenPercentOffPromo(price));

    let sales: Vec<Box<dyn Sale>> = vec![regular, coupon, promo];
    let revenue = calculate_revenue(&sales);
    println!("total revenue = {}", revenue);

    println!("---------------------------------------------");

    let carpet = Box::new(Carpet(20.0));
    let tile = Box::new(Tile(10.0));
    let wood = Box::new(Wood(30.0));

    let materials: Vec<Box<dyn Material>> = vec![carpet, tile, wood];

    let total = total_cost(&materials);
    println!("cost = ${}", total);
}
