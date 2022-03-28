pub struct RegisterPool {
    registers: Vec<i32>,
}

impl RegisterPool {
    pub fn new() -> RegisterPool {
        Self {
            registers: vec![0, 1, 2, 3, 4, 5,6 7, 8, 9, 10, 11, 12, 13, 14, 15],
        }
    }
    pub fn get_register(&mut self) -> i32 {
        let reg_option = self.registers.pop();
        match reg_option {
            None => panic!("Registers have been used up"),
            Some(rid) => rid,
        }
    }

    pub fn give_back(&mut self, rid: i32) {
        self.registers.push(rid);
    }
}
