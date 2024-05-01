use clap::Parser;
use db_introspect::get_tables;
mod db_introspect;
mod faking;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    connection_string: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let table_definitions = get_tables(&args.connection_string).await?;

    for table_definition in table_definitions.iter() {
        println!("table_name: {}", table_definition.table_name);
        for column_definition in table_definition.column_definitions.iter() {
            println!(
                "  column_name: {} {}",
                column_definition.column_name, column_definition.data_type
            );
        }
    }

    Ok(())
}
