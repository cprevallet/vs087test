use vsop87::vsop87c;
use vsop87::{RectangularCoordinates};

// Convert decimal degrees to hours, minutes, seconds.
fn deg_to_hms(angle_deg:f64) -> (i32, i32, i32) {
    let d:f64 = angle_deg / 360.0 * 24.0;
    let h = d.floor() as i32;
    let m = (d.fract() * 60.0).floor() as i32;
    let s = ((d.fract() * 60.0).fract() * 60.0) as i32;
    return (h, m, s)
}

// Represents a calendar type.
enum CalType {
    Gregorian,
    Julian
}

// Represent a planet.
enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

// Julian day calculation per Astonomical Algorithms by Meeus.
fn julian_day(year: i16, month: u8, decimal_day: f64, cal_type: CalType) -> f64 {

    let (y, m) =
        if month == 1 || month == 2 {
            ((year - 1) as f64, (month + 12) as f64)
        } else {
            (year as f64, month as f64)
        };

    let a = (y / 100.0).floor();
    let b = match cal_type {
        CalType::Gregorian => 2.0 - a + (a / 4.0).floor(),
        CalType::Julian    => 0.0,
    };

    (365.25 * (y + 4716.0)).floor()
  + (30.6001 * (m + 1.0)).floor()
  + decimal_day
  + b
  - 1524.5
}

// Return the geocentric, spherical coordinates (RA and Declination) for a planet on a
// Julian Day.
fn vsp087_coords(julian_day:f64, planet:Planet) -> (i32, i32, i32, f64) {
    // Get cartesian, ecliptic, heliocentric coordinates for Earth.
    let e_helio_ecliptic_rectcoord = vsop87c::earth(julian_day);
    // Get cartesian, ecliptic, heliocentric coordinates for a planet.
    let p_helio_ecliptic_rectcoord = match planet {
        Planet::Mercury => vsop87c::mercury(julian_day),
        Planet::Venus => vsop87c::venus(julian_day),
        Planet::Earth => vsop87c::earth(julian_day),
        Planet::Mars=> vsop87c::mars(julian_day),
        Planet::Jupiter=> vsop87c::jupiter(julian_day),
        Planet::Saturn=> vsop87c::saturn(julian_day),
        Planet::Uranus=> vsop87c::uranus(julian_day),
        Planet::Neptune=> vsop87c::neptune(julian_day)
    };
    //let p_helio_ecliptic_rectcoord = vsop87c::venus(julian_day);
    // Translate to cartesian, ecliptic, geocentric coordiates for planet.
    let p_geo_eclip: RectangularCoordinates = RectangularCoordinates{
        x: (p_helio_ecliptic_rectcoord.x - e_helio_ecliptic_rectcoord.x),
        y: (p_helio_ecliptic_rectcoord.y - e_helio_ecliptic_rectcoord.y),
        z: (p_helio_ecliptic_rectcoord.z - e_helio_ecliptic_rectcoord.z)
    };
    // Convert to cartesian, equatorial, geocentric coordinates for planet.
    // TODO SEE obliquity section at https://en.wikipedia.org/wiki/Position_of_the_Sun
    let obliquity = 23.4457889_f64.to_radians(); //earth tilt, in radians
    let p_geo_equat: RectangularCoordinates = RectangularCoordinates{
        x: p_geo_eclip.x,
        y: (obliquity.cos() * p_geo_eclip.y) - (obliquity.sin() * p_geo_eclip.z),
        z: (obliquity.sin() * p_geo_eclip.y ) + (obliquity.cos() * p_geo_eclip.z)
    };
    // Transform cartesian coordinates to spherical, formulate in terms of atan to get right quadrant.
    let r = (p_geo_equat.x.powi(2) + p_geo_equat.y.powi(2)).powf(0.5);
    let mut declination = p_geo_equat.z.atan2(r);  //in radians
    declination = declination.to_degrees();
    let mut right_ascension = p_geo_equat.y.atan2(p_geo_equat.x); //in radians
    if right_ascension < 0.0 {
        right_ascension = 360.0 + right_ascension.to_degrees();  
    } else {
        right_ascension = right_ascension.to_degrees();
    }
    let (right_ascension_hr,right_ascension_min,right_ascension_sec) = deg_to_hms(right_ascension);
    // Return the results.
    return ( right_ascension_hr, right_ascension_min, right_ascension_sec, declination );
}

fn main() {
    // Pick a date and calculate the position of a planet.
    let jd = julian_day(2021, 6, 28.5, CalType::Gregorian);
    let (ra_hr, ra_min, ra_sec, dec) = vsp087_coords(jd, Planet::Venus);
    println!("declination     = {} degrees", dec);  
    println!("right ascension = {} hr {} min {} sec", ra_hr, ra_min, ra_sec);

}
