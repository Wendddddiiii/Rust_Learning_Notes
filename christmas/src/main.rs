#[derive(Debug)]
struct Light {
    r: u8,
    g: u8,
    b: u8,
    next: Option<Box<Light>>
}


impl Light {
    fn new(r: u8, g:u8, b:u8) -> Light {
        Light { r, g, b, next: None}
    }

    fn add_to_chain(&mut self, r:u8, g: u8, b:u8) {
        let mut current_light = self;

        while let Some(ref mut next) = current_light.next {
            current_light = next;
        }
        current_light.next = Some(Box::new(Light::new(r, g, b)));
    }
}

fn main() {
    let mut light = Light::new(255, 0, 0);
    light.add_to_chain(0, 255, 0);
    light.add_to_chain(0, 0, 255);
    light.add_to_chain(255, 255, 0);

    let mut current_light: Option<&Box<Light>> = light.next.as_ref();
    while let Some(light) = current_light {
        println!("{:?}", light);
        current_light = light.next.as_ref();
    }
}
