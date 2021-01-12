use crate::resources::issue::Issue;
use ansi_term::Colour::Cyan;

pub trait Printer {
    fn print_result(self);
}

impl Printer for Issue {
    fn print_result(self) {
        println!("ID: {}", Cyan.paint(self.issue.id.to_string()));
        println!("Author:  {}", Cyan.paint(self.issue.author.name));
        println!("Project: {}", Cyan.paint(self.issue.project.name));
        println!("Tracker: {}", Cyan.paint(self.issue.tracker.name));
        println!("Subject: {}", Cyan.paint(self.issue.subject));
        println!(
            "Description: {}",
            Cyan.paint(
                self.issue
                    .description
                    .replace("\r\n", "")
                    .replace("\t\t", "")
                    .replace("\n", "")
                    .replace("\\", "")
                    .replace("  ", "")
            )
        );
    }
}
