fn main() {
    let data_list = vec![5, 2, 8, 3, 1, 4, 6];
    let sorted = merge_sort(&data_list);
    println!("{:?}", sorted);
}

fn merge_sort(data_list: &[usize]) -> Vec<usize> {
    if data_list.len() == 1 {
        return data_list.to_vec();
    }
    let half_point = data_list.len() / 2;
    let (l, r) = data_list.split_at(half_point);
    let sorted_l = merge_sort(l);
    let sorted_r = merge_sort(r);

    sort(sorted_l, sorted_r)
}

fn sort(l: Vec<usize>, r: Vec<usize>) -> Vec<usize> {
    let mut sorted = Vec::with_capacity(l.len() + r.len());
    let mut i = 0;
    let mut j = 0;

    while i < l.len() && j < r.len() {
        if l[i] <= r[j] {
            sorted.push(l[i]);
            i += 1;
        } else {
            sorted.push(r[j]);
            j += 1;
        }
    }

    // Append any remaining elements from `left`
    sorted.extend_from_slice(&l[i..]);

    // Append any remaining elements from `right`
    sorted.extend_from_slice(&r[j..]);

    sorted
}
// fn main() {
//     println!("iko swafi");
// }
// fn merge_sort(data_list: &[usize]) -> Option<Vec<usize>> {
//     if data_list.len() == 1 {
//         return Some(data_list);
//     }
//     let half_point = data_list.len() / 2;
//     let (l, r) = data_list.split_at(half_point);
//     let sorted_l = merge_sort(l)?;
//     let sorted_r = merge_sort(r)?;

//     let sorted = sort(sorted_l, sorted_r);
//     return Some(sorted.to_vec());
// }
// fn sort(l: &[usize], r: &[usize]) -> Vec<usize> {
//     let mut sorted = Vec::with_capacity(l.len() + r.len());
//     let mut i = 0;
//     let mut j = 0;

//     while i < l.len() && j < r.len() {
//         if l[i] <= r[j] {
//             sorted.push(l[i].clone());
//             i += 1;
//         } else {
//             sorted.push(r[j].clone());
//             j += 1;
//         }
//     }

//     // Append any remaining elements from `left`
//     sorted.extend_from_slice(&l[i..]);

//     // Append any remaining elements from `right`
//     sorted.extend_from_slice(&r[j..]);

//     sorted
// }
