#[derive(Debug)]
struct Graph {
    plane: [[i32; 51]; 51], // 2D cartesian plane
    inputs: Vec<i32>,       // x values
    outputs: Vec<i32>,      // y values
    equation: String,       // equation, duh!!
}

impl Graph {
    fn new() -> Self {
        let plane = [[0; 51]; 51];
        Graph {
            plane,
            inputs: vec![],
            outputs: vec![],
            equation: String::new(),
        }
    }

    fn plot(&mut self) {
        let expr = self.equation.replace("y = ", "");
        let terms: Vec<&str> = expr.trim().split(" ").collect();

        let mut y_intercept;
        y_intercept = terms[2]
            .parse::<i32>()
            .expect("Failed to parse y-intercept");
        if terms[1].eq("-") {
            y_intercept *= -1;
        }

        // compute gradient
        let gradient = terms[0].replace("x", "");
        let m = if gradient.eq("") {
            1
        } else {
            if gradient.eq("-") {
                -1
            } else {
                gradient.parse::<i32>().expect("Failed to parse gradient")
            }
        };
        let gradient = m;

        self.outputs = self.compute_outputs(gradient, y_intercept);

        // plotting
        let origin = [25; 2];
        for idx in 0..self.inputs.len() {
            let x_pos = (origin[0] + self.inputs[idx]) as usize;
            let y_pos = (origin[1] - self.outputs[idx]) as usize;
            if x_pos < 51 && y_pos < 51 {
                self.plane[y_pos][x_pos] = 1;
            }
        }
    }

    fn compute_outputs(&self, gradient: i32, y_intercept: i32) -> Vec<i32> {
        self.inputs.iter().map(|x| gradient * x + y_intercept).collect()
    }
    

    fn display_graph(&mut self) {
        // mark origin
        self.plane[25][25] = 9;

        // mark axes
        for idx in 0..51 {
            if self.plane[25][idx] == 0 {
                self.plane[25][idx] = 4;
            };
            if self.plane[idx][25] == 0 {
                self.plane[idx][25] = 4
            }
        }

        // display
        for i in 0..51 {
            for j in 0..51 {
                match self.plane[i][j] {
                    1 => print!("*"), // point
                    4 => print!("-"), // axis
                    9 => print!("+"), // origin
                    _ => print!(" ")  // empty spot
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.equation = "y = x + 0".to_string(); // must take the form y = mx + b
    graph.inputs = (-25..=25).collect();
    graph.outputs = vec![0; graph.inputs.len()];
    graph.plot();
    graph.display_graph();
    // dbg!(&graph);
}
