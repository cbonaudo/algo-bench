pub struct CircBuf {
    values: Vec<i32>,
    write_position: usize,
    size: usize,
}

impl CircBuf {
    pub fn new(size: usize) -> Self {
        if size < 1 { 
            panic!("can't create a negative sized buffer");
        }

        Self {
            values: Vec::with_capacity(size),
            size,
            write_position: 0,
        }
    }

    pub fn push(&mut self, value: i32) {
    
        if self.values.len() == self.size {
            self.values.remove(self.write_position);
            self.values.insert(self.write_position, value);
        } else {
            self.values.push(value);
        }
        dbg!(&self.write_position);
        dbg!(&self.values);

        self.write_position += 1;
        if self.write_position == self.size {
            self.write_position = 0;
        }

    }

    pub fn values(&self) -> Vec<i32>  {
        let mut values_vec = Vec::new();

        for i in 0..self.size {
            let mut read_position = self.write_position + i;
            if read_position >= self.size {
                read_position = read_position - self.size;
            }

            dbg!(&read_position);

            if let Some(value) = self.values.get(read_position) {
                values_vec.push(*value);
            }


        }
        dbg!(&values_vec);

        values_vec
    }
}