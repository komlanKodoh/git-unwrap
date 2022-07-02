use clap::Parser;
use git_unwrap::clone;
use git_unwrap::CloneConfig;

const AUTHOR: &str = "\nKomlan Kodoh\n\n";
const ABOUT: &str = "Simple command line application that clones remote git repositories and unwraps them as separate folder";
const LONG_ABOUT : &str = "I am currently working as an instructor for a high school boot camp. As I wanted to work on the curriculum, I wanted to use git to separate different stages of progress. However, while this structure would have been advantageous for me, I would require the student to use git to navigate between the stages. Because git was beyond the scope of our camp, I would have been forced to use a more friendly folder structure. I decided to ,instead, develop git-unwrap to transform my git structure into a simple file tree.  ";

#[derive(Parser, Debug)]
#[clap(author = AUTHOR , version, about = ABOUT, long_about = LONG_ABOUT)]
enum Args {
    Clone(CloneConfig),
}

fn main() {
    let args = Args::parse();

    match args {
        Args::Clone(mut config) => {
            clone(&mut config);
        }
    }
}
