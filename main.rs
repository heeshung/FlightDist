use serde::{Deserialize};
use std::fs;
use geoutils::Location;
use colored::Colorize;
use dialoguer::{Input, BasicHistory};
use unidecode::unidecode;

#[derive(Debug, Deserialize)]
struct Airport {
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

fn queryhandler(airports: &Vec<&Airport>, unit: &str, version: &str, factypes: &Vec<&str>, unwrappedargs: String) -> i32 {

    //initiialize variables
    let mut lat: f64 = 0.0;
    let mut lon: f64 = 0.0;
    let mut lathold: f64 = 0.0;
    let mut lonhold: f64 = 0.0;
    let mut airporticao = "";
    let mut airportiata = "";
    let mut airportname = "";
    let mut airportstate = "";
    let mut airportcountry = "";
    let mut airportfound: bool;
    let mut totaldist: f64 = 0.0;

    let mut queries: Vec<String> = vec![];

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

    //check if about
    else if queries[0].to_ascii_lowercase() == "about" {
        aboutdisp(airports.len(), version, factypes);
    }

    else if queries[0].to_ascii_lowercase() == "exit" {
        return 1; 
    }

    //check for single flag (search function)
    else if (queries.len() == 1) && (queries[0].contains("unit=") == false) {
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
                    println!("");
                }
                return 0;
            }
        }

        println!("");
        println!("{}{}{}","IATA/ICAO - Airport Name".blue().bold(), "                                                                         Distance        ".blue().bold(), "Total".blue().bold());
        println!("----------------------------------------------------------------------------------------------------------------------");

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
                        airportstate = &airport.iso_region;
                        airportcountry = &airport.iso_country;
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
                        airportstate = &airport.iso_region;
                        airportcountry = &airport.iso_country;
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
                        //city and state
                        if (unidecode(&airport.municipality).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (format!("{}{}", "us-", querycollect[1]).to_ascii_lowercase() == airport.iso_region.to_ascii_lowercase()) {
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
                            airportstate = &airport.iso_region;
                            airportcountry = &airport.iso_country;
                            airportiata = &airport.iata_code;
                            airporticao = &airport.icao_code;
                            airportfound = true;
                            break;
                        }
                    }
                    //city and country
                    if airportfound == false {
                        for airport in airports.iter() {
                            if (unidecode(&airport.municipality).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (querycollect[1].to_ascii_lowercase() == airport.iso_country.to_ascii_lowercase()) {
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
                                airportstate = &airport.iso_region;
                                airportcountry = &airport.iso_country;
                                airportiata = &airport.iata_code;
                                airporticao = &airport.icao_code;
                                airportfound = true;
                                break;
                            }
                        }
                    }
                    //airport name and state
                    if airportfound == false {
                        for airport in airports.iter() {
                            if (unidecode(&airport.name).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (format!("{}{}", "us-", querycollect[1]).to_ascii_lowercase() == airport.iso_region.to_ascii_lowercase()) {
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
                                airportstate = &airport.iso_region;
                                airportcountry = &airport.iso_country;
                                airportiata = &airport.iata_code;
                                airporticao = &airport.icao_code;
                                airportfound = true;
                                break;
                            }
                        }
                    }
                    //airport name and country
                    if airportfound == false {
                        for airport in airports.iter() {
                            if (unidecode(&airport.name).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (querycollect[1].to_ascii_lowercase() == airport.iso_country.to_ascii_lowercase()) {
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
                                airportstate = &airport.iso_region;
                                airportcountry = &airport.iso_country;
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
                        if querycollect[0].to_ascii_lowercase() == airport.local_code.to_ascii_lowercase() {
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
                            airportstate = &airport.iso_region;
                            airportcountry = &airport.iso_country;
                            airportiata = &airport.iata_code;
                            airporticao = &airport.icao_code;
                            airportfound = true;
                            break;
                        }
                    }
                    //try to match city
                    if airportfound == false {
                        for airport in airports.iter() {
                            if unidecode(&airport.municipality).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase()) {
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
                                airportstate = &airport.iso_region;
                                airportcountry = &airport.iso_country;
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
                            if unidecode(&airport.name).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase()) {
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
                                airportstate = &airport.iso_region;
                                airportcountry = &airport.iso_country;
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
                let mut airportnametrail = "";
                //truncate airport name if too long
                if airportname.chars().count()>74 {
                    airportname = &airportname[..71];
                    airportnametrail = "...";
                }
                let mut state = "";
                let mut statesuffix = "";
                if airportcountry == "US" {
                    state = &airportstate[3..];
                    statesuffix = "-";
                }
                let paddingiata = 3-airportiata.chars().count();
                let paddingicao = 4-airporticao.chars().count();
                let paddingname = 77-airportname.chars().count()-state.chars().count()-statesuffix.chars().count();
                println!("{:>paddingiata$}/{:>paddingicao$} - {}{:>paddingname$} [{}{}{}] {:>8.1} {}, {:>8.1} {}", airportiata.green(), airporticao.green(), airportname, airportnametrail, state.purple(), statesuffix.purple(), airportcountry.purple(), distance, unit, totaldist, unit);
            }
            else {
                //truncate query if too long
                let mut querytrail = "";
                let queryname;
                if query.chars().count()>70 {
                    queryname = &query[..67];
                    querytrail = "...";
                }
                else{
                    queryname = &query;
                }
                println!("'{}{}' could not be found.", queryname.cyan(), querytrail.cyan());
            }
        }
        println!("");
    }
    return 0;
}

fn airportsearch(query: &String, airports: &Vec<&Airport>) {
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
                    //city and state
                    if (unidecode(&airport.municipality).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (format!("{}{}", "us-", querycollect[1]).to_ascii_lowercase() == airport.iso_region.to_ascii_lowercase()) {
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
                //city and country
                if airportfound == false {
                    for airport in airports.iter() {
                        if (unidecode(&airport.municipality).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (querycollect[1].to_ascii_lowercase() == airport.iso_country.to_ascii_lowercase()) {
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
                //airport name and state
                if airportfound == false {
                    for airport in airports.iter() {
                        if (unidecode(&airport.name).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (format!("{}{}", "us-", querycollect[1]).to_ascii_lowercase() == airport.iso_region.to_ascii_lowercase()) {
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
                //airport name and country
                if airportfound == false {
                    for airport in airports.iter() {
                        if (unidecode(&airport.name).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (querycollect[1].to_ascii_lowercase() == airport.iso_country.to_ascii_lowercase()) {
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
                    if querycollect[0].to_ascii_lowercase() == airport.local_code.to_ascii_lowercase() {
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
                        if unidecode(&airport.municipality).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase()) {
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
                        if unidecode(&airport.name).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase()) {
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
            println!("{}{}", "Local ID/FAA LID: ".yellow(), airportlocalid);
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
            println!("'{}' could not be found.", query.cyan());
        }
    }
    println!("");
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
    println!("{}", "Search Function".yellow().bold());
    println!("Enter single term to use search function.");
    println!("");
    println!("{}", "Flight Distance Function".yellow().bold());
    println!("Enter terms separated by hyphens ('{}').", "-".cyan());
    println!("{}'{}'", "Example: ", "HND-KSEA-o'hare-Cape Town, ZA-Fort Lauderdale, FL".cyan());
    println!("");
    println!("{}", "Term Formatting".yellow().bold());
    println!("Acceptable term formats in order of accuracy (highest to lowest):");
    println!("{}'{}'", "-ICAO Code: ", "KABE".cyan());
    println!("{}'{}'", "-IATA Code: ", "ABE".cyan());
    println!("{}'{}'", "-FAA LID: ", "ABE".cyan());
    println!("{}'{}'", "-Airport Name: ", "Lehigh Valley International".cyan());
    println!("{}'{}'", "-Airport Name, State Code: ", "Lehigh Valley International, PA".cyan());
    println!("{}'{}'", "-Airport Name, Country Code: ", "Lehigh Valley International, US".cyan());
    println!("{}'{}'", "-City, State Code: ", "Allentown, PA".cyan());
    println!("{}'{}'", "-City, Country Code: ", "Allentown, US".cyan());
    println!("{}'{}'", "-City Only: ", "Allentown".cyan());
    println!("");
    println!("{}", "Navigation".yellow().bold());
    println!("Use up/down arrow to recall and navigate through past searches or queries.");
    println!("");
    println!("{}", "Commands".yellow().bold());
    println!("'{}'{}", "help".cyan(), ": Displays this help screen.");
    println!("'{}'/'{}'/'{}'{}", "unit=mi".cyan(), "unit=km".cyan(), "unit=nm".cyan(), ": Set units for miles, kilometers, or nautical miles.");
    println!("'{}'{}", "about".cyan(), ": Displays about screen.");
    println!("'{}'{}", "exit".cyan(), ": Exits FlightDist.");
    println!("");
}

fn aboutdisp(airports_len: usize, version: &str, factypes: &Vec<&str>){
    println!("{}{}", "FlightDist v".yellow(), version.yellow());
    println!("");
    println!("Included facility types: {:?}", factypes);
    println!("{} facilities loaded.", airports_len.to_string().green());
    println!("");
}

fn main() {

    //set version
    let version = env!("CARGO_PKG_VERSION");

    //read files and parse csv
    let file = "airports/airports.csv";
    let data = fs::read_to_string(file).expect("File Read Error");

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    let mut unsortedairports: Vec<Airport> = vec![];
    let mut halfsorted: Vec<&Airport> = vec![];
    let mut airports: Vec<&Airport> = vec![];

    let factypes: Vec<&str> = vec!["large_airport", "medium_airport", "small_airport", "seaplane_base", "heliport", "balloonport"];

    //load airports
    for airport in rdr.deserialize() {
        let unwrappedairport: Airport = airport.unwrap();
        unsortedairports.push(unwrappedairport);
    }

    //sort airports by facility size
    for factype in &factypes {
        for airport in &unsortedairports {
            if airport.facility == factype.to_string() {
                halfsorted.push(airport);
            }
        }
    }

    //sort airports by if icao code exists
    for airport in &halfsorted {
        if airport.icao_code != "" {
            airports.push(airport);
        }
    }

    for airport in &halfsorted {
        if airport.icao_code == "" {
            airports.push(airport);
        }
    }

    //initialize unit
    let mut unit = "mi";

    //initialize prompt history
    let mut history = BasicHistory::new();

    println!("{}{}", "FlightDist v".yellow().bold(), version.yellow().bold());
    println!("For help, type '{}'.", "help".cyan());
    println!("");

    loop {
        let unwrappedargs: String = Input::new().with_prompt(":").history_with(&mut history).interact_text().unwrap();
        let result = queryhandler(&airports, unit, version, &factypes, unwrappedargs);
        if result == 2 {
            unit = "mi";
            println!("Units set to {}.", unit.cyan());
            println!("");
        }
        else if result == 3 {
            unit = "nm";
            println!("Units set to {}.", unit.cyan());
            println!("");
        }
        else if result == 4 {
            unit = "km";
            println!("Units set to {}.", unit.cyan());
            println!("");
        }
        else if result == 1 {
            break;
        }
    }
}