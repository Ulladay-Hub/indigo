use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Print {
        #[arg(short, long)]
        option: Option<String>,
    },
    Add {
        #[arg(short = 'a', long)]
        option1: Option<i32>,
        #[arg(short = 'b', long)]
        option2: Option<i32>,
    },
    Subtract {
        #[arg(short = 'a', long)]
        option1: Option<i32>,
        #[arg(short = 'b', long)]
        option2: Option<i32>,
    },
    Multiply {
        #[arg(short = 'a', long)]
        option1: Option<i32>,
        #[arg(short = 'b', long)]
        option2: Option<i32>,
    },
    Divide {
        #[arg(short = 'a', long)]
        option1: Option<i32>,
        #[arg(short = 'b', long)]
        option2: Option<i32>,
    },
    Modulus {
        #[arg(short = 'a', long)]
        option1: Option<i32>,
        #[arg(short = 'b', long)]
        option2: Option<i32>,
    },
    Power {
        #[arg(short = 'a', long)]
        option1: Option<i32>,
        #[arg(short = 'b', long)]
        option2: Option<i32>,
    },
    Drv {
        #[arg(short, long)]
        path: Option<String>,
    },
    Help {
        #[arg(short, long)]
        command: Option<String>,
    },
}
