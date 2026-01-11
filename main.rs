use serde::{Deserialize};
use std::fs;
use geoutils::Location;
use inquire::Text;
use colored::Colorize;

#[derive(Debug, Deserialize)]
struct Airport {
    //ident: String,
    facility: String,
    name: String,
    latitude_deg: f64,
    longitude_deg: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    elevation_ft: Option<i32>,
    iso_country: String,
    iso_region: String,
    municipality: String,
    icao_code: String,
    iata_code: String,
    local_code: String
}

fn queryhandler(airports: &Vec<Airport>, unit: &str) -> i32 {

    //initiialize variables
    let mut lat: f64 = 0.0;
    let mut lon: f64 = 0.0;
    let mut lathold: f64 = 0.0;
    let mut lonhold: f64 = 0.0;
    let mut airporticao = "";
    let mut airportiata = "";
    let mut airportname = "";
    let mut airportfound: bool;
    let mut totaldist: f64 = 0.0;

    let mut queries: Vec<String> = vec![];
    let interactiveargs = Text::new(">> ").prompt();

    let unwrappedargs = interactiveargs.unwrap();

    //split for space delimiter
    let splitspaceargs = unwrappedargs.split("-");

    let collection = splitspaceargs.collect::<Vec<&str>>();
    for args in collection {
        queries.push(args.to_string());
    }

    //check if help
    if queries[0].to_ascii_lowercase() == "help" {
        helpdisp();
    }

    else if queries[0].to_ascii_lowercase() == "exit" {
        return 1; 
    }

    //check for single flag (search function)
    else if (queries.len() == 1) && (queries[0].contains("=") == false) {
        airportsearch(&queries[0], airports);
    }

    else {
        //check for flags
        for query in queries.iter() {
            if query.to_ascii_lowercase().contains("unit=") {
                if query == ("unit=mi") {
                    return 2;
                }
                else if query == ("unit=nm") {
                    return 3;
                }
                else if query == ("unit=km") {
                    return 4;
                }
                else {
                    println!("Unrecognized unit.");
                }
                return 0;
            }
        }

        println!("");
        println!("{}{}{}","IATA/ICAO - Airport Name".blue().bold(), "                                                          Distance        ".blue().bold(), "Total".blue().bold());
        println!("-------------------------------------------------------------------------------------------------------");

        for query in queries.iter() {
            airportfound = false;
            //check if empty term
            if query.trim().len() == 0 {
                println!("Empty term.");
                continue;
            }
            //search icaos
            if query.chars().count() == 4 {
                for airport in airports.iter() {
                    //check for icao identifiers
                    if query.to_ascii_lowercase() == airport.icao_code.to_ascii_lowercase() {
                        if lathold == 0.0 {
                            lathold = airport.latitude_deg;
                            lat = airport.latitude_deg;
                        }
                        else {
                            lathold = lat;
                            lat = airport.latitude_deg;
                        }
                        if lonhold == 0.0 {
                            lonhold = airport.longitude_deg;
                            lon = airport.longitude_deg;
                        }
                        else {
                            lonhold = lon;
                            lon = airport.longitude_deg;
                        }
                        airportname = &airport.name;
                        airportiata = &airport.iata_code;
                        airporticao = &airport.icao_code;
                        airportfound = true;
                        break;
                    }
                }
            }
            //search iatas
            else if query.chars().count() == 3 {
                for airport in airports.iter() {
                    //check for iata identifiers
                    if query.to_ascii_lowercase() == airport.iata_code.to_ascii_lowercase() {
                        if lathold == 0.0 {
                            lathold = airport.latitude_deg;
                            lat = airport.latitude_deg;
                        }
                        else {
                            lathold = lat;
                            lat = airport.latitude_deg;
                        }
                        if lonhold == 0.0 {
                            lonhold = airport.longitude_deg;
                            lon = airport.longitude_deg;
                        }
                        else {
                            lonhold = lon;
                            lon = airport.longitude_deg;
                        }
                        airportname = &airport.name;
                        airportiata = &airport.iata_code;
                        airporticao = &airport.icao_code;
                        airportfound = true;
                        break;
                    }
                }
            }
            //search in cities
            if airportfound == false {
                let querysplit = query.split(", ");
                let querycollect = querysplit.collect::<Vec<&str>>();
                //try to match city and country or city and state first
                if querycollect.len() > 1{
                    for airport in airports.iter() {
                        if (airport.municipality.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase())) && (airport.iso_country.to_ascii_lowercase() == querycollect[1].to_ascii_lowercase()) {
                            if lathold == 0.0 {
                                lathold = airport.latitude_deg;
                                lat = airport.latitude_deg;
                            }
                            else {
                                lathold = lat;
                                lat = airport.latitude_deg;
                            }
                            if lonhold == 0.0 {
                                lonhold = airport.longitude_deg;
                                lon = airport.longitude_deg;
                            }
                            else {
                                lonhold = lon;
                                lon = airport.longitude_deg;
                            }
                            airportname = &airport.name;
                            airportiata = &airport.iata_code;
                            airporticao = &airport.icao_code;
                            airportfound = true;
                            break;
                        }
                    }
                    if airportfound == false {
                        for airport in airports.iter() {
                            if (airport.municipality.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase())) && (airport.iso_region.to_ascii_lowercase() == querycollect[1].to_ascii_lowercase()) {
                                if lathold == 0.0 {
                                    lathold = airport.latitude_deg;
                                    lat = airport.latitude_deg;
                                }
                                else {
                                    lathold = lat;
                                    lat = airport.latitude_deg;
                                }
                                if lonhold == 0.0 {
                                    lonhold = airport.longitude_deg;
                                    lon = airport.longitude_deg;
                                }
                                else {
                                    lonhold = lon;
                                    lon = airport.longitude_deg;
                                }
                                airportname = &airport.name;
                                airportiata = &airport.iata_code;
                                airporticao = &airport.icao_code;
                                airportfound = true;
                                break;
                            }
                        }
                    }
                }
                //if only one search term
                else {
                    //try to match local id
                    for airport in airports.iter() {
                        if airport.local_code.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase()) {
                            if lathold == 0.0 {
                                lathold = airport.latitude_deg;
                                lat = airport.latitude_deg;
                            }
                            else {
                                lathold = lat;
                                lat = airport.latitude_deg;
                            }
                            if lonhold == 0.0 {
                                lonhold = airport.longitude_deg;
                                lon = airport.longitude_deg;
                            }
                            else {
                                lonhold = lon;
                                lon = airport.longitude_deg;
                            }
                            airportname = &airport.name;
                            airportiata = &airport.iata_code;
                            airporticao = &airport.icao_code;
                            airportfound = true;
                            break;
                        }
                    }
                    //try to match city
                    if airportfound == false {
                        for airport in airports.iter() {
                            if airport.municipality.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase()) {
                                if lathold == 0.0 {
                                    lathold = airport.latitude_deg;
                                    lat = airport.latitude_deg;
                                }
                                else {
                                    lathold = lat;
                                    lat = airport.latitude_deg;
                                }
                                if lonhold == 0.0 {
                                    lonhold = airport.longitude_deg;
                                    lon = airport.longitude_deg;
                                }
                                else {
                                    lonhold = lon;
                                    lon = airport.longitude_deg;
                                }
                                airportname = &airport.name;
                                airportiata = &airport.iata_code;
                                airporticao = &airport.icao_code;
                                airportfound = true;
                                break;
                            }
                        }
                    }
                    //search in airport name
                    if airportfound == false {
                        for airport in airports.iter() {
                            if airport.name.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase()) {
                                if lathold == 0.0 {
                                    lathold = airport.latitude_deg;
                                    lat = airport.latitude_deg;
                                }
                                else {
                                    lathold = lat;
                                    lat = airport.latitude_deg;
                                }
                                if lonhold == 0.0 {
                                    lonhold = airport.longitude_deg;
                                    lon = airport.longitude_deg;
                                }
                                else {
                                    lonhold = lon;
                                    lon = airport.longitude_deg;
                                }
                                airportname = &airport.name;
                                airportiata = &airport.iata_code;
                                airporticao = &airport.icao_code;
                                airportfound = true;
                                break;
                            }
                        }
                    }
                }
            }
            if airportfound == true {
                let distance = distancecalc(lathold,lonhold,lat,lon,unit);
                totaldist += distance;
                let paddingiata = 3-airportiata.chars().count();
                let paddingicao = 4-airporticao.chars().count();
                let paddingname = 75-airportname.chars().count();
                println!("{:>paddingiata$}/{:>paddingicao$} - {} {:>paddingname$.1} {}, {:>8.1} {}", airportiata.green(), airporticao.green(), airportname, distance, unit, totaldist, unit);
            }
            else {
                println!("'{}' can not be found.", query);
            }
        }
        println!("");
    }
    return 0;
}

