use std::ffi::OsStr;
use chrono::{DateTime, NaiveDate, Utc, Datelike, Weekday};
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct DayData {
    pub date: DateTime<Utc>,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64,
}

impl DayData {
    pub fn new(date: DateTime<Utc>, high: f64, low: f64, open: f64, close: f64) -> DayData {
        DayData {
            date,
            high,
            low,
            open,
            close,
        }
    }
}

#[derive(Debug)]
pub struct StockData<'a> {
    pub stock: &'a str,
    pub days: Vec<DayData>,
}

impl StockData<'_>{
    pub fn new(stock: &str, days: Vec<DayData>) -> StockData{
        StockData{
            stock,
            days,
        }
    }
}

pub fn read_data<'a>(filename: &'a OsStr, stock: &'a str) -> StockData<'a> {
    let mut rdr = csv::Reader::from_path(filename).unwrap();
    let mut data = vec![];
    let mut temp = [0.0;4];

    for result in rdr.records() {
        let record = result.unwrap();
        for i in 0..4 {
            let t = record.get(i+1).unwrap();
            temp[i as usize] = match t.parse::<f64>() {
                Ok(a) => a,
                Err(_) => continue,
            }
        }
        let date = record.get(0).unwrap();
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        let date = date.and_hms_opt(0,0,0).expect("REASON")
                   .and_local_timezone(Utc).unwrap();
        data.push(DayData::new(date, temp[1], temp[2], temp[0], temp[3]));
    }
    StockData::new(stock,data)
}

#[derive(Clone,Debug,Copy)]
pub struct Point {
    pub score: i32,
    pub var: i32,
}

impl Point {
    pub fn new(var:i32) -> Point {
        Point {
            score:0,
            var:var,
        }
    }
}

impl Into<(i32,i32)> for Point {
    fn into(self) -> (i32,i32){
        (self.var,self.score)
    }
}

#[derive(Debug)]
pub struct Line {
    pub points: Vec<Point>,
    pub info: String,
}

impl Line {
    pub fn new(info: String,points: &Vec<Point>) -> Line {
        let data = points.clone();
        Line {
            points:data,
            info:info,
        }
    }
}

type Cmp_fn = fn(f64,f64) -> Option<Ordering>;
pub fn analyse_data(stock_data: & mut StockData, highest:bool) -> Vec<Line> {
    let data = & mut stock_data.days;
    data.sort_by_key(|a| a.date);

    let mut ret = Vec::with_capacity(5);

    let points:Vec<Point> = (0..8).map(|a|Point::new(a)).collect();
    for i in 0..5 {
        ret.push(Line::new(format!("{}",Weekday::try_from(i).unwrap()),
                           &points));
    }
    
    let mut iter = data.iter().peekable();

    'outer: while iter.peek() != None {
        while iter.peek().unwrap().date.weekday() != Weekday::Mon {
            iter.next();
            if iter.peek() == None {
                break 'outer;
            }
        }
        let mut days:[(f64,Weekday);5] = [(0.0,Weekday::Sun);5];
        for i in 0..5 {
            let day = iter.peek();
            if day == None {
                break 'outer;
            }
            if day.unwrap().date.weekday().num_days_from_monday() != i {
                continue 'outer;
            }
            let day = iter.next();
            days[i as usize] = (day.unwrap().close,
                                day.unwrap().date.weekday());
        }
        if highest {
            days.sort_by(|(a,_),(b,_)|a.partial_cmp(b).unwrap());
        } else {
            days.sort_by(|(a,_),(b,_)|b.partial_cmp(a).unwrap());
        }
        let highest = days[4].1;
        let second = days[3].1;

        for i in 0..=7 {
            ret[highest.num_days_from_monday() as usize]
                .points[i as usize].score += 10;
            ret[second.num_days_from_monday() as usize]
                .points[i as usize].score += i;
        }

    }

    ret 
}
