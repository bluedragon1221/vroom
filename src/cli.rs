use crate::{
    error::VroomError,
    structure::{Item, List, Vroomfile},
};

// The delete command will delete something. easy!
pub fn delete_cmd(
    args: (Option<&String>, Option<&String>),
    vroomfile: &mut Vroomfile,
) -> Result<(), VroomError> {
    match args {
        (None, _) => unreachable!(),
        (Some(l_name), None) => {
            println!("Deleting list '{}'", l_name);
            vroomfile.rm_list(l_name)?;
            Ok(())
        }
        (Some(maybe_list_name), Some(maybe_item_name)) => {
            let list: &mut List = vroomfile.get_mut_list(maybe_list_name)?;
            let list_name = &list.get_name();

            let item: &mut Item = list.get_mut_item(maybe_item_name)?;
            let item_name = &item.get_name();

            println!("Deleting '{}' from '{}'", item_name, list_name);
            list.rm_item(item_name);
            Ok(())
        }
    }
}

// The recall command will find what the user wants and print it to stdout.
// It can print whole lists or the whole vroomfile if requested
pub fn recall_cmd(
    args: (Option<&String>, Option<&String>),
    vroomfile: &mut Vroomfile,
) -> Result<(), VroomError> {
    match args {
        (None, _) => Err(VroomError::MissingArgument("item_name or list_name".into())),
        (Some(something), None) => {
            // if it is a valid list name, then print the list
            if let Ok(list) = vroomfile.get_mut_list(something) {
                println!("{}", list.fmt(false));
                Ok(())
            // but if it isn't a valid list name...
            } else {
                // if it's a valid item name, then get the value
                if let Ok(item) = vroomfile.get_mut_item(something) {
                    println!("{}", item.get_value());
                    Ok(())
                } else {
                    Err(VroomError::NoSuchItemAny(something.into()))
                }
            }
        }
        (Some(maybe_list_name), Some(maybe_item_name)) => {
            let list = vroomfile.get_mut_list(maybe_list_name)?;
            let item = list.get_mut_item(maybe_item_name)?;

            println!("{}", item.get_value());
            Ok(())
        }
    }
}

// The fetch command will retrive the item that the user wants and return it.
// It can not retrive a list of items
pub fn fetch_cmd(
    args: (Option<&String>, Option<&String>),
    vroomfile: &mut Vroomfile,
) -> Result<String, VroomError> {
    match args {
        (None, _) => Err(VroomError::MissingArgument("item_name or list_name".into())),
        (Some(something), None) => {
            if let Ok(_list) = vroomfile.get_mut_list(something) {
                Err(VroomError::MissingArgument("item_name".into()))
            } else {
                if let Ok(item) = vroomfile.get_mut_item(something) {
                    Ok(item.get_value())
                } else {
                    Err(VroomError::NoSuchItemAny(something.into()))
                }
            }
        }
        (Some(maybe_list_name), Some(maybe_item_name)) => {
            let list = vroomfile.get_mut_list(maybe_list_name)?;
            let item = list.get_mut_item(maybe_item_name)?;

            Ok(item.get_value())
        }
    }
}

// The add command will add a new item to the vroomfile, given the list to add it to.
// If only one argument is specified, it will create a new list with that name.
pub fn add_cmd(
    args: (Option<&String>, Option<&String>, Option<&String>),
    vroomfile: &mut Vroomfile,
) -> Result<(), VroomError> {
    match args {
        (None, _, _) => Err(VroomError::MissingArgument(
            "list_name, item_name, value".into(),
        )),
        (Some(maybe_list_name), None, _) => {
            if let Err(_list) = vroomfile.get_mut_list(maybe_list_name) {
                vroomfile.add_list(maybe_list_name);
                Ok(())
            } else {
                Err(VroomError::MissingArgument("list_name".into()))
            }
        }
        (Some(_), Some(_), None) => Err(VroomError::MissingArgument("value".into())),
        (Some(maybe_list_name), Some(maybe_item_name), Some(value)) => {
            let list = vroomfile.get_mut_list(maybe_list_name)?;
            if let Ok(item) = list.get_mut_item(maybe_item_name) {
                item.set_value(value);
                Ok(())
            } else {
                list.add_item(maybe_item_name, value);
                Ok(())
            }
        }
    }
}
