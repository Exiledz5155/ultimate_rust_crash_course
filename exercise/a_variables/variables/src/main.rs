const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT); // let (mut var, var): (type, type) = (val, val);
    println!("Firing {} of my {} missiles!", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
    let x =  do_stuff(2.0, 12.5);

}

fn do_stuff(qty: f64, oz: f64) -> f64 {
    return qty * oz;
    // or qty * oz (so leave without semicolon)
    // something about macros???
}

