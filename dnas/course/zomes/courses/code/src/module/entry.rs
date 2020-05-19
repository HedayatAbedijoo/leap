pub struct Module {
    title: String,
    teacher_address: Address,
    course_anchor_address: Address,
    timestamp: i32,
}

// pub struct Borrowed_Module {
//     module_address: Address,
//     teacher_address: Address
// }

// Hedayat: "Holochian" module -> Module {"holochain", "holochain.url", {Hedayat address}, {some course address}, timestamp}
// Nastasia: borrows "Holochain"module into course Rust -> Borrowed_Module {{"holochain module address"}, {Nastasia address}}
