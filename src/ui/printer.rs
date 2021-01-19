use crate::resources::issue::Issue;
use crate::resources::issues::Issues;
use crate::resources::project::Project;
use crate::resources::projects::Projects;
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

impl Printer for Issues {
    fn print_result(self) {
        for issue in self.issues.into_iter() {
            println!();
            println!("ID: {}", Cyan.paint(issue.id.to_string()));
            println!("Author:  {}", Cyan.paint(issue.author.name));
            println!("Project: {}", Cyan.paint(issue.project.name));
            println!("Tracker: {}", Cyan.paint(issue.tracker.name));
            println!("Subject: {}", Cyan.paint(issue.subject));
            println!(
                "Description: {}",
                Cyan.paint(
                    issue
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
}

impl Printer for Project {
    fn print_result(self) {
        println!();
        println!("ID: {}", Cyan.paint(self.project.id.to_string()));
        println!("Project: {}", Cyan.paint(self.project.name));
        println!(
            "Description: {}",
            Cyan.paint(
                self.project
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

impl Printer for Projects {
    fn print_result(self) {
        for project in self.projects.into_iter() {
            println!();
            println!("ID: {}", Cyan.paint(project.id.to_string()));
            println!("Project: {}", Cyan.paint(project.name));
            println!(
                "Description: {}",
                Cyan.paint(
                    project
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
}
