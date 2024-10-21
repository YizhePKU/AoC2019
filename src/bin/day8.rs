use itertools::Itertools;

fn number_of_specific_char(layer: &Vec<Vec<char>>, target: char) -> usize {
    layer
        .into_iter()
        .map(|row| row.into_iter().filter(|&&c| c == target).count())
        .sum()
}

fn main() {
    let input = std::fs::read("data/day8").unwrap();
    let input = String::from_utf8(input).unwrap();

    let image = input
        .trim()
        .chars()
        .chunks(25 * 6)
        .into_iter()
        .map(|layer| {
            layer
                .chunks(25)
                .into_iter()
                .map(|row| row.collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let layer_with_fewest_zeros = image
        .iter()
        .min_by_key(|&layer| number_of_specific_char(layer, '0'))
        .unwrap();
    let part1 = number_of_specific_char(layer_with_fewest_zeros, '1')
        * number_of_specific_char(layer_with_fewest_zeros, '2');
    dbg!(part1);

    // render the image
    for i in 0..6 {
        for j in 0..25 {
            for layer in &image {
                if layer[i][j] == '0' {
                    print!(" ");
                    break;
                }
                if layer[i][j] == '1' {
                    print!("X");
                    break;
                }
            }
        }
        println!();
    }
}
