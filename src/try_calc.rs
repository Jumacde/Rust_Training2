pub struct Test_calc1 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub sum: i32,
    pub mul: i32,
}

impl Test_calc1{
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {x, y, z, sum:0, mul:0} 
    }

    // digit set in the main function.
    pub fn calc_sum(&mut self) {
        self.sum = self.x + self.y;
        println!("{} + {} = {}", self.x, self.y, self.sum);
    }

    pub fn calc_mul(&mut self) {
        self.mul = self.x * self.z;
        println!("{} * {} = {}", self.x, self.z, self.mul);
    }
}

