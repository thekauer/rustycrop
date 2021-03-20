extern crate bmp;
use bmp::Image;



use std::path::Path;


type bitmap_t = Vec<Vec<bool>>;
type normmap_t = Vec<bool>;
type bitmaps = Vec<bitmap_t>;





fn read_bmp(path:&str) -> bitmap_t{
    let thresh=128;
    let img = bmp::open(path).unwrap();
    let  w = img.get_width() as usize;
    let  h = img.get_height() as usize;
    let mut bitmap = vec![vec![false;h];w];
    for i in 0..w {
        for j in 0..h {
            let mut color = false;
            let px = img.get_pixel(i as u32,j as u32);
            //if px.==bmp::consts::BLACK {
            if (px.r>thresh) && (px.b >thresh) && (px.g >thresh) {
                color=false;
            } 
            else {
                color=true;
            }
            bitmap[i][j]= color;
        }
    }
    bitmap


}

fn normalize(bm : &bitmap_t) -> normmap_t {
     let w=bm.len();
    let h=bm[0].len();
    
    
    let mut norm = vec![false;w];
    for i in 0..w {
        for j in 0..h {
            if bm[i][j]==true {
                norm[i]=true;
                break;
            }
        }
    }

    
    
    


    
    
    for i in 1..(w-1) {
        if norm[i-1] && !norm[i] && norm[i+1] {
            norm[i]=true;
        }
    }
    
    norm
}

fn normalize_y(bm : &bitmap_t) -> normmap_t {
    let w=bm.len();
    let h=bm[0].len();
    
    let mut norm = vec![false;h];
    
    for i in 0..h {
    for j in 0..w {
        if bm[j][i]==false {
           
            norm[j]=true;
            break;
        }
    }
    }
    



    
    norm

}


fn crop(bm: &bitmap_t, norm: &normmap_t) -> bitmaps {
    let w = bm.len();
    let h = bm[0].len();
    let mut res : bitmaps= vec![];
    let mut iter=0;
    
    while iter<w {
        let mut tmp : bitmap_t = vec![];
        if norm[iter] {
            //tmp.push((&bm[iter]).to_owned());
            //iter+=1;
            
            while norm[iter] {
                tmp.push((&bm[iter]).to_owned());
                
                iter+=1;
            }
            
            res.push(tmp);
            
        }
        iter+=1;
    }

    res
}




fn crop_y(bm:&bitmap_t,norm:&normmap_t) -> bitmaps {
    let w = bm.len();
    let h = bm[0].len();
    
    let mut res : bitmaps= vec![];
    let mut iter=0;
    while iter<h {
        let mut tmp:bitmap_t=vec![];
        if norm[iter] {
            while norm[iter] {
                let mut row=vec![];
                    for j in 0..w {
                        row.push(bm[j][iter]);
                    }
                tmp.push(row);
                iter+=1;
            }
            res.push(tmp);
        }
        
        iter+=1;
    }


    res
}


fn save(path:&str,crops: &bitmaps,start:usize,saved:&mut usize) {
 let mut fpath = path.to_string();
 let mut imgc=start;
    for k in 0..crops.len() {
        let w = crops[k].len();
        //quick fix
        if w < 10 {
            continue;
        }
        //end of quick fix
        let h = crops[k][0].len();
        let mut timg = Image::new(w as u32,h as u32);
        
        for i in 0..w {
            for j in 0..h {
                let mut px = bmp::consts::BLACK;
                if crops[k][i][j] {
                    px = bmp::consts::WHITE;
                }

                timg.set_pixel(i as u32, j as u32, px);
            }
        }
        fpath+= &imgc.to_string();
        fpath+=".bmp";
        timg.save(fpath.to_string());
        fpath=path.to_string();
        imgc+=1;
    }
}





fn save2(path:&str,crops: &bitmaps) {
 let mut fpath = path.to_string();
 let mut imgc=0;
    for k in 0..crops.len() {
        let w = crops[k].len();
        //quick fix
        if w < 10 {
            continue;
        }
        //end of quick fix
        let h = crops[k][0].len();
        let mut timg = Image::new(w as u32,h as u32);
        
        for i in 0..w {
            for j in 0..h {
                let mut px = bmp::consts::BLACK;
                if crops[k][i][j] {
                    px = bmp::consts::WHITE;
                }

                timg.set_pixel(i as u32, j as u32, px);
            }
        }
        fpath+= &imgc.to_string();
        fpath+=".bmp";
        timg.save(fpath.to_string());
        fpath=path.to_string();
        imgc+=1;
    }
}




fn ps_num(num:usize) -> String {
    let mut res=num/10;
    let mut count=0;
    while res>1 {
        res=res/10;
        count+=1;
    }
    let mut ret="".to_string();
    for i in 0..(4-res-1) {
        ret+="0";
    }
    ret+=&num.to_string();
    ret
}


fn get_files(rpath:&str) -> Vec<String> {
    let paths = std::fs::read_dir(rpath).unwrap();
    let mut res: Vec<String>= vec![];

    for path in paths {
        res.push( path.unwrap().file_name().to_str().unwrap().to_string() );
    }
    res
}



fn waspxinline(bm :&bitmap_t,h:usize,norm:&mut normmap_t) {
    let w=bm.len();
   
    let mut waspx=false;
    for i in 0..w {
        println!("{:?}", i);
        
        if bm[w][h] {
            waspx=true;
           
            break;
        }
        
    }
    norm.push(waspx);
}

fn trim(bms:&bitmaps) -> bitmaps {
    let mut ret:bitmaps = vec![];
    for e in 0..bms.len() {
    let bm=&bms[e];
    let w = bm.len();
    let h = bm[0].len();
    let mut norm: Vec<bool>=vec![];
   


    for j in 0..h {
    let mut waspx=false;
    for i in 0..w {
        if bm[i][j] {
            waspx=true;
            break;
        }
    }
    norm.push(waspx);
    }

    
    
    
    let mut start =0;
    
    while norm[start]==false && start<norm.len(){
        start+=1;
    }
    

    let mut end=norm.len()-1;
    while norm[end]==false {
        end-=1;
    }
    let mut res: bitmap_t = vec![vec![false;end-start];w];
    
    let rw=res.len();
    let rh=res[0].len();
    

    for j in start..end {
        
        for i in 0..w {
            
            res[i][j-start]=bm[i][j];
            
        }
        
    }




    ret.push(res);
    }

    ret
}

fn main() {
   
    let mut READ_PATH : &str = "D:/to_crop/";
    let mut READ_FNAME :&str ="to_crop";
    
    let mut TO_PATH:&str="D:/crops/";
    let mut TO_FNAME:&str="crop";

    
    let args : Vec<String> = std::env::args().collect();
    





    
    




    let tpath= [TO_PATH,TO_FNAME].concat();
    let mut CROPS : bitmaps=vec![];
    let testy : Vec<_> = get_files(READ_PATH);
    for t in testy {

    let mut saved=0;
    let rpath=[READ_PATH,&t].concat();
    
    println!("{:?}",rpath);
    
    let bm = read_bmp(&rpath);
    let norm = normalize(&bm);
    let crops = crop(&bm,&norm);
    let mut trims = trim(&crops);
    CROPS.append(&mut trims);
    
    }
    println!("saving!", );
    save2(&tpath,&CROPS);
    println!("Done", )
   


}