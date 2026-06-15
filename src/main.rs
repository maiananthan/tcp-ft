use clap::Parser;
mod common;
mod recv;
mod send;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Send {
        #[arg(long)]
        recv_addr: String,
        #[arg(long)]
        file: String,
    },
    Recv,
}

fn main() {
    let arg = CliArgs::parse();

    match arg.command {
        Commands::Recv => {
            recv::recv_file();
        }
        Commands::Send { recv_addr, file } => {
            send::send_file(recv_addr, file);
        }
    }

    println!("exiting...");
}

//
// end of file
//
