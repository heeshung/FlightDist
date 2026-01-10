use serde::{Deserialize};
use std::fs;
use std::env;
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

fn queryhandler(){
    //read files and parse json
    let file = "airports/airports.json";
    let data = fs::read_to_string(file).expect("File Read Error");

    let airports: Vec<Airport> = serde_json::from_str(&data).expect("JSON Error");

    //initiialize variables
    let mut lat: f64 = 0.0;
    let mut lon: f64 = 0.0;
    let mut lathold: f64 = 0.0;
    let mut lonhold: f64 = 0.0;
    let mut airportname = "";
    let mut airporticao = "";
    let mut unit = "mi";
    let mut airportfound: bool;
    let mut totaldist: f64 = 0.0;

    //take in args
    let args: Vec<String> = env::args().collect();
    let mut queries: Vec<String> = args;
    if queries.len()==1{
        let interactiveargs = Text::new("Welcome to FlightDist. For help, type 'help'. >> ").prompt();

        //clear queries
        queries.clear();

        //add first dummy in queries vector (takes place of first arg)
        queries.push(" ".to_string());
        let unwrappedargs = interactiveargs.unwrap();

        //split for space delimiter
        let splitspaceargs = unwrappedargs.split(" ");

        let collection = splitspaceargs.collect::<Vec<&str>>();
        for args in collection{
            queries.push(args.to_string().to_ascii_uppercase());
        }
    }

    //check if help
    if queries[1]=="HELP"{
        helpdisp();
    }

    else{
        for query in queries[1.. ].iter(){
            airportfound = false;
            for airport in airports.iter(){
                if query.chars().count()==4{
                    //check for icao identifiers
                    if *query == airport.icao{
                        if lathold == 0.0 {
                            lathold = airport.lat;
                            lat = airport.lat;
                        }
                        else{
                            lathold = lat;
                            lat = airport.lat;
                        }
                        if lonhold == 0.0{
                            lonhold = airport.lon;
                            lon = airport.lon;
                        }
                        else{
                            lonhold = lon;
                            lon = airport.lon;
                        }
                        airportname = &airport.name;
                        airporticao = &airport.icao;
                        airportfound = true;
                    }
                }
                else if query.chars().count()==3{
                    //check for iata identifiers
                    if *query == airport.iata{
                        if lathold == 0.0 {
                            lathold = airport.lat;
                            lat = airport.lat;
                        }
                        else{
                            lathold = lat;
                            lat = airport.lat;
                        }
                        if lonhold == 0.0{
                            lonhold = airport.lon;
                            lon = airport.lon;
                        }
                        else{
                            lonhold = lon;
                            lon = airport.lon;
                        }
                        airportname = &airport.name;
                        airporticao = &airport.icao;
                        airportfound = true;
                    }
                }
            }
            if airportfound == true{
                let distance = distancecalc(lathold,lonhold,lat,lon,unit);
                totaldist += distance;
                println!("{}-{}, {:.2} {}, {:.2} {}", airporticao, airportname, distance, unit, totaldist, unit);
            }
            else{
                println!("'{}' is not a valid airport.", query);
            }
        }
    }
}

fn distancecalc(lathold: f64, lonhold: f64, lat: f64, lon: f64, unit: &str) -> f64 {
    //calculate distance
    let origin = Location::new(lathold, lonhold);
    let dest = Location::new(lat, lon);
    let distanceto = origin.distance_to(&dest).unwrap();
    let mut distance: f64 = 0.0;
    if unit == "mi"{
        distance = distanceto.meters()*0.0006213712;
    }
    return distance;
}

fn helpdisp(){
    println!("FlightDist Help: Enter airport codes (ICAO/IATA), separated by spaces. Output is in format 'ICAO'-'Airport Name', 'Distance', 'Total Distance'.");
    println!("Flags:");
    println!("--unit <mi><km><nm>: Set units for miles, kilometers, or nautical miles.");
}

fn main() {
    queryhandler();  
}