const LOWER_BOUND: i32 = 0;
const UPPER_BOUND: i32 = 45;

trait Angle {
    fn angle(self) -> i32;
}

trait CanIncrease: Angle + Sized {
    fn increase(self) -> State {
        let new_angle = self.angle() + 1;
        if new_angle < UPPER_BOUND {
            State::InBetween(InBetween(new_angle))
        } else {
            State::UpperBound(UpperBound)
        }
    }
}

trait CanDecrease: Angle + Sized {
    fn decrease(self) -> State {
        let new_angle = self.angle() - 1;
        if new_angle > LOWER_BOUND {
            State::InBetween(InBetween(new_angle))
        } else {
            State::LowerBound(LowerBound)
        }
    }
}


struct LowerBound;
impl Angle for LowerBound {
    fn angle(self) -> i32 { LOWER_BOUND }
}
impl CanIncrease for LowerBound {}

struct UpperBound;
impl Angle for UpperBound {
    fn angle(self) -> i32 { UPPER_BOUND }
}
impl CanDecrease for UpperBound {}

struct InBetween(i32);
impl Angle for InBetween {
    fn angle(self) -> i32 {
        let InBetween(angle) = self;
        angle
    }
}
impl CanIncrease for InBetween {}
impl CanDecrease for InBetween {}

enum State {
    LowerBound(LowerBound),
    UpperBound(UpperBound),
    InBetween(InBetween)
}


fn main() {
    /*
    let state = State::UpperBound(UpperBound(45));
    match state {
        State::UpperBound(state) => {
        },
        State::LowerBound(state) => {
        },
        State::InBetween(state) => {
        }
    }
    */
}