fn airportsearch(query: &String, airports: &Vec<Airport>) {
    //initiialize variables
    let mut airporticao = ""; 
    let mut airportiata = "";
    let mut airportlocalid = "";
    let mut airportname = "";
    let mut airportcity = "";
    let mut airportstate = "";
    let mut airportcountry = "";
    let mut elevation: Option<i32> = Some(0);
    let mut lat: f64 = 0.0;
    let mut lon: f64 = 0.0;
    let mut airportfound: bool = false;

    //check if empty term
    if query.trim().len() == 0 {
        println!("Empty term.");
    }

    else {
        //search icaos
        if query.chars().count() == 4 {
            for airport in airports.iter() {
                //check for icao identifiers
                if query.to_ascii_lowercase() == airport.icao_code.to_ascii_lowercase() {                
                    airporticao = &airport.icao_code;
                    airportiata = &airport.iata_code;
                    airportlocalid = &airport.local_code;
                    airportname = &airport.name;
                    airportcity = &airport.municipality;
                    airportstate = &airport.iso_region;
                    airportcountry = &airport.iso_country;
                    elevation = airport.elevation_ft;
                    lat = airport.latitude_deg;
                    lon = airport.longitude_deg;

                    airportfound = true;
                    break;
                }
            }
        }
        //search iatas
        else if query.chars().count() == 3 {
            for airport in airports.iter() {
                //check for iata identifiers
                if query.to_ascii_lowercase() == airport.iata_code.to_ascii_lowercase() {                
                    airporticao = &airport.icao_code;
                    airportiata = &airport.iata_code;
                    airportlocalid = &airport.local_code;
                    airportname = &airport.name;
                    airportcity = &airport.municipality;
                    airportstate = &airport.iso_region;
                    airportcountry = &airport.iso_country;
                    elevation = airport.elevation_ft;
                    lat = airport.latitude_deg;
                    lon = airport.longitude_deg;

                    airportfound = true;
                    break;
                }
            }
        }
        //search in cities
        if airportfound == false {
            let querysplit = query.split(", ");
            let querycollect = querysplit.collect::<Vec<&str>>();
            //try to match city and country or city and state first
            if querycollect.len() > 1{
                for airport in airports.iter() {
                    if (airport.municipality.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase())) && (airport.iso_country.to_ascii_lowercase() == querycollect[1].to_ascii_lowercase()) {
                        airporticao = &airport.icao_code;
                        airportiata = &airport.iata_code;
                        airportlocalid = &airport.local_code;
                        airportname = &airport.name;
                        airportcity = &airport.municipality;
                        airportstate = &airport.iso_region;
                        airportcountry = &airport.iso_country;
                        elevation = airport.elevation_ft;
                        lat = airport.latitude_deg;
                        lon = airport.longitude_deg;

                        airportfound = true;
                        break;
                    }
                }
                if airportfound == false {
                    for airport in airports.iter() {
                        if (airport.municipality.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase())) && (airport.iso_region.to_ascii_lowercase() == querycollect[1].to_ascii_lowercase()) {
                            airporticao = &airport.icao_code;
                            airportiata = &airport.iata_code;
                            airportlocalid = &airport.local_code;
                            airportname = &airport.name;
                            airportcity = &airport.municipality;
                            airportstate = &airport.iso_region;
                            airportcountry = &airport.iso_country;
                            elevation = airport.elevation_ft;
                            lat = airport.latitude_deg;
                            lon = airport.longitude_deg;

                            airportfound = true;
                            break;
                        }
                    }
                }
            }
            //if only one search term
            else {
                //try to match local id
                for airport in airports.iter() {
                    if airport.local_code.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase()) {
                        airporticao = &airport.icao_code;
                        airportiata = &airport.iata_code;
                        airportlocalid = &airport.local_code;
                        airportname = &airport.name;
                        airportcity = &airport.municipality;
                        airportstate = &airport.iso_region;
                        airportcountry = &airport.iso_country;
                        elevation = airport.elevation_ft;
                        lat = airport.latitude_deg;
                        lon = airport.longitude_deg;

                        airportfound = true;
                        break;
                    }
                }
                //try to match city
                if airportfound == false {
                    for airport in airports.iter() {
                        if airport.municipality.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase()) {
                            airporticao = &airport.icao_code;
                            airportiata = &airport.iata_code;
                            airportlocalid = &airport.local_code;
                            airportname = &airport.name;
                            airportcity = &airport.municipality;
                            airportstate = &airport.iso_region;
                            airportcountry = &airport.iso_country;
                            elevation = airport.elevation_ft;
                            lat = airport.latitude_deg;
                            lon = airport.longitude_deg;

                            airportfound = true;
                            break;
                        }
                    }
                }
                //search in airport name
                if airportfound == false {
                    for airport in airports.iter() {
                        if airport.name.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase()) {
                            airporticao = &airport.icao_code;
                            airportiata = &airport.iata_code;
                            airportlocalid = &airport.local_code;
                            airportname = &airport.name;
                            airportcity = &airport.municipality;
                            airportstate = &airport.iso_region;
                            airportcountry = &airport.iso_country;
                            elevation = airport.elevation_ft;
                            lat = airport.latitude_deg;
                            lon = airport.longitude_deg;

                            airportfound = true;
                            break;
                        }
                    }
                }
            }
        }
        if airportfound == true {
            println!("");
            println!("{}", "Airport Information".blue().bold());
            println!("-------------------");
            println!("{}{}", "Airport Name: ".yellow(), airportname);
            println!("{}{}", "IATA Code: ".yellow(), airportiata);
            println!("{}{}", "ICAO Code: ".yellow(), airporticao);
            println!("{}{}", "FAA LID: ".yellow(), airportlocalid);
            println!("{}{}", "City: ".yellow(), airportcity);
            println!("{}{}", "State: ".yellow(), airportstate);
            println!("{}{}", "Country: ".yellow(), airportcountry);
            println!("{}{}", "Latitude: ".yellow(), lat);
            println!("{}{}", "Longitude: ".yellow(), lon);

            //handle blank elevations
            let elevationfinal = elevation.unwrap_or(-9999999);
            if elevationfinal == -9999999 {
                println!("{}", "Elevation: ".yellow());
            }
            else {
                println!("{}{}{}", "Elevation: ".yellow(), elevationfinal, " ft");
            }
        }
        else {
            println!("'{}' can not be found.", query);
        }
    }
}

