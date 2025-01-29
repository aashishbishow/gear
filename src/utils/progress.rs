use indicatif::{ProgressBar, ProgressStyle};

// Function to create a progress bar
pub fn create_progress_bar() -> ProgressBar {
    let bar = ProgressBar::new(4);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
            .unwrap()
            .progress_chars("\u{2699} >-"),
    );

    bar
}