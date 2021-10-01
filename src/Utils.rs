use rand::Rng;




pub fn randomize_mat(array: &Vec<Vec<i8>>, radius: u32, chance: f64) -> Vec<Vec<i8>>{
    let mut rng = rand::thread_rng();
    let mut rn: f64;

    let center_y = (array.len() as f32 / 2.0).round() as i32;
    let center_x = (array[0].len() as f32 / 2.0).round() as i32;
    let mut arr2 = array.clone();
    for y in 1..radius*2{
        for x in 1..radius*2{
            rn = rng.gen();
            if rn <= chance{
                arr2[(center_y - radius as i32 + y as i32) as usize][(center_x - radius as i32 + x as i32) as usize] = 1;
            }
            else{
                let y_val = (center_y - radius as i32 + y as i32) as usize;
                let x_val = (center_x - radius as i32 + x as i32) as usize;
                arr2[y_val][x_val]
                    = array[y_val][x_val];
            }
        }
    }
    return arr2;
}

pub fn randomize_entire_mat(array: &Vec<Vec<i8>>, chance: f64) -> Vec<Vec<i8>>{
    let mut rng = rand::thread_rng();
    let mut rn: f64;

    let mut arr2 = array.clone();
    for y in 1..array.len(){
        for x in 1..array[y].len(){
            rn = rng.gen();
            if rn <= chance{
                arr2[y][x] = 1;
            }
            else{
                arr2[y][x] = array[y][x];
            }
        }
    }
    return arr2;
}

pub fn update_cell(array: &Vec<&Vec<i8>>, pos_yr: &i64, pos_xr: &i64) -> i8{
    let pos_y = *pos_yr;
    let pos_x = *pos_xr;
    let mut neighbors = 0;
    if pos_y - 1 >= 0{ //check top
        neighbors += array[(pos_y -1) as usize][pos_x as usize];
        if pos_x - 1 >= 0{
            neighbors += array[(pos_y -1) as usize][(pos_x -1) as usize];
        }
        if pos_x + 1 < array[pos_y as usize].len() as i64{
            neighbors += array[(pos_y -1) as usize][(pos_x +1) as usize];
        }
    }
    else{
        neighbors+=10;
    }
    if pos_y + 1 < array.len() as i64{
        neighbors += array[(pos_y +1) as usize][pos_x as usize];
        if pos_x - 1 >= 0{
            neighbors += array[(pos_y +1) as usize][(pos_x -1) as usize];
        }
        if pos_x + 1 < array[pos_y as usize].len() as i64 {
            neighbors += array[(pos_y + 1) as usize][(pos_x + 1) as usize];
        }
    }
    else{
        neighbors += 10;
    }
    if pos_x - 1 >= 0{
        neighbors += array[pos_y as usize][(pos_x - 1) as usize];
    }
    else{
        neighbors += 10;
    }
    if pos_x + 1 < array[pos_y as usize].len() as i64{
        neighbors += array[pos_y as usize][(pos_x + 1) as usize];
    }
    else{
        neighbors += 10
    }

    return if neighbors < 2 {
        0
    } else if neighbors > 3 {
        0
    } else if array[pos_y as usize][pos_x as usize] == 1 && neighbors >= 2 && neighbors <= 3{
        1
    } else if array[pos_y as usize][pos_x as usize] == 0 && neighbors == 3{
        1
    } else {
        0
    }
}


pub fn update_toroidal(array: &Vec<&Vec<i8>>, pos_yr: &i64, pos_xr: &i64) -> i8{
    let pos_y = *pos_yr;
    let pos_x = *pos_xr;
    let mut neighbors = 0;
    if pos_y == 1{ //check top
        neighbors += array[(array.len()-1) as usize][pos_x as usize];
        if pos_x == 1{ // top left
            neighbors += array[(array.len()-1) as usize][(array[pos_y as usize].len()-1) as usize];
        }
        else{ // top left across
            neighbors += array[(array.len()-1) as usize][(pos_x -1) as usize];
        }
        if pos_x == (array[pos_y as usize].len() - 1 as usize) as i64{
            neighbors += array[(array.len()-1) as usize][1 as usize];
        }
        else{ // top left across
            neighbors += array[(array.len()-1) as usize][(pos_x +1) as usize];
        }
    }
    else{ //check top across
        neighbors += array[(pos_y -1) as usize][pos_x as usize];
        if pos_x == 1{ // top left
            neighbors += array[(pos_y-1) as usize][(array[pos_y as usize].len()-1) as usize];
        }
        else{ // top left across
            neighbors += array[(pos_y -1) as usize][(pos_x -1) as usize];
        }
        if pos_x == (array[pos_y as usize].len() - 1 as usize) as i64{
            neighbors += array[(pos_y-1) as usize][1 as usize];
        }
        else{ // top left across
            neighbors += array[(pos_y -1) as usize][(pos_x +1) as usize];
        }

    }
    if pos_y == (array.len() - 1) as i64{
        neighbors += array[1 as usize][pos_x as usize];
        if pos_x == 1{ // check bottom left
            neighbors += array[1 as usize][(array[pos_y as usize].len() - 1 as usize) as usize];
        }
        else{
            neighbors += array[1 as usize][(pos_x -1) as usize];
        }
        if pos_x == (array[pos_y as usize].len() - 1 as usize) as i64{
            neighbors += array[1 as usize][1 as usize];
        }
        else{
            neighbors += array[1 as usize][(pos_x + 1) as usize];
        }
    }
    else{
        neighbors += array[(pos_y +1) as usize][pos_x as usize];
        if pos_x == 1{ // check bottom left
            neighbors += array[(pos_y + 1) as usize][(array[pos_y as usize].len() - 1 as usize) as usize];
        }
        else{
            neighbors += array[(pos_y +1) as usize][(pos_x -1) as usize];
        }
        if pos_x == (array[pos_y as usize].len() - 1) as i64 {
            neighbors += array[(pos_y + 1) as usize][1 as usize];
        }
        else{
            neighbors += array[(pos_y + 1) as usize][(pos_x + 1) as usize];
        }

    }
    if pos_x == 1{
        neighbors += array[pos_y as usize][(array[pos_y as usize].len() - 1) as usize];
    }
    else{

        neighbors += array[pos_y as usize][(pos_x - 1) as usize];
    }
    if pos_x == (array[pos_y as usize].len() - 1 as usize) as i64{
        neighbors += array[pos_y as usize][1 as usize];
    }
    else{
        neighbors += array[pos_y as usize][(pos_x + 1) as usize];
    }



    return if neighbors < 2 {
        0
    } else if neighbors > 3 {
        0
    } else if array[pos_y as usize][pos_x as usize] == 1 && neighbors >= 2 && neighbors <= 3{
        1
    } else if array[pos_y as usize][pos_x as usize] == 0 && neighbors == 3{
        1
    } else {
        0
    }
}

