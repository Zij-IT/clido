use clap::Args;

use super::{clido_dir, format, ArgMatches, Database, Result, Status, Table};

#[derive(Debug, Args)]
pub struct List {
    #[arg(short = 'p', long = "pending")]
    show_pending: bool,

    #[arg(short = 'c', long = "complete")]
    show_complete: bool,

    #[arg(long = "filter", value_name = "TAGS")]
    filter_tags: Option<Vec<String>>,
}

pub fn list(command: &List) -> Result<()> {
    let db = Database::from_path(clido_dir()?)?;
    let todos = db.todos();

    let todos = todos.iter().enumerate().filter(|(_, todo)| {
        (command.show_complete && todo.status == Status::Complete)
            || (command.show_pending && todo.status == Status::Pending)
            || match command.filter_tags.as_ref() {
                Some(tags) => tags.iter().any(|tag| todo.tags.contains(&tag)),
                None => !command.show_pending && !command.show_complete,
            }
    });

    let mut table = get_list_table();

    for (id, todo) in todos {
        let id: String = id.to_string();
        let status = todo.status.as_symbol();
        let start = todo.start.date().format("%d-%m-%Y").to_string();
        let priority = todo
            .prio
            .as_ref()
            .map_or(String::from("None"), |p| p.to_string());
        let due_date = todo.due.map_or_else(
            || String::from("None"),
            |d| d.date().format("%d-%m-%Y").to_string(),
        );

        table.add_row(row![c->id, c->status, l->todo.desc, c->start, c->priority, c->due_date,]);
    }

    if table.is_empty() {
        println!("\nThere were no To-Dos to matching those filters!\n");
    } else {
        println!();
        table.printstd();
        println!();
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