fn distancecalc(lathold: f64, lonhold: f64, lat: f64, lon: f64, unit: &str) -> f64 {
    //calculate distance
    let origin = Location::new(lathold, lonhold);
    let dest = Location::new(lat, lon);
    let distanceto = origin.distance_to(&dest).unwrap();
    let mut distance: f64 = 0.0;
    if unit == "mi" {
        distance = distanceto.meters()*0.0006213712;
    }
    else if unit == "nm" {
        distance = distanceto.meters()*0.0005399565;
    }
    else if unit == "km" {
        distance = distanceto.meters()*0.001;
    }
    return distance;
}

fn helpdisp() {
    println!("");
    println!("{}", "Term Formatting".yellow().bold());
    println!("Acceptable term formats in order of accuracy (highest to lowest):");
    println!("{}{}", "-ICAO Code: ", "'KABE'".cyan());
    println!("{}{}", "-IATA Code: ", "'ABE'".cyan());
    println!("{}{}", "-FAA LID: ", "'ABE'".cyan());
    println!("{}{}", "-Airport Name: ", "'Lehigh Valley International'".cyan());
    println!("{}{}", "-City, State Name: ", "'Allentown, PA'".cyan());
    println!("{}{}", "-City, Country Code: ", "'Allentown, US'".cyan());
    println!("{}{}", "-City Only: ", "'Allentown'".cyan());
    println!("");
    println!("{}", "Search Function".yellow().bold());
    println!("Enter single term to use search function.");
    println!("");
    println!("{}", "Flight Distance Function".yellow().bold());
    println!("Enter terms separated by hyphens (-).");
    println!("{}{}", "Example: ", "'hnd-ksea-o'hare-cape town, za-Fort Lauderdale, FL'".cyan());
    println!("");
    println!("{}", "Commands".yellow().bold());
    println!("{}{}", "'help'".cyan(), ": Displays this help screen.");
    println!("{}/{}/{}{}", "'unit=mi'".cyan(), "'unit=km'".cyan(), "'unit=nm'".cyan(), ": Set units for miles, kilometers, or nautical miles.");
    println!("{}{}", "'exit'".cyan(), ": Exits FlightDist.");
    println!("");
}

