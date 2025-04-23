pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    let mut min = nb_1;
    let mut max = nb_1;

    if nb_2 < nb_1 {
        min = nb_2;
    } else if nb_2 > nb_1 {
        max = nb_2;
    }
    if nb_3 < min {
        min = nb_3;
    } else if nb_3 > max {
        max = nb_3;
    }
    (min, max)
}