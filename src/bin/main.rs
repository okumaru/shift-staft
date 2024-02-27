use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use chrono::naive::{NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    count: u32,
    overtime: u32
}

#[derive(Serialize, Debug, Clone)]
struct Shift {
    name: String,
    persons: Vec< Person >
}

fn main() -> std::io::Result<()> {

    let init_year: i32 = env!("YEAR").to_string().parse::<i32>().unwrap();
    let init_month: i32 = env!("MONTH").to_string().parse::<i32>().unwrap();
    let init_sat: u32 = env!("SAT_DATE").to_string().parse::<u32>().unwrap();
    let init_sun: u32 = env!("SUN_DATE").to_string().parse::<u32>().unwrap();
    let init_prev_year: bool = env!("PREV_YEAR").to_string().parse::<bool>().unwrap();
    let init_prev_month: bool = env!("PREV_MONTH").to_string().parse::<bool>().unwrap(); 
    let init_start_period: i32 = 25; // Periode dimulai pertanggal.

    let staftsdb = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("stafts.json")
        .expect("Failed to get stafts.json");

    let mut stafts = serde_json::from_reader::<File, Vec<Person>>(staftsdb)
        .expect("Failed to read file json");

    // dbg!(&stafts);

    let mut sat_weeks_len = 0;
    let mut sun_weeks_len = 0;
    let mut shifts_sat: Vec<Shift> = vec![];
    let mut shifts_sun: Vec<Shift> = vec![];
    let mut shifts: Vec<Shift> = vec![];

    let from_month = if init_prev_month { &init_month - 1 } else { init_month };
    let from_year = if init_prev_year { &init_year - 1 } else { init_year };
    let first_sat_year = NaiveDate::from_ymd_opt(
        from_year.try_into().unwrap(), 
        from_month.try_into().unwrap(), 
        init_sat
    ).unwrap();
    let first_sun_year = NaiveDate::from_ymd_opt(
        from_year.try_into().unwrap(), 
        from_month.try_into().unwrap(), 
        init_sun
    ).unwrap();

    for (_idx, d_sat) in first_sat_year.iter_weeks().enumerate() {
        let _year = d_sat.format("%Y").to_string().parse::<i32>().unwrap();
        let month = d_sat.format("%m").to_string().parse::<i32>().unwrap();
        let day = d_sat.format("%d").to_string().parse::<i32>().unwrap();

        if init_month == month && init_start_period < day {
            break;
        }

        shifts_sat.push( Shift {
            name: d_sat.format("%Y-%m-%d").to_string(),
            persons: vec![]
        } );

        sat_weeks_len += 1;
    }

    for (_idx, d_sun) in first_sun_year.iter_weeks().enumerate() {
        let _year = d_sun.format("%Y").to_string().parse::<i32>().unwrap();
        let month = d_sun.format("%m").to_string().parse::<i32>().unwrap();
        let day = d_sun.format("%d").to_string().parse::<i32>().unwrap();

        if init_month == month && init_start_period < day {
            break;
        }

        shifts_sun.push( Shift {
            name: d_sun.format("%Y-%m-%d").to_string(),
            persons: vec![]
        } );

        sun_weeks_len += 1;
    }

    let max_weeks_len = if sat_weeks_len > sun_weeks_len { sat_weeks_len } else { sun_weeks_len };
    let stafts_len = stafts.len();

    let mut loop_weeks = 0;
    while loop_weeks < max_weeks_len {

        // Sort people by count
        stafts.sort_by(|a, b| b.count.cmp(&a.count));

        let sat = &mut shifts_sat[loop_weeks];
        let sun = &mut shifts_sun[loop_weeks];

        {
            let mut sat_staft: usize = 0;
            let mut sun_staft: usize = 0;

            while sat_staft < 3 {
                let staft_rev_nm = ( ( loop_weeks * 5 ) + sat_staft ) % stafts_len;

                // dbg!(staft_rev_nm);

                let staft = &mut stafts[staft_rev_nm];
                staft.count += 1;
                sat.persons.push(staft.clone());

                sat_staft += 1;
            }

            sun_staft += sat_staft;
            while sun_staft < 5 {
                let staft_rev_nm = ( ( loop_weeks * 5 ) + sun_staft ) % stafts_len;

                // dbg!(staft_rev_nm);

                let staft = &mut stafts[staft_rev_nm];
                staft.count += 1;
                sun.persons.push(staft.clone());

                sun_staft += 1;
            }

            shifts.push(sat.clone());
            shifts.push(sun.clone());

        }

        loop_weeks += 1;
    }

    // dbg!(shifts);

    let file = File::create("shifts.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &shifts)?;
    writer.flush()?;
    Ok(())

}
