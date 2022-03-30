pub struct RegisterPool {
    registers: Vec<i32>,
}

impl RegisterPool {
    pub fn new() -> RegisterPool {
        Self {
            // temporary registers from 1 to 12
            // r0 is constant 0
            // r13 for function return value
            // r14 for stack pointer
            // r15 for jump back link (to jump back after a func call)
            registers: vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2],
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
