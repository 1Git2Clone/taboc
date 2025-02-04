mod app;
mod macros;
mod prelude;
mod utils;

use prelude::*;

fn main() -> Result<(), Error> {
    App::init()?.run()
}
