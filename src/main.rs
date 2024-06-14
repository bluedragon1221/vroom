use vroom::{
    cli::{add_cmd, delete_cmd, recall_cmd},
    error::VroomError,
    system::{load, save},
};

fn program() -> Result<(), VroomError> {
    let args: Vec<String> = std::env::args().collect();

    let mut vroomfile = load()?;

    match args.get(1).map(|x| x.as_str()) {
        Some("delete") => delete_cmd((args.get(2), args.get(3)), &mut vroomfile)?,
        Some("all") => println!("{}", vroomfile.fmt_all()),
        None => println!("{}", vroomfile.fmt_overview()),
        Some(_) => {
            if args.get(3).is_some() {
                add_cmd((args.get(1), args.get(2), args.get(3)), &mut vroomfile)?
            } else {
                recall_cmd((args.get(1), args.get(2)), &mut vroomfile)?
            }
        }
    };

    save(&vroomfile)?;

    Ok(())
}

fn main() {
    match program() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}
