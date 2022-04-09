use std::{f64::consts::TAU, time::SystemTime};

const MOON_SYNODIC_PERIOD: f64 = 29.530588853; // Period of moon cycle in days.
const MOON_SYNODIC_OFFSET: f64 = 2451550.26; // Reference cycle offset in days.
const MOON_DISTANCE_PERIOD: f64 = 27.55454988; // Period of distance oscillation
const MOON_DISTANCE_OFFSET: f64 = 2451562.2;
const MOON_LATITUDE_PERIOD: f64 = 27.212220817; // Latitude oscillation
const MOON_LATITUDE_OFFSET: f64 = 2451565.2;
const MOON_LONGITUDE_PERIOD: f64 = 27.321582241; // Longitude oscillation
const MOON_LONGITUDE_OFFSET: f64 = 2451555.8;

// Names of lunar phases
const PHASE_NAMES: &[&str] = &[
    "New",
    "Waxing Crescent",
    "First Quarter",
    "Waxing Gibbous",
    "Full",
    "Waning Gibbous",
    "Last Quarter",
    "Waning Crescent",
];
// Names of Zodiac constellations
const ZODIAC_NAMES: [&str; 12] = [
    "Pisces",
    "Aries",
    "Taurus",
    "Gemini",
    "Cancer",
    "Leo",
    "Virgo",
    "Libra",
    "Scorpio",
    "Sagittarius",
    "Capricorn",
    "Aquarius",
];
// Ecliptic angles of Zodiac constellations
const ZODIAC_ANGLES: [f64; 12] = [
    33.18, 51.16, 93.44, 119.48, 135.30, 173.34, 224.17, 242.57, 271.26,
    302.49, 311.72, 348.58,
];

#[derive(Debug, Copy, Clone)]
pub struct MoonPhase {
    pub j_date: f64,
    pub phase: f64,                // 0 - 1, 0.5 = full
    pub age: f64,                  // Age in days of current cycle
    pub fraction: f64,             // Fraction of illuminated disk
    pub distance: f64,             // Moon distance in earth radii
    pub latitude: f64,             // Moon ecliptic latitude
    pub longitude: f64,            // Moon ecliptic longitude
    pub phase_name: &'static str,  // New, Full, etc.
    pub zodiac_name: &'static str, // Constellation
}

fn julian_date(time: SystemTime) -> f64 {
    let secs = match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs_f64(),
        Err(earlier) => -1. * earlier.duration().as_secs_f64(),
    };
    secs / 86400. + 2440587.5
}

impl MoonPhase {
    pub fn new(time: SystemTime) -> Self {
        let j_date = julian_date(time);

        // Calculate illumination (synodic) phase.
        // From number of days since new moon on Julian date MOON_SYNODIC_OFFSET
        // (1815UTC January 6, 2000), determine remainder of incomplete cycle.
        let phase =
            ((j_date - MOON_SYNODIC_OFFSET) / MOON_SYNODIC_PERIOD).fract();
        // Calculate age and illuination fraction.
        let age = phase * MOON_SYNODIC_PERIOD;
        let fraction = (1. - (std::f64::consts::TAU * phase)).cos() / 2.;
        let phase_name = PHASE_NAMES[(phase * 8.).round() as usize % 8];
        // Calculate distance fro anoalistic phase.
        let distance_phase =
            ((j_date - MOON_DISTANCE_OFFSET) / MOON_DISTANCE_PERIOD).fract();
        let distance_phase_tau = TAU * distance_phase;
        let phase_tau = 2. * TAU * phase;
        let phase_distance_tau_difference = phase_tau - distance_phase_tau;
        let distance = 60.4
            - 3.3 * distance_phase_tau.cos()
            - 0.6 * (phase_distance_tau_difference).cos()
            - 0.5 * (phase_tau).cos();

        // Calculate ecliptic latitude from nodal (draconic) phase.
        let lat_phase =
            ((j_date - MOON_LATITUDE_OFFSET) / MOON_LATITUDE_PERIOD).fract();
        let latitude = 5.1 * (TAU * lat_phase).sin();

        // Calculate ecliptic longitude ffrom sidereal motion.
        let long_phase =
            ((j_date - MOON_LONGITUDE_OFFSET) / MOON_LONGITUDE_PERIOD).fract();
        let longitude = (360. * long_phase
            + 6.3 * (distance_phase_tau).sin()
            + 1.3 * (phase_distance_tau_difference).sin()
            + 0.7 * (phase_tau).sin())
            % 360.;

        let zodiac_name = ZODIAC_ANGLES
            .iter()
            .zip(ZODIAC_NAMES.iter())
            .find_map(|(angle, name)| {
                if longitude < *angle {
                    Some(*name)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| ZODIAC_NAMES[0]);
        MoonPhase {
            j_date,
            phase,
            age,
            fraction,
            distance,
            latitude,
            longitude,
            phase_name,
            zodiac_name,
        }
    }
}
