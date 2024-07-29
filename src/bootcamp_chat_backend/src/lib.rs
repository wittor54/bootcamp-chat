use std::{cell::RefCell, collections::HashMap, path::StripPrefixError, string};

thread_local! {
    static NOTES: RefCell<Vec<String>> = RefCell::default();
}


#[ic_cdk::query]
fn get_notes() -> Vec<String> {
    NOTES.with_borrow(|notes |notes.clone())
}


#[ic_cdk::update]
fn add_note(note: String){
    NOTES.with_borrow_mut(|notes|{
        notes.push(note)
    })
}

