use super::{clido_dir, format, ArgMatches, Database, Result, Status, Table};

pub fn list(sub_args: &ArgMatches<'_>) -> Result<()> {
    let db = Database::from_path(clido_dir()?)?;
    let todos = db.todos();

    if todos.len() == 0 {
        println!("\nThere were no To-Dos to print! Good job!\n");
        return Ok(());
    }

    let filters = sub_args
        .values_of("filter")
        .map(|args| args.into_iter().map(str::to_string).collect::<Vec<_>>());

    let req_comp = sub_args.is_present("is_comp");
    let req_pend = sub_args.is_present("is_pend");

    let mut table = get_list_table();
    let mut at_least_one = false;

    for (id, todo) in todos.iter().enumerate() {
        if let Some(filter_list) = filters.as_ref() {
            if !filter_list.iter().any(|filter| todo.tags.contains(filter)) {
                continue;
            }
        }

        if (req_comp && todo.status != Status::Complete)
            || (req_pend && todo.status != Status::Pending)
        {
            continue;
        }

        at_least_one = true;

        let id: String = id.to_string();
        let status = todo.status.as_symbol();
        let start = todo.start.date().to_string();
        let priority = todo
            .prio
            .as_ref()
            .map_or(String::from("None"), |p| p.to_string());
        let due_date = todo
            .due
            .map_or_else(|| String::from("None"), |d| d.date().to_string());

        table.add_row(row![c->id, c->status, l->todo.desc, c->start, c->priority, c->due_date,]);
    }

    if at_least_one {
        println!();
        table.printstd();
        println!();
    } else {
        println!("\nThere were no To-Dos to matching those filters!\n");
    }

    Ok(())
}

fn get_list_table() -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER);

    table.set_titles(row![
        bc-> "ID",
        bc-> "Status",
        bl->"Description",
        bc-> "Start",
        bc-> "Priority",
        bc-> "Due Date"
    ]);

    table
}
