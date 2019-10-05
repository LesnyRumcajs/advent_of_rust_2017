#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Program {
    pub name: String,
    pub weight: i32,
}

use regex::Regex;
pub struct Tower {
    pub program: Program,
    pub disks_above: Option<Vec<String>>,
}

impl Tower {
    pub fn from_line(line: String) -> Tower {
        lazy_static! {
            static ref RE: Regex =
                Regex::new("^(?P<name>\\w+) \\((?P<weight>\\d+)\\)(?: -> (?P<next>.*))?$").unwrap();
        }
        let captures = RE.captures(line.as_str()).unwrap();
        Tower {
            program: Program {
                name: captures.name("name").unwrap().as_str().to_owned(),
                weight: captures.name("weight").unwrap().as_str().parse().unwrap(),
            },
            disks_above: match captures.name("next") {
                Some(x) => Some(x.as_str().split(", ").map(|x| x.to_owned()).collect()),
                None => None,
            },
        }
    }
}

#[test]
fn simple_disk() {
    let line = "pbga (66)".to_owned();
    let tower = Tower::from_line(line);
    assert_eq!(tower.program.name, "pbga");
    assert_eq!(tower.program.weight, 66);
    assert_eq!(tower.disks_above, None);
}

#[test]
fn complex_disk() {
    let line = "fwft (72) -> ktlj, cntj, xhth".to_owned();
    let tower = Tower::from_line(line);
    assert_eq!(tower.program.name, "fwft");
    assert_eq!(tower.program.weight, 72);
    assert_eq!(tower.disks_above.unwrap(), vec!["ktlj", "cntj", "xhth"]);
}
