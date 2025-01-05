use clap::Parser;

mod cnf;

mod digraph;
use digraph::digraph_2sat;

mod dpll;
use dpll::dpll_sat;

mod plot;
use plot::plot_series;

mod rand_cnf;
use rand_cnf::generate_cnf;


/// Investigate phase transition in k-SAT problems
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The number k of literals per clause (e.g. 3 for 3-SAT)
    #[arg(long, short = 'k', default_value_t = 3)]
    k: u8,

    /// The number n of available variables
    #[arg(long, short = 'n', default_value_t = 25)]
    n: u32,

    /// The number of generated samples per point (s.p.p.)
    #[arg(long, short = 's', default_value_t = 100)]
    samples: u32,

    /// Lower bound for values of alpha
    #[arg(long, default_value_t = 0.)]
    alpha_start: f32,

    /// Upper bound for values of alpha
    #[arg(long, default_value_t = 10.)]
    alpha_end: f32,

    /// Number of values for alpha
    #[arg(long, default_value_t = 100)]
    alpha_steps: usize,

    /// Verbosity (when turned on, the computed values are displayed)
    #[arg(long)]
    verbose: bool,
}


fn main() {
    let cli = Cli::parse();

    let alphas: Vec<f32> = (0..=cli.alpha_steps)
        .map(|i|
            cli.alpha_start + (cli.alpha_end - cli.alpha_start) * (i as f32) / cli.alpha_steps as f32)
        .collect();
    let values = alphas
        .iter()
        .map(|alpha| {
            if cli.verbose {
                println!("alpha = {}", alpha);
            }
            (0..cli.samples)
                .filter(|_| {
                    let cnf = generate_cnf(cli.k, cli.n, *alpha, None);
                    if cli.k == 2 {
                        digraph_2sat(&cnf)
                    } else {
                        dpll_sat(&cnf)
                    }
                })
                .count() as f32
                / cli.samples as f32
        })
        .collect();
    if cli.verbose {
        println!("alphas: {:?}", alphas);
        println!("values: {:?}", values);
    }
    match plot_series(format!("{}-SAT, N={} ({} s.p.p.)", cli.k, cli.n, cli.samples), alphas, values) {
        Ok(path) => println!("Generated file {}", path),
        Err(err) => panic!("An error occurred while generating the plot: {:?}", err),
    }
}
