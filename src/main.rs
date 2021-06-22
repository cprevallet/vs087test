use vsop87::vsop87c;
use vsop87::vsop87d;
/*
fn dec_deg_tohms(angle_deg:f64) -> (i32, i32, i32) {
    let d:f64 = angle_deg / 15.0;
    let h = d.floor() as i32;
    let m = d.floor() as i32;
    let s = d.floor() as i32;
    return (h, m, s)
}
*/

fn main() {
    // Get rectangular, ecliptic, heliocentric coordinates for Saturn.
    let s_helio_ecliptic_rectcoord = vsop87c::venus(2459388.55970);
    println!("x={}", s_helio_ecliptic_rectcoord.x);
    println!("y={}", s_helio_ecliptic_rectcoord.y);
    println!("z={}", s_helio_ecliptic_rectcoord.z);

    // Get rectangular, ecliptic, heliocentric coordinates for Saturn.
    let e_helio_ecliptic_rectcoord = vsop87c::earth(2459388.55970);
    println!("x={}", e_helio_ecliptic_rectcoord.x);
    println!("y={}", e_helio_ecliptic_rectcoord.y);
    println!("z={}", e_helio_ecliptic_rectcoord.z);

    // Translate to rectangular, ecliptic, geocentric for Saturn.
    let s_geo_x = s_helio_ecliptic_rectcoord.x - e_helio_ecliptic_rectcoord.x;
    let s_geo_y = s_helio_ecliptic_rectcoord.y - e_helio_ecliptic_rectcoord.y;
    let s_geo_z = s_helio_ecliptic_rectcoord.z - e_helio_ecliptic_rectcoord.z;
    println!("geo_x={}", s_geo_x);
    println!("geo_y={}", s_geo_y);
    println!("geo_z={}", s_geo_z);

    // Convert to equatorial rectangular geocentric coordinates for Saturn.
    // TODO SEE obliquity section at https://en.wikipedia.org/wiki/Position_of_the_Sun
    let obliquity = 23.4457889_f64.to_radians(); //earth tilt, in radians
    let s_geo_equat_x = s_geo_x;
    let s_geo_equat_y = (obliquity.cos() * s_geo_y) - (obliquity.sin() * s_geo_z);
    let s_geo_equat_z = (obliquity.sin() * s_geo_y ) + (obliquity.cos() * s_geo_z);
    println!("geo_equat_x={}", s_geo_equat_x);
    println!("geo_equat_y={}", s_geo_equat_y);
    println!("geo_equat_z={}", s_geo_equat_z);

    // Delta, distance = sqrt sum of squares, reformulate in terms of atan to get right quadrant.
    let r = (s_geo_equat_x.powi(2) + s_geo_equat_y.powi(2)).powf(0.5);
    let declination = s_geo_equat_z.atan2(r);  //in radians
    println!("r={}", r);
    println!("declination={}", declination.to_degrees());  
    let right_ascension = s_geo_equat_y.atan2(s_geo_equat_x); //in radians
    if right_ascension < 0.0 {
        println!("right ascension={}", 360.0 + right_ascension.to_degrees());  
    }
    else {
        println!("right ascension={}", right_ascension.to_degrees());  
    }

    //todo
    //let (a,b,c) = dec_deg_tohms(right_ascension);
    //println!("{:?}", (a,b,c));


    // Separate calculation, heliocentric, eliptical, spherical coordinates for Saturn.
    let s_helio_ecliptic_spherecoord = vsop87d::saturn(2459388.71595);
    println!("longitude={}, degrees", s_helio_ecliptic_spherecoord.longitude().to_degrees());
    println!("latitude={}, degrees", s_helio_ecliptic_spherecoord.latitude().to_degrees());
    println!("distance, au={}", s_helio_ecliptic_spherecoord.distance());

}
