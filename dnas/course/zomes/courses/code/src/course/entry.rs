pub struct Course {
    title: String,
    teacher_address: AGENT_ADDRESS,
    modules: Vec<Address>,
    anchor_address: Address,
    timestamp: i32,
}

pub struct Course_Anchor {
    timestamp: i32,
    teacher_address: Address,
}

impl HolochainEntry for Course {
    fn entry_type() -> String {
        String::from("course")
    }
}

// Anchor
// All_Course_Anchor

// Links
// All_Course_Anchor -> Course_Anchor

//  Course_Anchor -> Course
