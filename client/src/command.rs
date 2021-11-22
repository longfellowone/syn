use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Command {
    Punchin(Employee),
    Punchout(Employee),
}

#[derive(StructOpt, Debug)]
pub struct Employee {
    #[structopt(short)]
    username: String,
    #[structopt(short)]
    password: String,
}

pub fn punchin(e: Employee) {
    println!("{:?}", e);
    unimplemented!()
}

pub fn punchout(e: Employee) {
    println!("{:?}", e);
    unimplemented!()
}