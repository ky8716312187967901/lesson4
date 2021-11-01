enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub trait TrafficLightDuration {
    fn get_duration(&self) -> u32;
}
impl TrafficLightDuration for TrafficLight {
    fn get_duration(&self) -> u32 {
        match &self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 50,
            TrafficLight::Yellow => 30,
        }
    }
}

fn main() {
    let light = TrafficLight::Red;
    println!("Red => {}", light.get_duration());
    let light = TrafficLight::Green;
    println!("Green => {}", light.get_duration());
    let light = TrafficLight::Yellow;
    println!("Yellow => {}", light.get_duration());
}
