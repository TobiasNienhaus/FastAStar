use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Weight {
    Infinity,
    Some(f64),
}

impl Display for Weight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match &self {
            Weight::Infinity => write!(f, "Infinity"),
            Weight::Some(num) => write!(f, "{}", num),
        }
    }
}

impl Weight {
    pub fn random() -> Weight {
        Self::random_inf_weight(0.01)
    }

    pub fn random_inf_weight(threshold: f64) -> Weight {
        if rand::random::<f64>() < threshold {
            Weight::Infinity
        } else {
            Weight::Some(rand::random::<f64>())
        }
    }
}

impl From<f64> for Weight {
    fn from(num: f64) -> Self {
        Weight::Some(num)
    }
}

impl PartialEq for Weight {
    fn eq(&self, other: &Self) -> bool {
        // TODO weird stuff here
        match &self {
            Weight::Infinity => {
                if let Weight::Some(_) = other {
                    false
                } else {
                    true
                }
            }
            Weight::Some(num) => {
                if let Weight::Some(num2) = other {
                    num == num2
                } else {
                    false
                }
            }
        }
    }
}

impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        match &self {
            Weight::Infinity => match other {
                Weight::Infinity => Some(Ordering::Equal),
                Weight::Some(_) => Some(Ordering::Greater),
            },
            Weight::Some(num) => match other {
                Weight::Infinity => Some(Ordering::Less),
                Weight::Some(num2) => num.partial_cmp(num2),
            },
        }
    }
}

impl std::ops::Add for Weight {
    type Output = Weight;

    fn add(self, rhs: Self) -> Self::Output {
        if let Weight::Some(num) = self {
            if let Weight::Some(num2) = rhs {
                Weight::Some(num + num2)
            } else {
                Weight::Infinity
            }
        } else {
            Weight::Infinity
        }
    }
}
