pub struct MonadVariant {
    variables: [[isize; 3]; 14],
}

impl MonadVariant {
    pub fn from_program(program: &str) -> Self {
        let mut variables = [[0isize; 3]; 14];
        let mut lines_iter = program.lines();

        for i in 0..14 as usize {
            lines_iter.next();
            lines_iter.next();
            lines_iter.next();
            lines_iter.next();
            let a_instruction = lines_iter.next().unwrap();
            let b_instruction = lines_iter.next().unwrap();
            lines_iter.next();
            lines_iter.next();
            lines_iter.next();
            lines_iter.next();
            lines_iter.next();
            lines_iter.next();
            lines_iter.next();
            lines_iter.next();
            lines_iter.next();
            let c_instruction = lines_iter.next().unwrap();
            lines_iter.next();
            lines_iter.next();

            variables[i][0] = a_instruction.split(" ").nth(2).unwrap().parse().unwrap();
            variables[i][1] = b_instruction.split(" ").nth(2).unwrap().parse().unwrap();
            variables[i][2] = c_instruction.split(" ").nth(2).unwrap().parse().unwrap();
        }

        Self { variables }
    }

    pub fn execute(&self, possible_model_number: &isize) -> isize {
        let model = possible_model_number.to_string();
        if model.contains("0") {
            return 1;
        }
        let digits: Vec<isize> = model
            .chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect();

        let mut z = 0;
        for i in 0..14 {
            let w = digits[i];

            let test = (z % 26) + self.variables[i][1] != w;
            z /= self.variables[i][0];

            if test {
                z = (z * 26) + w + self.variables[i][2];
            };
        }

        z
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_validation_variables_from_program() {
        let program = fs::read_to_string("input.txt").unwrap();
        let monad = MonadVariant::from_program(&program);

        assert_eq!(
            monad.variables,
            [
                [1, 12, 4],
                [1, 15, 11],
                [1, 11, 7],
                [26, -14, 2],
                [1, 12, 11],
                [26, -10, 13],
                [1, 11, 9],
                [1, 13, 12],
                [26, -7, 6],
                [1, 10, 2],
                [26, -2, 11],
                [26, -1, 12],
                [26, -4, 3],
                [26, -12, 13]
            ]
        )
    }
}
