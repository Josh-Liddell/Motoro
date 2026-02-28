mod dialog;

use clap::{Parser, Subcommand};
use jlrs::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // #[arg(long)]
    // one: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Play,
    Dialog,
    Price,
    Dowhat,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Play | Commands::Price => {
            // this just has to be created once so do the thing
            let handle = Builder::new().start_local().expect("cannot init Julia");

            unsafe {
                handle
                    .include("julia/Motoro/src/Motoro.jl")
                    .expect("Failed to load Motoro.jl");
            }

            match args.command {
                Commands::Play => handle.local_scope::<_, 3>(|mut frame| {
                    let game_func = Module::main(&frame)
                        .submodule(&mut frame, "Motoro")
                        .expect("Motoro not found")
                        .global(&mut frame, "game")
                        .expect("game not found");

                    unsafe {
                        game_func.call(&mut frame, []).expect("exception in game()");
                    }
                }),
                Commands::Price => handle.local_scope::<_, 19>(|mut frame| {
                    let inputs = dialog::get_option_inputs();
                    // let option_type = JuliaString::from(inputs.option_type);
                    // converts data from Rust to Julia
                    let strike = Value::new(&mut frame, inputs.strike);
                    let expiry = Value::new(&mut frame, inputs.expiry);
                    let binomial = Value::new(&mut frame, inputs.binomial);
                    let spot = Value::new(&mut frame, inputs.spot);
                    let rate = Value::new(&mut frame, inputs.rate);
                    let vol = Value::new(&mut frame, inputs.vol);
                    let div = Value::new(&mut frame, inputs.div);

                    let option = unsafe {
                        Module::main(&frame)
                            .submodule(&mut frame, "Motoro")
                            .expect("Motoro not found")
                            .global(&mut frame, inputs.option_type) // this gets the custome type
                            .unwrap()
                            .call(&mut frame, [strike, expiry]) // this instantiates the custom type, the one unsafe part
                            .expect("cannot call constructor of CustomType")
                    };

                    let engine = unsafe {
                        Module::main(&frame)
                            .submodule(&mut frame, "Motoro")
                            .expect("Motoro not found")
                            .global(&mut frame, "Binomial")
                            .unwrap()
                            .call(&mut frame, [binomial])
                            .expect("cannot call constructor of CustomType")
                    };

                    let data = unsafe {
                        Module::main(&frame)
                            .submodule(&mut frame, "Motoro")
                            .expect("Motoro not found")
                            .global(&mut frame, "MarketData")
                            .unwrap()
                            .call(&mut frame, [spot, rate, vol, div])
                            .expect("cannot call constructor of CustomType")
                    };

                    let price_func = Module::main(&frame)
                        .submodule(&mut frame, "Motoro")
                        .expect("Motoro not found")
                        .global(&mut frame, "price")
                        .expect("price not found");

                    let result = unsafe {
                        price_func
                            .call(&mut frame, [option, engine, data])
                            .expect("exception in price()")
                    };

                    let price = result.unbox::<f64>().expect("price did not return Float64");

                    println!("\nResult: {price}");
                }),
                _ => {}
            }
        }
        Commands::Dialog => dialog::example_thing(),
        _ => println!("Not implemented yet"),
    }

    // println!("two: {:?}", args.two);
    // println!("one: {:?}", args.one);
}

// unsafe fn motoro_type<N: ToSymbol>(
//     frame: LocalGcFrame<'_, 19>,
//     name: N,
//     args: [Value],
// ) -> Value<'_, '_> {
//     Module::main(frame)
//         .submodule(frame, "Motoro")
//         .expect("Motoro not found")
//         .global(frame, name)
//         .unwrap()
//         .call(frame, args)
// }

// Call a constructor in Motoro submodule with any number of arguments
// unsafe fn motoro_type<'a>(
//     frame: &mut LocalFrame<'a>,
//     name: &str,
//     args: &[Value<'a, 'a>],
// ) -> Value<'a, 'a> {
//     let module = Module::main(frame)
//         .submodule(frame, "Motoro")
//         .expect("Motoro not found");

//     let constructor = module.global(frame, name).expect("constructor not found");

//     constructor
//         .call(frame, args)
//         .expect("cannot call constructor of CustomType")
// }
