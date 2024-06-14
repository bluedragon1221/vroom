use crate::structure::{List, Vroomfile};

impl List {
    pub fn fmt(&self, tab: bool) -> String {
        let mut res: Vec<String> = Vec::new();

        let str_len: usize = self
            .contents
            .iter()
            .map(|x| x.get_name().len())
            .max()
            .unwrap();

        let tab_w = String::from(if tab { "  " } else { "" });

        for (index, item) in self.contents.iter().enumerate() {
            let mut line: Vec<String> = Vec::new();
            line.push(tab_w.clone());
            line.push(item.get_name());
            line.push(String::from(": "));
            (0..str_len - item.get_name().len()).for_each(|_| line.push(String::from(" ")));
            line.push(item.get_value());

            if index != self.contents.len() - 1 {
                line.push(String::from("\n"))
            };

            res.append(&mut line)
        }

        res.join("")
    }
}

impl Vroomfile {
    pub fn fmt_overview(&self) -> String {
        let mut res: Vec<String> = Vec::new();
        for i in self.contents.iter() {
            let item_count = i.contents.len();
            res.push(format!("{} ({})", i.get_name(), item_count));
        }

        res.join("\n")
    }

    pub fn fmt_all(&self) -> String {
        let mut res: Vec<String> = Vec::new();
        for (index, item) in self.contents.iter().enumerate() {
            res.push(format!("{}\n", item.get_name()));
            res.push(item.fmt(true));
            if index != self.contents.len() - 1 {
                res.push("\n".to_string())
            };
        }
        res.join("")
    }
}
