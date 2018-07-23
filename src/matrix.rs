extern crate cgmath;

use cgmath::{Matrix3, Matrix4};

fn pretty_print4(m: Matrix4<f32>) {
    println!(
        " {:>13.10}, {:>13.10}, {:>13.10}, {:>13.10},",
        m.x.x, m.y.x, m.z.x, m.w.x
    );
    println!(
        " {:>13.10}, {:>13.10}, {:>13.10}, {:>13.10},",
        m.x.y, m.y.y, m.z.y, m.w.y
    );
    println!(
        " {:>13.10}, {:>13.10}, {:>13.10}, {:>13.10},",
        m.x.z, m.y.z, m.z.z, m.w.z
    );
    println!(
        " {:>13.10}, {:>13.10}, {:>13.10}, {:>13.10} ",
        m.x.w, m.y.w, m.z.w, m.w.w
    );
    println!("");
}

fn pretty_print3(m: Matrix3<f32>) {
    println!(" {:>13.10}, {:>13.10}, {:>13.10}", m.x.x, m.y.x, m.z.x);
    println!(" {:>13.10}, {:>13.10}, {:>13.10}", m.x.y, m.y.y, m.z.y);
    println!(" {:>13.10}, {:>13.10}, {:>13.10}", m.x.z, m.y.z, m.z.z);
    println!("");
}
