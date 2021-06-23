use vsop87::vsop87c;
//use vsop87::vsop87d;

fn deg_to_hms(angle_deg:f64) -> (i32, i32, i32) {
    let d:f64 = angle_deg / 360.0 * 24.0;
    let h = d.floor() as i32;
    let m = (d.fract() * 60.0).floor() as i32;
    let s = ((d.fract() * 60.0).fract() * 60.0) as i32;
    return (h, m, s)
}

fn main() {
    // Get rectangular, ecliptic, heliocentric coordinates for a planet.
    let p_helio_ecliptic_rectcoord = vsop87c::venus(2459388.55970);

    // Get rectangular, ecliptic, heliocentric coordinates for Earth
    let e_helio_ecliptic_rectcoord = vsop87c::earth(2459388.55970);

    // Translate to rectangular, ecliptic, geocentric for planet.
    let p_geo_x = p_helio_ecliptic_rectcoord.x - e_helio_ecliptic_rectcoord.x;
    let p_geo_y = p_helio_ecliptic_rectcoord.y - e_helio_ecliptic_rectcoord.y;
    let p_geo_z = p_helio_ecliptic_rectcoord.z - e_helio_ecliptic_rectcoord.z;

    // Convert to equatorial rectangular geocentric coordinates for planet.
    // TODO SEE obliquity section at https://en.wikipedia.org/wiki/Position_of_the_Sun
    let obliquity = 23.4457889_f64.to_radians(); //earth tilt, in radians
    let p_geo_equat_x = p_geo_x;
    let p_geo_equat_y = (obliquity.cos() * p_geo_y) - (obliquity.sin() * p_geo_z);
    let p_geo_equat_z = (obliquity.sin() * p_geo_y ) + (obliquity.cos() * p_geo_z);

    // Transform cartesian coordinates to spherical, formulate in terms of atan to get right quadrant.
    let r = (p_geo_equat_x.powi(2) + p_geo_equat_y.powi(2)).powf(0.5);
    let mut declination = p_geo_equat_z.atan2(r);  //in radians
    declination = declination.to_degrees();
    let mut right_ascension = p_geo_equat_y.atan2(p_geo_equat_x); //in radians
    if right_ascension < 0.0 {
        right_ascension = 360.0 + right_ascension.to_degrees();  
    } else {
        right_ascension = right_ascension.to_degrees();
    }
    let (right_ascension_hr,right_ascension_min,right_ascension_sec) = deg_to_hms(right_ascension);
    println!("declination     = {} degrees", declination);  
    println!("right ascension = {} hr {} min {} sec", right_ascension_hr, right_ascension_min, right_ascension_sec);


    // Separate calculation, heliocentric, eliptical, spherical coordinates for Saturn.
    // let s_helio_ecliptic_spherecoord = vsop87d::saturn(2459388.71595);
    // println!("longitude={}, degrees", s_helio_ecliptic_spherecoord.longitude().to_degrees());
    // println!("latitude={}, degrees", s_helio_ecliptic_spherecoord.latitude().to_degrees());
    // println!("distance, au={}", s_helio_ecliptic_spherecoord.distance());

}
