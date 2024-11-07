use rquickjs::{Context, Runtime, JsValue};
use std::env;
use std::fs;
use std::process;

fn main() {
    // Pobieramy argumenty wejściowe
    let args: Vec<String> = env::args().collect();

    // Sprawdzamy, czy podano odpowiednie argumenty
    if args.len() < 3 || args[1] != "compile" {
        eprintln!("Użycie: rstab compile <nazwa_pliku>");
        process::exit(1);
    }

    // Pobieramy nazwę pliku
    let file_name = &args[2];

    // Ładujemy kod JavaScript z pliku
    let js_code = fs::read_to_string(file_name).expect("Nie udało się odczytać pliku");

    // Uruchamiamy kod JavaScript przy użyciu QuickJS
    let runtime = Runtime::new().unwrap();
    let ctx = Context::full(&runtime).unwrap();
    ctx.with(|ctx| {
        let result: JsValue = ctx.eval(&js_code).unwrap();
        println!("{:?}", result);
    });
}
