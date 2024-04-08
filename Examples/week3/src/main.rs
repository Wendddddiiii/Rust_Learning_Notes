// temporality and lifetime

use bmp::Image;

pub fn main() {
    let mut image = Image::new(256, 256);
    draw_line(&mut image, 20, 20, 80, LineDirection::Horizontal);

    image.save("my_image.bmp").unwrap();
}

enum LineDirection {
    Vertical,
    Horizontal
}

fn draw_line(image: &mut Image, x: u32, y: u32, length: u32, direction:LineDirection) {
    for i in 0..length{
        let (cur_x, cur_y) = match direction {
            LineDirection::Horizontal => {
                (x + i, y)
            }
            LineDirection::Vertical => {
                (x, y + i)
            }
        };
        image.set_pixel(cur_x, cur_y, bmp::consts::RED);
    }
}


fn string_chars_len(string: &String) -> usize {
    string.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_chars_len() {
        let test_string = String::from("Hello, world!");
        assert_eq!(string_chars_len(&test_string), 13);

        let test_string = String::from("!");
        assert_eq!(string_chars_len(&test_string), 1);
    }
}


fn emphasize(string: &mut String) {
    string.make_ascii_uppercase();
    string.push_str("!!!");
}

mod tests_2 {
    use super::emphasize;

    #[test]
    fn empty() {
        let mut s = String::from("");
        emphasize(&mut s);
        assert_eq!(s, "!!!");
    }

    #[test]
    fn lowercase() {
        let mut s = String::from("hello");
        emphasize(&mut s);
        assert_eq!(s, "HELLO!!!");
    }

    #[test]
    fn mixed_case() {
        let mut s = String::from("HeLlo");
        emphasize(&mut s);
        assert_eq!(s, "HELLO!!!");
    }

    #[test]
    fn already_uppercase() {
        let mut s = String::from("WORLD");
        emphasize(&mut s);
        assert_eq!(s, "WORLD!!!");
    }
}