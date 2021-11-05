use std::rc::Rc;

use rand::{prelude::Distribution};
sixtyfps::include_modules!();

fn main() {
    let ui = AppWindow::new();
    let w : usize = 10;
    let h : usize = 10;

    let mut minefield_vec : Vec<TileData> = rand::distributions::Bernoulli::new(0.1).unwrap().sample_iter(rand::rngs::OsRng::default()).map(|m| TileData{is_mine: m, opened:true, ..Default::default()}).take(w*h).collect();
    let mine_positions : Vec<usize> = minefield_vec.iter().enumerate().filter(|(_,t)| t.is_mine).map(|(i,_)| i).collect();
    for p in mine_positions {
        let p = p as i32;
        let w = w as i32;
        let h = h as i32;
        for ni in [-w-1, -w, -w+1, -1, 1, w-1, w, w+1].iter().map(|x| p+x ).filter(|x| *x >=0 && *x< w*h ) {
            let ni = ni as usize;
            if !minefield_vec[ni].is_mine {
                minefield_vec[ni].neighbours += 1;
            }
        }
    }
    let minefield_model = Rc::new(sixtyfps::VecModel::<TileData>::from(minefield_vec));
    ui.set_minefield_model(sixtyfps::ModelHandle::new(minefield_model));
    ui.set_row_length(w as i32);
    ui.run();
}
