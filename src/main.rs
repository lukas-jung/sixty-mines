use std::rc::Rc;

sixtyfps::include_modules!();

fn main() {
    let ui = AppWindow::new();
    let minefield_model = Rc::new(sixtyfps::VecModel::<TileData>::from(vec![
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
        TileData {is_mine: false, opened: false, marked: false},
    ]));
    ui.set_minefield_model(sixtyfps::ModelHandle::new(minefield_model));
    ui.run();
}
