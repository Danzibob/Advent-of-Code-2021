// Unfinished class to stream individual bits from a vec of u32
// Replaced in solution (day16) with bool bitvec, using a bool array

struct BitFeed {
    data: Vec<u32>,
    pointer: usize
}

impl BitFeed {
    fn new(data: Vec<u32>) -> BitFeed{
        BitFeed {data: data, pointer: 0}
    }

    fn take_bits(&mut self, amount: usize) -> Option<u32> {
        if amount + self.pointer > self.data.len()*32 {
            return None;
        }

        // Indexes in the data Vec
        let first_idx = self.pointer >> 5; // Divide by 32
        let second_idx = (self.pointer + amount) >> 5;

        // Bit offsets within the u32
        let start = self.pointer - (first_idx << 5);
        let end = (self.pointer + amount) - (second_idx << 5);
        self.pointer += amount;
        
        if first_idx == second_idx {
            // Item is in a single index
            // start v v end
            //   0123456789...
            // 0b0000111000...
            let mask:u32 = (((1_u64 << (32 - start)) - 1) ^ ((1_u64 << (32 - start - amount)) - 1)) as u32;
            println!("Mask A:  {:032b}", (1_u64 << (32 - start))-1);
            println!("Mask B:  {:032b}", (1_u64 << (32 - start - amount))-1);
            println!("Mask  :  {:032b}", mask);

            println!("Start:{} and amount:{} at pointer:{}",start, amount, self.pointer);
            println!("Masked:  {:032b}", mask & self.data[first_idx]);
            println!("Shifted: {:032b}", (mask & self.data[first_idx]) >> (32-end));
            println!("");
            return Some((mask & self.data[first_idx]) >> (32-end));
        } else {
            // Item is split between indexes
            // start v         v end 
            //   01234567    01234567
            // 0b00001111  0b11100000
            let top_mask = 1 << (32 - start);
            let bot_mask = u32::MAX ^ (1 << (32-end));
            let top = self.data[first_idx] & top_mask;
            let bot = self.data[first_idx] & top_mask;

            unimplemented!();
            
        }


        return Some(0);
    }
}