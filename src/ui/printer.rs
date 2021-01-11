use crate::resources::issue::Issue;

pub trait Printer {
    fn print_result(self);
}

impl Printer for Issue {
    // WIP
    fn print_result(self) {
        println!("ID: {:?}", self.issue.id);
        println!("Author:  {:?}", self.issue.author.name);
        println!("Project: {:?}", self.issue.project.name);
        println!("Tracker: {:?}", self.issue.tracker.name);
        println!("Subject: {:?}", self.issue.subject);
        // TODO: Trim newline
        println!("Description: {:?}", self.issue.description);
    }
}