fn main() {
    //read files and parse json
    let file = "airports/airports.csv";
    let data = fs::read_to_string(file).expect("File Read Error");

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    let mut airports: Vec<Airport> = vec![];

    //add airports and sort by facility size
    let mut large: Vec<Airport> = vec![];
    let mut medium: Vec<Airport> = vec![];
    let mut small: Vec<Airport> = vec![];
    let mut seaplane: Vec<Airport> = vec![];

    for airport in rdr.deserialize() {
        let unwrappedairport: Airport = airport.unwrap();
        if (unwrappedairport.facility != "heliport") && (unwrappedairport.facility != "balloonport"){
            if unwrappedairport.facility == "large_airport" {
                large.push(unwrappedairport);
            }
            else if unwrappedairport.facility == "medium_airport" {
                medium.push(unwrappedairport);
            }
            else if unwrappedairport.facility == "small_airport" {
                small.push(unwrappedairport);
            }
            else if unwrappedairport.facility == "seaplane_base" {
                seaplane.push(unwrappedairport);
            }
        }
    }

    //concatenate vectors
    airports.append(&mut large);
    airports.append(&mut medium);
    airports.append(&mut small);
    airports.append(&mut seaplane);

    //initialize unit
    let mut unit = "mi";

    println!("");
    println!("Welcome to FlightDist. For help, type 'help'.");
    loop {
        let result = queryhandler(&airports, unit);
        if result == 2 {
            unit = "mi";
            println!("Units set to {}.", unit);
        }
        else if result == 3 {
            unit = "nm";
            println!("Units set to {}.", unit);
        }
        else if result == 4 {
            unit = "km";
            println!("Units set to {}.", unit);
        }
        else if result == 1 {
            break;
        }
    }
}