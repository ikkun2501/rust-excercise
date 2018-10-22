#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years"
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_right_matches('s')
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

impl RoughTime {
    fn timeunit(self) -> Option<TimeUnit> {
        match self {
            RoughTime::InThePast(t, u) | RoughTime::InTheFuture(t, u) => Some(t),
            _ => None
        }
    }
}

enum Shape {
    Sphere { center: String, radius: f32 },
    Cuboid { center1: String, corner2: String },
}

enum RelationshipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: u32,
        cdr: i32,
    },
}

enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

enum BinaryTree<T> {
    Empty,
    NotEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

fn main() {
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);

    println!("{}", four_score_and_seven_years_ago.timeunit().unwrap().plural());
//    println!("{}", three_hours_from_now.into().2);
    let unit_sphere = Shape::Sphere { center: "ORIGIN", radius: 1.0 };
}
