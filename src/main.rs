struct UnitySeeds {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

const MT_CONSTANT: u32 = 1812433253;

fn next_mersenne(val: u32) -> u32 {
    val.wrapping_mul(MT_CONSTANT).wrapping_add(1)
}

impl UnitySeeds {
    pub fn new(init_seed: u32) -> UnitySeeds {
        let x = init_seed;
        let y = next_mersenne(x);
        let z = next_mersenne(y);
        let w = next_mersenne(z);
        UnitySeeds { x, y, z, w }
    }

    fn perform_shift(&mut self) -> u32 {
        let temp = self.x ^ (self.x.wrapping_shl(11));
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ temp ^ (temp >> 8);
        self.w
    }

    pub fn next_u32(&mut self) -> u32 {
        self.perform_shift()
    }

    pub fn next_u32_max(&mut self, max: u32) -> u32 {
        if max == 0 {
            0
        } else {
            self.next_u32() % max
        }
    }

    pub fn next_u32_range(&mut self, min: u32, max: u32) -> u32 {
        if max - min == 0 {
            min
        } else {
            if max < min {
                min - self.next_u32() % (max + min)
            } else {
                min + self.next_u32() % (max - min)
            }
        }
    }

    pub fn next_i32(&mut self) -> i32 {
        self.perform_shift() as i32
    }

    pub fn next_i32_max(&mut self, max: i32) -> i32 {
        if max == 0 {
            0
        } else {
            self.next_i32() % max
        }
    }

    pub fn next_i32_range(&mut self, min: i32, max: i32) -> i32 {
        if max - min == 0 {
            min
        } else {
            let val: i64 = self.next_u32() as i64;
            if max < min {
                (min as i64 - val % (max as i64 + min as i64)) as i32
            } else {
                (min as i64 + val % (max as i64 - min as i64)) as i32
            }
        }
    }

    pub fn next_f32(&mut self) -> f32 {
        1.0f32 - self.next_f32_range(0.0f32, 1.0f32)
    }

    pub fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        // TODO: This is like totally wrong
        (min * max) * f32::from_bits(self.perform_shift().wrapping_shl(9)) / 0xFFFFFFFFu32 as f32 + max
    }
}

fn main() {
    let mut value = UnitySeeds::new(1234);
    println!("x: {}, y: {}", value.next_u32_range(5, 100), value.next_i32_range(-100, -97));
    println!("asdf: {}", value.next_f32_range(0.0, 10.0));
}
