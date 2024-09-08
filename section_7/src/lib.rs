mod list {

    pub struct Tasks {
        pub item: String,
    }
}

mod things_todo;
use crate::things_todo::add_activity;
use things_todo::items_completed;
use things_todo::items_completed::test::do_some_test;

fn lets_add_task() {
    let task = list::Tasks {
        item: String::from("tasks"),
    };
    add_activity();
    items_completed::remove_task();
    do_some_test();
}
