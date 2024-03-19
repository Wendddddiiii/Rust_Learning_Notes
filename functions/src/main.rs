// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl Point<f64> {
//     fn distance_from_origin(&self) -> f64 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p: Point<f64> = Point{x: 5.0, y: 10.0};

//     println!("{}", p.distance_from_origin());
// }

struct Array<T, const N: usize> {
    data: [T; N]
}

fn main() {
    let arrays: [Array<i32, 3>; 3] = [
        Array {
            data: [1, 2, 3],
        },
r
        Array {
            data: [3, 4, 5],
        },

        Array {
            data: [1, 2, 0],
        },
    ];

    let floats: [Array<f64, 2>; 3] = [
        Array {data: [1.0, 2.0]},
        Array {data: [3.0, 2.5]},
        Array {data: [1.1, 2.3]},
    ];

    println!("success");

}