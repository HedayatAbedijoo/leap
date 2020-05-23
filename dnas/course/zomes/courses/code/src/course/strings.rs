// Constant strings

pub const COURSE_ANCHOR_ENTRY_NAME: &str = "course_anchor";
pub const COURSE_ANCHOR_ENTRY_DESCRIPTION: &str =
    "CourseAnchor entry provides constant address for a course to be referenced by";

pub const ERR_ONLY_TEACHER_CAN: &str = "Only the teacher can {} their courses";
pub const ERR_NO_TEACHER_CHANGE: &str = "Cannot change the teacher of the course";

pub const LINK_COURSE_ANCHOR_TO_COURSE: &str = "course_anchor->course";
pub const LINK_TEACHER_TO_COURSE_ANCHOR: &str = "teacher->courses";
pub const LINK_STUDENT_TO_COURSE_ANCHOR: &str = "student->courses";
pub const LINK_COURSE_ANCHOR_TO_STUDENT: &str = "course->students";
