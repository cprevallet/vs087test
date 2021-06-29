use vsop87::vsop87a;
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
#[allow(dead_code)]
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
// Decimal day should be relative to UTC.
fn julian_day(year: i16, month: u8, decimal_day: f64, cal_type: CalType) -> f64 {

    let (y, m) =
        if month == 1 || month == 2 {
            ((year - 1) as f64, (month + 12) as f64)
        } else {
            (year as f64, month as f64)
        };

    let a = (y / 100.0).trunc();
    let b = match cal_type {
        CalType::Gregorian => 2.0 - a + (a / 4.0).trunc(),
        CalType::Julian    => 0.0,
    };
    //println!("y={}",y);
    //println!("m={}",m);
    //println!("a={}",a);
    //println!("b={}",b);

    (365.25 * (y + 4716.0)).trunc()
  + (30.6001 * (m + 1.0)).trunc()
  + decimal_day
  + b
  - 1524.5
}

// Return the geocentric, spherical coordinates (RA and Declination) for a planet on a
// Julian Day.
fn vsp087_coords(julian_day:f64, planet:Planet) -> (i32, i32, i32, f64) {
    // Get cartesian, ecliptic, heliocentric coordinates for Earth.
    let e_helio_ecliptic_rectcoord = vsop87a::earth(julian_day);
    // Get cartesian, ecliptic, heliocentric coordinates for a planet.
    let p_helio_ecliptic_rectcoord = match planet {
        Planet::Mercury => vsop87a::mercury(julian_day),
        Planet::Venus => vsop87a::venus(julian_day),
        Planet::Earth => vsop87a::earth(julian_day),
        Planet::Mars=> vsop87a::mars(julian_day),
        Planet::Jupiter=> vsop87a::jupiter(julian_day),
        Planet::Saturn=> vsop87a::saturn(julian_day),
        Planet::Uranus=> vsop87a::uranus(julian_day),
        Planet::Neptune=> vsop87a::neptune(julian_day)
    };
    //let p_helio_ecliptic_rectcoord = vsop87c::venus(julian_day);
    // Translate to cartesian, ecliptic, geocentric coordinates for planet.
    let p_geo_eclip: RectangularCoordinates = RectangularCoordinates{
        x: (p_helio_ecliptic_rectcoord.x - e_helio_ecliptic_rectcoord.x),
        y: (p_helio_ecliptic_rectcoord.y - e_helio_ecliptic_rectcoord.y),
        z: (p_helio_ecliptic_rectcoord.z - e_helio_ecliptic_rectcoord.z)
    };
    // Ref: http://www.stjarnhimlen.se/comp/tutorial.html#3
    /* The position of a planet can be given in one of several ways. Two different ways that we'll use are rectangular and spherical coordinates.

        Suppose a planet is situated at some RA, Decl and r, where RA is the Right Ascension, Decl the declination, and r the distance in some length unit. If r is unknown or irrelevant, set r = 1. Let's convert this to rectangular coordinates, x,y,z:

            x = r * cos(RA) * cos(Decl)
            y = r * sin(RA) * cos(Decl)
            z = r * sin(Decl)

        (before we compute the sine/cosine of RA, we must first convert RA from hours/minutes/seconds to hours + decimals. Then the hours are converted to degrees by multiplying by 15)

        If we know the rectangular coordinates, we can convert to spherical coordinates by the formulae below:

            r    = sqrt( x*x + y*y + z*z )
            RA   = atan2( y, x )
            Decl = asin( z / r ) = atan2( z, sqrt( x*x + y*y ) )

    At the north and south celestial poles, both x and y are zero. Since atan2(0,0) is undefined, the RA is undefined too at the celestial poles. The simplest way to handle this is to assign RA some arbitrary value, e.g. zero. Close to the celestial poles the formula asin(z/r) to compute the declination becomes sensitive to round-off errors - here the formula atan2(z,sqrt(x*x+y*y)) is preferable.

    Not only equatorial coordinates can be converted between spherical and rectangular. These conversions can also be applied to ecliptic and horizontal coordinates. Just exchange RA,Decl with long,lat (ecliptic coordinates) or azimuth,altitude (horizontal coordinates).

    A coordinate system can be rotated. If a rectangular coordinate system is rotated around, say, the X axis, one can easily compute the new x,y,z coordinates. As an example, let's consider rotating an ecliptic x,y,z system to an equatorial x,y,z system. This rotation is done around the X axis (which points to the Vernal Point, the common point of origin in ecliptic and equatorial coordinates), through an angle of oblecl (the obliquity of the ecliptic, which is approximately 23.4 degrees):

        xequat = xeclip
        yequat = yeclip * cos(oblecl) - zeclip * sin(oblecl)
        zequat = yeclip * sin(oblecl) + zeclip * cos(oblecl)

    Now the x,y,z system is equatorial. It's easily rotated back to ecliptic coordinates by simply switching sign on oblecl:

        xeclip = xequat
        yeclip = yequat * cos(-oblecl) - zequat * sin(-oblecl)
        zeclip = yequat * sin(-oblecl) + zequat * cos(-oblecl)

    When computing sin and cos of -oblecl, one can use the identities:

        cos(-x) = cos(x), sin(-x) = -sin(x)
            
    */

    // Convert to cartesian, equatorial, geocentric coordinates for planet.
    // Ref: https://en.wikipedia.org/wiki/Ecliptic_coordinate_system#Conversion_between_celestial_coordinate_systems
    let obliquity = 23.43922911_f64.to_radians(); //earth tilt in J2000, in radians
    let p_geo_equat: RectangularCoordinates = RectangularCoordinates{
        x: p_geo_eclip.x,
        y: (obliquity.cos() * p_geo_eclip.y) - (obliquity.sin() * p_geo_eclip.z),
        z: (obliquity.sin() * p_geo_eclip.y ) + (obliquity.cos() * p_geo_eclip.z)
    };
    // Transform cartesian coordinates to spherical, formulate in terms of atan to get right quadrant.
    // Ref: http://www.stjarnhimlen.se/comp/tutorial.html#3
    let s = (p_geo_equat.x.powi(2) + p_geo_equat.y.powi(2)).powf(0.5);
    let mut declination = p_geo_equat.z.atan2(s);  //in radians
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

fn print_result(planet: Planet, coords: (i32, i32, i32, f64)) {
//    println!("Planet                 : {:?} ", planet); 
    let _p = match planet {
        Planet::Mercury => println!("Planet: Mercury"),
        Planet::Venus => println!("Planet: Venus"),
        Planet::Earth => println!("Planet: Earth"),
        Planet::Mars=> println!("Planet: Mars "),
        Planet::Jupiter=> println!("Planet: Jupiter"),
        Planet::Saturn=> println!("Planet: Saturn"),
        Planet::Uranus=> println!("Planet: Uranus"),
        Planet::Neptune=> println!("Planet: Neptune"),
    };
    println!("declination     (J2000): {} degrees", coords.3);  
    println!("right ascension (J2000): {} hr {} min {} sec", coords.0, coords.1, coords.2);
    println!("");

}
fn main() {
    // Pick a date and calculate the position of a planet.
    // Be careful!  Julian Days are based on UTC time.
    let jd = julian_day(2021, 6, 28.96, CalType::Gregorian);
    println!("julian day             : {} ", jd); 
    print_result(Planet::Mercury, vsp087_coords(jd, Planet::Mercury));
    print_result(Planet::Venus, vsp087_coords(jd, Planet::Venus));
    print_result(Planet::Earth, vsp087_coords(jd, Planet::Earth));
    print_result(Planet::Mars, vsp087_coords(jd, Planet::Mars));
    print_result(Planet::Jupiter, vsp087_coords(jd, Planet::Jupiter));
    print_result(Planet::Saturn, vsp087_coords(jd, Planet::Saturn));
    print_result(Planet::Neptune, vsp087_coords(jd, Planet::Neptune));
    print_result(Planet::Uranus, vsp087_coords(jd, Planet::Uranus));
//    let (ra_hr, ra_min, ra_sec, dec) = vsp087_coords(jd, Planet::Jupiter);

}
