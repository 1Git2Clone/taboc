mod app;
mod prelude;
mod utils;

use prelude::*;

fn main() -> Result<(), AppError> {
    App::init()?.run()
}
