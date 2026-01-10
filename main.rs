use serde::{Deserialize};
use std::fs;
use geoutils::Location;
use inquire::Text;

#[derive(Debug, Deserialize)]
struct Airport {
    icao: String,
    iata: String,
    name: String,
    //city: String,
    //state: String,
    //country: String,
    //elevation: i32,
    lat: f64,
    lon: f64
}

fn queryhandler(data: &String, unit: &str) -> i32 {

    let airports: Vec<Airport> = serde_json::from_str(&data).expect("JSON Error");

    //initiialize variables
    let mut lat: f64 = 0.0;
    let mut lon: f64 = 0.0;
    let mut lathold: f64 = 0.0;
    let mut lonhold: f64 = 0.0;
    let mut airportname = "";
    let mut airportiata = "";
    let mut airporticao = "";
    let mut airportfound: bool;
    let mut totaldist: f64 = 0.0;

    let mut queries: Vec<String> = vec![];
    let interactiveargs = Text::new(">> ").prompt();

    let unwrappedargs = interactiveargs.unwrap();

    //split for space delimiter
    let splitspaceargs = unwrappedargs.split(" ");

    let collection = splitspaceargs.collect::<Vec<&str>>();
    for args in collection {
        queries.push(args.to_string().to_ascii_uppercase());
    }


    //check if help
    if queries[0] == "HELP" {
        helpdisp();
    }

    else if queries[0] == "EXIT" {
        return 1; 
    }

    else {
        //check for flags
        for query in queries.iter() {
            if query.contains("UNIT-") {
                if query == ("UNIT-MI") {
                    return 2;
                }
                else if query == ("UNIT-NM") {
                    return 3;
                }
                else if query == ("UNIT-KM") {
                    return 4;
                }
                else {
                    println!("Unrecognized unit.");
                }
                return 0;
            }
        }

        println!("");
        println!("IATA/ICAO-Airport Name                                      Distance        Total");
        println!("---------------------------------------------------------------------------------");

        for query in queries.iter() {
            airportfound = false;
            for airport in airports.iter() {
                if query.chars().count()==4 {
                    //check for icao identifiers
                    if *query == airport.icao {
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
                    }
                }
                else if query.chars().count()==3 {
                    //check for iata identifiers
                    if *query == airport.iata {
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
                    }
                }
            }
            if airportfound == true {
                let distance = distancecalc(lathold,lonhold,lat,lon,unit);
                totaldist += distance;
                //let distlog = distance.log10();
                let padding = 55-airportname.len();
                println!("{}/{}-{} {:>padding$.1} {}, {:>8.1} {}", airportiata, airporticao, airportname, distance, unit, totaldist, unit);
            }
            else {
                println!("'{}' is not a valid airport.", query);
            }
        }
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
    println!("FlightDist Help: Enter airport codes (ICAO/IATA), separated by spaces. Output is in format 'IATA/ICAO'-'Airport Name', 'Distance', 'Total Distance'.");
    println!("");
    println!("Commands:");
    println!("'help': Displays this help screen.");
    println!("'unit-mi'/'unit-km'/'unit-nm': Set units for miles, kilometers, or nautical miles.");
    println!("'exit': Exits FlightDist.");
}

fn main() {
    //read files and parse json
    let file = "airports/airports.json";
    let data = fs::read_to_string(file).expect("File Read Error");

    //initialize unit
    let mut unit = "mi";

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