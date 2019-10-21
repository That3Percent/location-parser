use crate::*;

// https://www.w3.org/2005/Incubator/geo/Wiki/LatitudeLongitudeAltitude
// https://tnp.uservoice.com/knowledgebase/articles/172110-latitude-longitude-formats-and-conversion
// https://www.maptools.com/tutorials/lat_lon/formats

#[test]
pub fn latitude_signed_degrees() {
    assert_eq!(parse_latitude("81.528"), Ok(81.528)); // Normal case
    assert_eq!(parse_latitude("24"), Ok(24.)); // Integer Ok
    assert_eq!(parse_latitude("0.0"), Ok(0.)); // 0 Ok
    assert_eq!(parse_latitude("90.01"), Err(Error::OutOfRange)); // Range
    assert_eq!(parse_latitude("-90.01"), Err(Error::OutOfRange)); // Range
    assert_eq!(parse_longitude("90.0"), Ok(90.)); // Range Inclusive
    assert_eq!(parse_longitude("-90.0"), Ok(-90.)); // Range Inclusive
    assert!(parse_latitude("NaN").is_err()); // Non-Nan
    assert_eq!(parse_latitude("bob.0"), Err(Error::InvalidFormat)); // Bogus
}

#[test]
pub fn longitude_signed_degrees() {
    assert_eq!(parse_longitude("161.99"), Ok(161.99)); // Out of range for latitude
    assert_eq!(parse_longitude(" 55.0 "), Ok(55.0)); // External whitespace Ok
    assert_eq!(parse_longitude("55 .0"), Err(Error::InvalidFormat)); // Internal whitespace Err
    assert_eq!(parse_longitude("2.528°"), Ok(2.528)); // Degrees symbol Ok
    assert_eq!(parse_longitude("-161.528°"), Ok(-161.528)); // Negative Ok
    assert_eq!(parse_longitude("+161.528°"), Ok(161.528)); // Positive Ok
    assert_eq!(parse_longitude("161"), Ok(161.)); // Integer Ok
    assert_eq!(parse_longitude("181.0"), Err(Error::OutOfRange)); // Range
    assert_eq!(parse_longitude("-181.0"), Err(Error::OutOfRange)); // Range
    assert_eq!(parse_longitude("180.0"), Ok(180.)); // Range Inclusive
    assert_eq!(parse_longitude("-180.0"), Ok(-180.)); // Range Inclusive
    assert!(parse_longitude("NaN").is_err()); // Non-Nan
    assert_eq!(parse_longitude("0.x"), Err(Error::InvalidFormat)); // Bogus
}

#[test]
pub fn location_signed_degrees() {
    assert_eq!(parse_location("45.0°,-10.0"), Ok((45., -10.)));
    assert_eq!(parse_location("-5.1°, 1.0"), Ok((-5.1, 1.)));
    assert_eq!(parse_location("45.0°,"), Err(Error::InvalidFormat)); // Missing longitude
    assert_eq!(parse_location("-5.1+1.0"), Ok((-5.1, 1.))); // Uses sign as separation character, common format.
}

// TODO: DMS
// https://en.wikipedia.org/wiki/ISO_6709
/*
#[test]
pub fn latitude_dms() {

}
*/

// TODO: MGRS
// https://en.wikipedia.org/wiki/Military_Grid_Reference_System
