use chrono::NaiveDateTime;
use earthquake_info::models::eew::Eew;
pub fn eew_format(eew: &Eew) -> String {
    format!(
        r"
## 緊急地震速報: 第{}報{}{}
- 震源地: **{}**
- 最大震度: **{}**  
- マグニチュード: **{}**
- 地震発生時刻: **{}**
        ",
        eew.report_num,
        if eew.is_final.unwrap() {
            "(最終)"
        } else {
            ""
        },
        if eew.is_training.unwrap() {
            "(訓練)"
        } else {
            ""
        },
        eew.region_name,
        eew.calcintensity,
        eew.magunitude,
        fix_date_format(&eew.origin_time)
    )
}

fn fix_date_format(time: &str) -> String {
    let t = NaiveDateTime::parse_from_str(time, "%Y%m%d%H%M%S").unwrap();
    t.format("%Y/%m/%d %H:%M:%S").to_string()
}
