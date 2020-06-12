use super::anchor::{AnchorTrait, TheCourseAnchor};
use super::entry::Course;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    let new_course = Course::new(title, timestamp);
    let course_address = hdk::commit_entry(&new_course.entry())?;

    hdk::link_entries(
        &TheCourseAnchor {}.address()?,
        &course_address,
        TheCourseAnchor::link_type(),
        "".to_string(),
    )?;

    Ok(course_address)
}

pub fn update(
    title: String,
    modules_addresses: Vec<Address>,
    course_address: &Address,
) -> ZomeApiResult<Address> {
    let course: Course = hdk::utils::get_as_type(course_address.clone())?;

    let new_version_course = Course::update_from(course, title, modules_addresses);

    // Remove link from Anchor to old version of course
    hdk::remove_link(
        &TheCourseAnchor {}.address()?,
        &course_address,
        TheCourseAnchor::link_type(),
        "".to_string(),
    )?;

    let updated_course_address = hdk::update_entry(new_version_course.entry(), course_address)?;

    // Add new link from Anchor to new version of course
    hdk::link_entries(
        &TheCourseAnchor {}.address()?,
        &updated_course_address,
        TheCourseAnchor::link_type(),
        "".to_string(),
    )?;

    Ok(updated_course_address)
}
