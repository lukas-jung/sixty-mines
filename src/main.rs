use std::rc::Rc;

use rand::prelude::Distribution;
use sixtyfps::Model;
sixtyfps::include_modules!();

fn main() {
    let ui = AppWindow::new();
    let w : usize = 10;
    let h : usize = 10;

    let mut minefield_vec : Vec<TileData> = rand::distributions::Bernoulli::new(0.1).unwrap().sample_iter(rand::rngs::OsRng::default()).map(|m| TileData{is_mine: m, ..Default::default()}).take(w*h).collect();
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

    ui.set_minefield_model(sixtyfps::ModelHandle::new(minefield_model.clone()));
    ui.on_open_tile( move |click_index| {
        let mut nstack = Vec::new();
        let mut tile = minefield_model.row_data(click_index as usize);
        if tile.neighbours == 0 && !tile.is_mine {
            nstack.push(click_index as usize);
            while let Some(i) = nstack.pop() {
                let mut n = minefield_model.row_data(i as usize);
                if !n.opened {
                    let w = w as i32;
                    let h = h as i32;
                    let i = i as i32;
                    if n.neighbours == 0 {
                        for ni in [-w, -1, 1, w].iter().map(|x| i+x ).filter(|x| *x >= 0 && *x < w*h ) {
                            let other_n = minefield_model.row_data(ni as usize);
                            if !other_n.is_mine && !other_n.opened {
                                nstack.push(ni as usize);
                            }
                        }
                    }
                    n.opened = true;
                    minefield_model.set_row_data(i as usize, n);
                }
            }
        } else {
            tile.opened = true;
            minefield_model.set_row_data(click_index as usize, tile);
        }
    });
    ui.set_row_length(w as i32);
    ui.run();
}
