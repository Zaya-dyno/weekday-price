use crate::fin_data::*;
use plotters::prelude::*;
use std::ffi::OsStr;

static colors_: [&RGBColor;5] = [&RED,&GREEN,&BLUE,&full_palette::PURPLE,&MAGENTA];

fn find_max_line(line: &Line) -> i32{
    return line.points.iter().max_by_key(|a|a.score).unwrap().score;
}
pub fn draw_graph(ret: Vec<Line>,file_dest: &OsStr, label: &str){
    let mut max = ret.iter().map(|a| find_max_line(a)).max().unwrap();
    max += 100;
    
    let drawing_area = BitMapBackend::new(file_dest, (1024, 768))
        .into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&drawing_area)
                    .caption(label, ("Arial",30))
                    .set_label_area_size(LabelAreaPosition::Left, 40)
                    .set_label_area_size(LabelAreaPosition::Bottom, 40)
                    .build_cartesian_2d(0..8,0..max)
                    .unwrap();


    for (i,line) in ret.iter().enumerate() {
        let color = colors_[i].clone();
        println!("{:#?}",line.points);
        ctx.draw_series(LineSeries::new(
                line.points.iter().map(|a| (*a).into()),
                colors_[i],
                )).unwrap()
                .label(&line.info)
                .legend(move |(x,y)| PathElement::new(vec![(x,y),(x+20,y)],
                                                 color));
    }
    ctx.configure_mesh().draw().unwrap();
    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
}
