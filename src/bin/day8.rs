use array2d::Array2D;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day8");
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let layers: Vec<Layer> = INPUT
        .trim()
        .chars()
        .chunks(WIDTH * HEIGHT)
        .into_iter()
        .map(|layer| Layer::new(&layer.collect::<String>()))
        .collect();
    
    // DAY 8A
    let mut layer_index = 0;
    let mut least_zeros = 999999999;
    for i in 0..layers.len() {
        let j = layers[i].count_occurrences(0);
        if j < least_zeros {
            layer_index = i;
            least_zeros = j;
        }
    }

    println!("{:?}", layers[layer_index].count_occurrences(1) * layers[layer_index].count_occurrences(2));

    // DAY 8B
    let mut final_layer = Layer::new_transparent();
    for layer in layers {
        final_layer = layer.place_behind(final_layer);
    }

    final_layer.print();
}

struct Layer {
    rows: Array2D<u8>,
    raw_layer: Vec<u8>,
}

impl Layer {
    fn new(input: &str) -> Self {
        let (row_iter, row_iter_clone) = input.chars().map(|i| i.to_digit(10).unwrap() as u8).tee();
        Layer {
            raw_layer: row_iter.collect(),
            rows: Array2D::from_iter_row_major(
                row_iter_clone,
                HEIGHT,
                WIDTH,
            ),
        }
    }

    fn new_transparent() -> Self {
        Layer {
            rows: Array2D::filled_with(2, HEIGHT, WIDTH),
            raw_layer: vec![2; HEIGHT*WIDTH],
        }
    }

    fn count_occurrences(&self, target: u8) -> usize {
        self.raw_layer.iter().filter(|&n| *n == target).count()
    }

    fn place_behind(&self, top: Layer) -> Layer {
        let mut raw_layer_string = "".to_owned();
        for (row_self, row_top) in self.rows.rows_iter().zip(top.rows.rows_iter()) {
            for (col_self, col_top) in row_self.zip(row_top) {
                match col_top {
                    2 => { raw_layer_string.push_str(&col_self.to_string())}
                    _ => { raw_layer_string.push_str(&col_top.to_string())}
                }
            }
        }
        Layer::new(&raw_layer_string)
    }

    fn print(&self) {
        for row in self.rows.rows_iter() {
            for col in row {
                match col {
                    1 => print!("."),
                    _ => print!(" "),
                }
            }
            println!();
        }
    }
}
