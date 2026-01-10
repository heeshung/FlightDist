use serde::{Deserialize};
use std::fs;
use geoutils::Location;
use inquire::Text;
use colored::Colorize;

#[derive(Debug, Deserialize)]
struct Airport {
    icao: String,
    iata: String,
    name: String,
    city: String,
    state: String,
    country: String,
    elevation: i32,
    lat: f64,
    lon: f64
}

fn airportsearch(query: &String, airports: Vec<Airport>) {
    //initiialize variables
    let mut airporticao = ""; 
    let mut airportiata = "";
    let mut airportname = "";
    let mut airportcity = "";
    let mut airportstate = "";
    let mut airportcountry = "";
    let mut elevation: i32 = 0;
    let mut lat: f64 = 0.0;
    let mut lon: f64 = 0.0;
    let mut airportfound: bool = false;

    //check if empty term
    if query.trim().len() == 0 {
        println!("Empty term.");
    }

    else {
        //search icaos
        if query.len() == 4 {
            for airport in airports.iter() {
                //check for icao identifiers
                if query.to_ascii_uppercase() == airport.icao {                
                    airporticao = &airport.icao;
                    airportiata = &airport.iata;
                    airportname = &airport.name;
                    airportcity = &airport.city;
                    airportstate = &airport.state;
                    airportcountry = &airport.country;
                    elevation = airport.elevation;
                    lat = airport.lat;
                    lon = airport.lon;

                    airportfound = true;
                    break;
                }
            }
        }
        //search iatas
        else if query.len() == 3 {
            for airport in airports.iter() {
                //check for iata identifiers
                if query.to_ascii_uppercase() == airport.iata {                
                    airporticao = &airport.icao;
                    airportiata = &airport.iata;
                    airportname = &airport.name;
                    airportcity = &airport.city;
                    airportstate = &airport.state;
                    airportcountry = &airport.country;
                    elevation = airport.elevation;
                    lat = airport.lat;
                    lon = airport.lon;

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
                if querycollect[1].len() == 2{
                    for airport in airports.iter() {
                        if (airport.city.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase())) && (airport.country.to_ascii_lowercase() == querycollect[1].to_ascii_lowercase()) {
                            airporticao = &airport.icao;
                            airportiata = &airport.iata;
                            airportname = &airport.name;
                            airportcity = &airport.city;
                            airportstate = &airport.state;
                            airportcountry = &airport.country;
                            elevation = airport.elevation;
                            lat = airport.lat;
                            lon = airport.lon;

                            airportfound = true;
                            break;
                        }
                    }
                }
                else {
                    for airport in airports.iter() {
                        if (airport.city.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase())) && (airport.state.to_ascii_lowercase() == querycollect[1].to_ascii_lowercase()) {
                            airporticao = &airport.icao;
                            airportiata = &airport.iata;
                            airportname = &airport.name;
                            airportcity = &airport.city;
                            airportstate = &airport.state;
                            airportcountry = &airport.country;
                            elevation = airport.elevation;
                            lat = airport.lat;
                            lon = airport.lon;

                            airportfound = true;
                            break;
                        }
                    }
                }
            }
            //if only one search term
            else {
                //try to match city first
                for airport in airports.iter() {
                    if airport.city.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase()) {
                        println!("yes");
                        airporticao = &airport.icao;
                        airportiata = &airport.iata;
                        airportname = &airport.name;
                        airportcity = &airport.city;
                        airportstate = &airport.state;
                        airportcountry = &airport.country;
                        elevation = airport.elevation;
                        lat = airport.lat;
                        lon = airport.lon;

                        airportfound = true;
                        break;
                    }
                }
                //search in airport name
                if airportfound == false {
                    for airport in airports.iter() {
                        if airport.name.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase()) {
                            airporticao = &airport.icao;
                            airportiata = &airport.iata;
                            airportname = &airport.name;
                            airportcity = &airport.city;
                            airportstate = &airport.state;
                            airportcountry = &airport.country;
                            elevation = airport.elevation;
                            lat = airport.lat;
                            lon = airport.lon;

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
            println!("{}{}", "City: ".yellow(), airportcity);
            println!("{}{}", "State: ".yellow(), airportstate);
            println!("{}{}", "Country: ".yellow(), airportcountry);
            println!("{}{}", "Latitude: ".yellow(), lat);
            println!("{}{}", "Longitude: ".yellow(), lon);
            println!("{}{}{}", "Elevation: ".yellow(), elevation, " ft");
        }
        else {
            println!("'{}' can not be found.", query);
        }
    }
}

fn queryhandler(data: &String, unit: &str) -> i32 {

    let airports: Vec<Airport> = serde_json::from_str(&data).expect("JSON Error");

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
    else if queries.len() == 1 {
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
            if query.len() == 4 {
                for airport in airports.iter() {
                    //check for icao identifiers
                    if query.to_ascii_uppercase() == airport.icao {
                        if lathold == 0.0 {
                            lathold = airport.lat;
                            lat = airport.lat;
                        }
                        else {
                            lathold = lat;
                            lat = airport.lat;
                        }
                        if lonhold == 0.0 {
                            lonhold = airport.lon;
                            lon = airport.lon;
                        }
                        else {
                            lonhold = lon;
                            lon = airport.lon;
                        }
                        airportname = &airport.name;
                        airportiata = &airport.iata;
                        airporticao = &airport.icao;
                        airportfound = true;
                        break;
                    }
                }
            }
            //search iatas
            else if query.len() == 3 {
                for airport in airports.iter() {
                    //check for iata identifiers
                    if query.to_ascii_uppercase() == airport.iata {
                        if lathold == 0.0 {
                            lathold = airport.lat;
                            lat = airport.lat;
                        }
                        else {
                            lathold = lat;
                            lat = airport.lat;
                        }
                        if lonhold == 0.0 {
                            lonhold = airport.lon;
                            lon = airport.lon;
                        }
                        else {
                            lonhold = lon;
                            lon = airport.lon;
                        }
                        airportname = &airport.name;
                        airportiata = &airport.iata;
                        airporticao = &airport.icao;
                        airportfound = true;
                        break;
                    }
                }
            }
            //search in cities
            if airportfound == false {
                let querysplit = query.split(", ");
                let querycollect = querysplit.collect::<Vec<&str>>();
                //try to match city and country first
                if querycollect.len() > 1{
                    for airport in airports.iter() {
                        if (airport.city.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase())) && (airport.country.to_ascii_lowercase() == querycollect[1].to_ascii_lowercase()) {
                            if lathold == 0.0 {
                                lathold = airport.lat;
                                lat = airport.lat;
                            }
                            else {
                                lathold = lat;
                                lat = airport.lat;
                            }
                            if lonhold == 0.0 {
                                lonhold = airport.lon;
                                lon = airport.lon;
                            }
                            else {
                                lonhold = lon;
                                lon = airport.lon;
                            }
                            airportname = &airport.name;
                            airportiata = &airport.iata;
                            airporticao = &airport.icao;
                            airportfound = true;
                            break;
                        }
                    }
                }
                //if only one search term
                else {
                    //try to match city first
                    for airport in airports.iter() {
                        if airport.city.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase()){
                            if lathold == 0.0 {
                                lathold = airport.lat;
                                lat = airport.lat;
                            }
                            else {
                                lathold = lat;
                                lat = airport.lat;
                            }
                            if lonhold == 0.0 {
                                lonhold = airport.lon;
                                lon = airport.lon;
                            }
                            else {
                                lonhold = lon;
                                lon = airport.lon;
                            }
                            airportname = &airport.name;
                            airportiata = &airport.iata;
                            airporticao = &airport.icao;
                            airportfound = true;
                            break;
                        }
                    }
                    //search in airport name
                    if airportfound == false {
                        for airport in airports.iter() {
                            if airport.name.to_ascii_lowercase().contains(&querycollect[0].to_ascii_lowercase()) {
                                if lathold == 0.0 {
                                    lathold = airport.lat;
                                    lat = airport.lat;
                                }
                                else {
                                    lathold = lat;
                                    lat = airport.lat;
                                }
                                if lonhold == 0.0 {
                                    lonhold = airport.lon;
                                    lon = airport.lon;
                                }
                                else {
                                    lonhold = lon;
                                    lon = airport.lon;
                                }
                                airportname = &airport.name;
                                airportiata = &airport.iata;
                                airporticao = &airport.icao;
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
                let paddingiata = 3-airportiata.len();
                let paddingicao = 4-airporticao.len();
                let paddingname = 75-airportname.len();
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
    println!("{}{}", "-Airport Name: ", "'Lehigh Valley International'".cyan());
    println!("{}{}", "-City, State Name: ", "'Allentown, Pennsylvania'".cyan());
    println!("{}{}", "-City, Country Code: ", "'Allentown, US'".cyan());
    println!("{}{}", "-City Only: ", "'Allentown'".cyan());
    println!("");
    println!("{}", "Search Function".yellow().bold());
    println!("Enter single term to use search function.");
    println!("");
    println!("{}", "Flight Distance Function".yellow().bold());
    println!("Enter airports separated by hyphens (-).  ICAO and IATA codes can be used.  Airports can also be searched via city and country code in the following format: 'New York, US'.");
    println!("Airports can also be searched by name.");
    println!("");
    println!("{}{}", "Example: ", "'hnd-ksea-la guardia-cape town, za'".cyan());
    println!("");
    println!("{}", "Commands".yellow().bold());
    println!("{}{}", "'help'".cyan(), ": Displays this help screen.");
    println!("{}/{}/{}{}", "'unit=mi'".cyan(), "'unit=km'".cyan(), "'unit=nm'".cyan(), ": Set units for miles, kilometers, or nautical miles.");
    println!("{}{}", "'exit'".cyan(), ": Exits FlightDist.");
    println!("");
}

fn main() {
    //read files and parse json
    let file = "airports/airports.json";
    let data = fs::read_to_string(file).expect("File Read Error");

    //initialize unit
    let mut unit = "mi";

    println!("");
    println!("Welcome to FlightDist. For help, type 'help'.");
    loop {
        let result = queryhandler(&data, unit);
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