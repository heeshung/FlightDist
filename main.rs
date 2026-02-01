use serde::{Deserialize};
use std::fs;
use std::process::Command;
use geoutils::Location;
use colored::Colorize;
use dialoguer::{Input, BasicHistory};
use unidecode::unidecode;
use reqwest::header::USER_AGENT;
use reqwest::header::ACCEPT;
use version_compare::Version;
use rand::Rng;
use terminal_hyperlink::Hyperlink;
use splitty::*;

#[derive(Debug, Deserialize)]
struct Airport {
    r#type: String,
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
    local_code: String,
    ident: String
}

#[derive(Debug, Deserialize)]
struct Runway {
    airport_ident: String,
    length_ft: i32,
    width_ft: i32,
    closed: i8,
    le_ident: String,
    he_ident: String
}

//for version check and update
#[derive(Debug, Deserialize)]
struct Response {
    tag_name: String
}

fn queryhandler(airports: &Vec<&Airport>, runways: &Vec<Runway>, unit: &str, version: &str, factypes: &Vec<&str>, unwrappedargs: String, argcounter: i32, argslen: usize, latestversion: &str) -> (i32, i32, f64) {

    //initiialize variables
    let mut lat: f64 = 0.0;
    let mut lon: f64 = 0.0;
    let mut lathold: f64 = 0.0;
    let mut lonhold: f64 = 0.0;

    let mut foundairport = &&Airport {
        r#type: "".to_string(), 
        name: "".to_string(),
        latitude_deg: 0.0,
        longitude_deg: 0.0,
        elevation_ft: Some(0),
        iso_country: "".to_string(),
        iso_region: "".to_string(),
        municipality: "".to_string(),
        icao_code: "".to_string(),
        iata_code: "".to_string(),
        local_code: "".to_string(),
        ident: "".to_string()
    };
    
    let mut airportfound: bool;
    let mut totaldist: f64 = 0.0;

    let mut counter: i32 = 0;

    let mut queries: Vec<String> = vec![];

    //split for hyphen delimiter
    let splitspaceargs = split_unquoted_char(&unwrappedargs, '-').unwrap_quotes(true);

    let collection = splitspaceargs.collect::<Vec<&str>>();
    for args in collection {
        queries.push(args.to_string());
    }

    //return if queries are blank (only hyphens in input)
    if queries.len() == 0 {
        println!("Invalid input.");
        println!("");
        return (5, counter, totaldist);
    }

    //only run if first iteration
    if argcounter == 0 {
        //check if help
        if queries[0].to_ascii_lowercase() == "help" {
            helpdisp();
            return (5, counter, totaldist);
        }

        //check if update
        else if queries[0].to_ascii_lowercase() == "update" {
            let latestversionunwrapped = Version::from(&latestversion).unwrap();
            let versionunwrapped = Version::from(version).unwrap();
            if versionunwrapped < latestversionunwrapped {
                update(version).unwrap_or_else(|error| {
                    eprintln!("Error: {}", error);
                    eprintln!("{}", "Update failed!".red().bold());
                    eprintln!("");
                    #[cfg(target_os = "windows")] {
                        press_btn_continue::wait("FlightDist will now exit. Press any key to continue...").unwrap();
                    }
                });
                #[cfg(not(target_os = "windows"))] {
                    println!("FlightDist will now exit.");
                }
                return (1, counter, totaldist);
            }
            else {
                println!("This is the latest version.");
                println!("");
                return (5, counter, totaldist);
            }
        }

        //check if about
        else if queries[0].to_ascii_lowercase() == "about" {
            aboutdisp(airports.len(), runways.len(), version, factypes, latestversion);
            return (5, counter, totaldist);
        }

        else if queries[0].to_ascii_lowercase() == "exit" {
            return (1, counter, totaldist); 
        }

        else if queries[0].to_ascii_lowercase().contains("unit=") {
            if queries[0] == ("unit=mi") {
                return (2, counter, totaldist);
            }
            else if queries[0] == ("unit=nm") {
                return (3, counter, totaldist);
            }
            else if queries[0] == ("unit=km") {
                return (4, counter, totaldist);
            }
            else {
                println!("Unrecognized unit.");
                println!("");
                return (5, counter, totaldist);
            }
        }

        //check if rand
        if queries[0].to_ascii_lowercase() == "random" {
            randomairport(airports, runways);
            return (5, counter, totaldist);
        }

        //check for single flag (search function)
        else if (queries.len() == 1) && (argslen == 1){
            airportsearch(&queries[0], airports, runways);
            return (5, counter, totaldist);
        }
    }

    if argcounter == 0 {
        println!("");
        println!("{}{}{}","IATA/ICAO - Airport Name".blue().bold(), "                                                                         Distance        ".blue().bold(), "Total".blue().bold());
    }

    println!("----------------------------------------------------------------------------------------------------------------------");

    for query in queries.iter() {
        airportfound = false;
        //check if empty term
        if query.trim().len() == 0 {
            println!("Empty search term.");
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
                    foundairport = airport;
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
                    foundairport = airport;
                    airportfound = true;
                    break;
                }
            }
        }
        //search in cities
        //city uses contains because some cities in csv contain extraneous terms
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
                        foundairport = airport;
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
                            foundairport = airport;
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
                            foundairport = airport;
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
                            foundairport = airport;
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
                        foundairport = airport;
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
                            foundairport = airport;
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
                            foundairport = airport;
                            airportfound = true;
                            break;
                        }
                    }
                }
            }
        }
        if airportfound == true {
            counter += 1;
            let distance = distancecalc(lathold,lonhold,lat,lon,unit);
            totaldist += distance;

            let mut airportnametrail = "";
            //truncate airport name if too long
            let mut airportname: &str = &foundairport.name;
            if airportname.chars().count()>74 {
                airportname = &airportname[..71];
                airportnametrail = "...";
            }
            let mut state = "";
            let mut statesuffix = "";
            let mut airportfaa = "";
            if foundairport.iso_country == "US" {
                state = &foundairport.iso_region[3..];
                statesuffix = "-";

                //faa lid if no icao
                if foundairport.icao_code == "" {
                    airportfaa = &foundairport.local_code;
                }
            }
            let paddingiata = paddingsafety(foundairport.iata_code.chars().count(), 3);
            let paddingicaofaa = paddingsafety(foundairport.icao_code.chars().count()+airportfaa.chars().count(), 4);
            let paddingname = paddingsafety(airportname.chars().count()+state.chars().count()+statesuffix.chars().count(), 77);
            println!("{:>paddingiata$}/{:>paddingicaofaa$}{} - {}{:>paddingname$} [{}{}{}] {:>8.1} {}, {:>8.1} {}", foundairport.iata_code.green().bold(), foundairport.icao_code.green().bold(), airportfaa.yellow(), airportname, airportnametrail, state.purple().bold(), statesuffix.purple().bold(), foundairport.iso_country.purple().bold(), distance, unit, totaldist, unit);
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
            println!("{}{} {}", queryname.cyan(), querytrail.cyan(), "could not be found.");
        }
    }

    //remove first airport from counter
    if counter > 0 {
        counter -= 1;
    }
    let mut countersuffix: String = "flight".to_string();
    if counter != 1 {
        countersuffix = "flights".to_string();
    }
    let paddingcounter = paddingsafety(countersuffix.chars().count()+counter.to_string().chars().count(), 103);
    let totalformat = format!("{:.1}", totaldist);
    //only print if more than one subtotal
    if argslen > 1 {
        println!("----------------------------------------------------------------------------------------------------------------------");
        if counter > 0 {
            println!("{}  {} {}{:>paddingcounter$} {}", "Subtotal:".yellow(), counter.to_string().cyan(), countersuffix, totalformat.to_string().cyan(), unit);
        }
        else {
            println!("{}  {} {}{:>95} {}", "Subtotal:".yellow(), "0".cyan(), "flights", "0".cyan(), unit);
        }
    }
    
    //only print newline for last iteration
    if argcounter == (argslen as i32)-1 {
        println!("======================================================================================================================");
        return (6, counter, totaldist);
    }
    return (0, counter, totaldist);
}

fn airportsearch(query: &String, airports: &Vec<&Airport>, runways: &Vec<Runway>) {

    let mut foundairport = &&Airport {
        r#type: "".to_string(), 
        name: "".to_string(),
        latitude_deg: 0.0,
        longitude_deg: 0.0,
        elevation_ft: Some(0),
        iso_country: "".to_string(),
        iso_region: "".to_string(),
        municipality: "".to_string(),
        icao_code: "".to_string(),
        iata_code: "".to_string(),
        local_code: "".to_string(),
        ident: "".to_string()
    };
    
    let mut airportfound: bool = false;

    //check if empty term
    if query.trim().len() == 0 {
        println!("Empty search term.");
    }

    else {
        //search icaos
        if query.chars().count() == 4 {
            for airport in airports.iter() {
                //check for icao identifiers
                if query.to_ascii_lowercase() == airport.icao_code.to_ascii_lowercase() {                
                    foundairport = airport;
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
                    foundairport = airport;
                    airportfound = true;
                    break;
                }
            }
        }
        //search in cities
        //city uses contains because some cities in csv contain extraneous terms
        if airportfound == false {
            let querysplit = query.split(", ");
            let querycollect = querysplit.collect::<Vec<&str>>();
            //try to match city and country or city and state first
            if querycollect.len() > 1{
                for airport in airports.iter() {
                    //city and state
                    if (unidecode(&airport.municipality).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (format!("{}{}", "us-", querycollect[1]).to_ascii_lowercase() == airport.iso_region.to_ascii_lowercase()) {
                        foundairport = airport;
                        airportfound = true;
                        break;
                    }
                }
                //city and country
                if airportfound == false {
                    for airport in airports.iter() {
                        if (unidecode(&airport.municipality).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (querycollect[1].to_ascii_lowercase() == airport.iso_country.to_ascii_lowercase()) {
                            foundairport = airport;
                            airportfound = true;
                            break;
                        }
                    }
                }
                //airport name and state
                if airportfound == false {
                    for airport in airports.iter() {
                        if (unidecode(&airport.name).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (format!("{}{}", "us-", querycollect[1]).to_ascii_lowercase() == airport.iso_region.to_ascii_lowercase()) {
                            foundairport = airport;
                            airportfound = true;
                            break;
                        }
                    }
                }
                //airport name and country
                if airportfound == false {
                    for airport in airports.iter() {
                        if (unidecode(&airport.name).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase())) && (querycollect[1].to_ascii_lowercase() == airport.iso_country.to_ascii_lowercase()) {
                            foundairport = airport;
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
                        foundairport = airport;
                        airportfound = true;
                        break;
                    }
                }
                //try to match city
                if airportfound == false {
                    for airport in airports.iter() {
                        if unidecode(&airport.municipality).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase()) {
                            foundairport = airport;
                            airportfound = true;
                            break;
                        }
                    }
                }
                //search in airport name
                if airportfound == false {
                    for airport in airports.iter() {
                        if unidecode(&airport.name).to_ascii_lowercase().contains(&unidecode(&querycollect[0]).to_ascii_lowercase()) {
                            foundairport = airport;
                            airportfound = true;
                            break;
                        }
                    }
                }
            }
        }
        if airportfound == true {
            airportinfo(foundairport, runways);
        }
        else {
            println!("{} {}", query.cyan(), "could not be found.");
        }
    }
    println!("");
}

fn randomairport(airports: &Vec<&Airport>, runways: &Vec<Runway>) {
    let num = rand::rng().random_range(0..airports.len());
    airportinfo(airports[num], runways);
    println!("");
}

fn airportinfo(airport: &Airport, runways: &Vec<Runway>) {
    println!("");
    println!("{}", "Airport Information".blue().bold());
    println!("-------------------");
    println!("{} {}", "Airport Name:".yellow(), airport.name);
    println!("{} {}", "IATA Code:".yellow(), airport.iata_code);
    println!("{} {}", "ICAO Code:".yellow(), airport.icao_code);
    println!("{} {}", "Local ID/FAA LID:".yellow(), airport.local_code);
    println!("{} {}", "City:".yellow(), airport.municipality);
    println!("{} {}", "State:".yellow(), airport.iso_region);
    println!("{} {}", "Country:".yellow(), airport.iso_country);
    println!("{} {}", "Latitude:".yellow(), airport.latitude_deg);
    println!("{} {}", "Longitude:".yellow(), airport.longitude_deg);

    //handle blank elevations
    let elevationfinal = airport.elevation_ft.unwrap_or(-9999999);
    if elevationfinal == -9999999 {
        println!("{} ", "Elevation:".yellow());
    }
    else {
        println!("{} {} {}", "Elevation:".yellow(), elevationfinal, "ft");
    }

    //find runways
    let mut runwaycount: i8 = 0;
    for runway in runways {
        if runway.airport_ident == airport.ident {
            //ensure runway is not closed
            if runway.closed == 0 {
                //print runways label if first found
                if runwaycount == 0 {
                    println!("");
                    println!("{}", "Runways:".yellow());
                }
                println!("[{}/{}] - {} x {} ft", runway.le_ident.green().bold(), runway.he_ident.green().bold(), runway.length_ft, runway.width_ft);
                runwaycount += 1;
            }
        }
    }
    println!("");
    println!("{}", "See location on Google Maps".hyperlink(format!("{}{}{}{}", "https://www.google.com/maps/search/?api=1&query=", airport.latitude_deg, "%2C", airport.longitude_deg)).cyan().bold());
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
    println!("Enter single search term to use search function.");
    println!("");
    println!("{}", "Flight Distance Function".yellow().bold());
    println!("Enter search terms separated by hyphens ({}).", "-".cyan());
    println!("Blocks of search terms can be delimited with semicolons ({}).", ";".cyan());
    println!("{} {} {}{}", "If a facility has an FAA LID, but not an ICAO code,", "the FAA LID will be shown in place of the ICAO code in".bold(), "yellow".yellow(), ".");
    println!("{} {}", "Example:", "HND-KSEA-o'hare;Cape Town, ZA-Fort Lauderdale, FL-S60".cyan());
    println!("");
    println!("{}", "Search Term Formatting".yellow().bold());
    println!("Acceptable search term formats in order of accuracy (highest to lowest):");
    println!("{} {}", "-ICAO Code:", "KABE".cyan());
    println!("{} {}", "-IATA Code:", "ABE".cyan());
    println!("{} {}", "-FAA LID:", "ABE".cyan());
    println!("{} {}", "-Airport Name:", "Lehigh Valley International".cyan());
    println!("{} {}", "-Airport Name, State Code:", "Lehigh Valley International, PA".cyan());
    println!("{} {}", "-Airport Name, Country Code:", "Lehigh Valley International, US".cyan());
    println!("{} {}", "-City, State Code:", "Allentown, PA".cyan());
    println!("{} {}", "-City, Country Code:", "Allentown, US".cyan());
    println!("{} {}", "-City Only:", "Allentown".cyan());
    println!("");
    println!("{}{}{}", "Search terms can be wrapped in double quotes to perform a literal search (e.g. ", r#""Wilkes-Barre, PA""#.cyan(), ").");
    println!("");
    println!("{}", "Navigation".yellow().bold());
    println!("Use up/down arrow to recall and navigate through past searches or queries.");
    println!("");
    println!("{}", "Commands".yellow().bold());
    println!("{}{}", "help".cyan(), ": Display this help screen.");
    println!("{}/{}/{}{}", "unit=mi".cyan(), "unit=km".cyan(), "unit=nm".cyan(), ": Set units for miles, kilometers, or nautical miles.");
    println!("{}{}", "random".cyan(), ": Find a random facility.");
    println!("{}{}", "update".cyan(), ": Update FlightDist.");
    println!("{}{}", "about".cyan(), ": Display about screen.");
    println!("{}{}", "exit".cyan(), ": Exits FlightDist.");
    println!("");
}

fn aboutdisp(airports_len: usize, runways_len: usize, version: &str, factypes: &Vec<&str>, latestversion: &str){
    println!("{}{}", "FlightDist v".yellow(), version.yellow());
    let latestversionunwrapped = Version::from(&latestversion).unwrap();
    let versionunwrapped = Version::from(version).unwrap();
    if versionunwrapped == latestversionunwrapped {
        println!("This is the latest version.")
    }
    else if versionunwrapped < latestversionunwrapped {
        println!("{}{}{} {}{}{}", "A new version (", latestversion.yellow(), ") is available.", "To update, type ", "update".cyan(), ".");
    }
    println!("");
    println!("Included facility types: {:?}", factypes);
    println!("{} facilities loaded.", airports_len.to_string().green().bold());
    println!("{} runways loaded.", runways_len.to_string().green().bold());
    println!("");
}

fn paddingsafety(padding: usize, default: usize) -> usize {
    if default >= padding {
        return default-padding;
    }
    else {
        return 0;
    }
}

fn update(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("heeshung")
        .repo_name("FlightDist")
        .bin_name("FlightDist")
        .show_download_progress(true)
        .current_version(version)
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());

    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("heeshung")
        .repo_name("FlightDist")
        .build()?
        .fetch()?;

    println!("Updating airport and facility data...");
    // get the first available release
    let asset = releases[0]
        .asset_for(&self_update::get_target(), None)
        .unwrap();

    let tmp_dir = tempfile::Builder::new()
            .prefix("FlightDist")
            .tempdir_in(::std::env::current_dir()?)?;

    let tmp_zip_path = tmp_dir.path().join(&asset.name);
    let file = ::std::fs::File::create(&tmp_zip_path)?;
   
    self_update::Download::from_url(&asset.download_url)
        .set_header(USER_AGENT, "FlightDist".parse()?)
        .set_header(ACCEPT, "application/octet-stream".parse()?)
        .download_to(&file)?;

    let airports_resource_name = std::path::PathBuf::from("assets/airports.csv");
    let runways_resource_name = std::path::PathBuf::from("assets/runways.csv");

    self_update::Extract::from_source(&tmp_zip_path)
        .archive(self_update::ArchiveKind::Zip)
        .extract_file(&tmp_dir.path(), &airports_resource_name)?;

    self_update::Extract::from_source(&tmp_zip_path)
        .archive(self_update::ArchiveKind::Zip)
        .extract_file(&tmp_dir.path(), &runways_resource_name)?;

    let new_airports_resource = tmp_dir.path().join(&airports_resource_name);
    let new_runways_resource = tmp_dir.path().join(&runways_resource_name);

    println!("{:?}", fs::rename(&new_airports_resource, ::std::env::current_dir()?.join(&airports_resource_name)));
    println!("{:?}", fs::rename(&new_runways_resource, ::std::env::current_dir()?.join(&runways_resource_name)));

    #[cfg(target_os = "windows")] {
        let exe_path = std::env::current_dir().unwrap().join("FlightDist.exe");
        Command::new("cmd").args(&["/C", "start", "", exe_path.to_str().unwrap()]).spawn().expect("Failed to start FlightDist, please relaunch manually.");
    }

    Ok(())
}

#[tokio::main]
async fn versioncheck() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.github.com/repos/heeshung/FlightDist/releases/latest")
        .header(USER_AGENT, "FlightDist")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(resp.tag_name)
}

fn main() {

    //set version
    let version = env!("CARGO_PKG_VERSION");

    //read files and parse csv
    let airportfile = "assets/airports.csv";
    let runwayfile = "assets/runways.csv";
    let airportdata = fs::read_to_string(airportfile).expect("File Read Error");
    let runwaydata = fs::read_to_string(runwayfile).expect("File Read Error");

    let mut airportrdr = csv::Reader::from_reader(airportdata.as_bytes());
    let mut runwayrdr = csv::Reader::from_reader(runwaydata.as_bytes());

    let mut unsortedairports: Vec<Airport> = vec![];
    let mut airports: Vec<&Airport> = vec![];

    let mut runways: Vec<Runway> = vec![];

    let factypes: Vec<&str> = vec!["large_airport", "medium_airport", "small_airport", "seaplane_base", "heliport", "balloonport"];

    //load airports
    for airport in airportrdr.deserialize() {
        let unwrappedairport: Airport = airport.unwrap();
        unsortedairports.push(unwrappedairport);
    }

    //load runways
    for runway in runwayrdr.deserialize() {
        //create null runway to use when unwrap fails for invalid runway data
        let nullrunway = Runway {
            airport_ident: "0000".to_string(),
            length_ft: 0,
            width_ft: 0,
            closed: 1,
            le_ident: "0".to_string(),
            he_ident: "0".to_string()
        };
        let unwrappedrunway: Runway = runway.unwrap_or(nullrunway);
        runways.push(unwrappedrunway);
    }

    //sort airports by facility size and icao code existence
    for factype in &factypes {
        let mut factempholdyesicao: Vec<&Airport> = vec![];
        let mut factempholdnoicao: Vec<&Airport> = vec![];
        for airport in &unsortedairports {
            if airport.r#type == factype.to_string() {
                if airport.icao_code != "" {
                    factempholdyesicao.push(airport);
                }
                else {
                    factempholdnoicao.push(airport);
                }
            }
        }
        airports.append(&mut factempholdyesicao);
        airports.append(&mut factempholdnoicao);
    }

    //initialize unit
    let mut unit = "mi";

    //initialize prompt history
    let mut history = BasicHistory::new();

    println!("{}{}", "FlightDist v".yellow().bold(), version.yellow().bold());
    
    //check if latest version
    let latestversion = versioncheck().unwrap_or("".to_string());
    let latestversionunwrapped = Version::from(&latestversion).unwrap();
    let versionunwrapped = Version::from(version).unwrap();
    if versionunwrapped < latestversionunwrapped {
        println!("{}{}{} {}{}{}", "A new version (", latestversion.yellow(), ") is available.", "To update, type ", "update".cyan(), ".");
    }

    println!("For help, type {}.", "help".cyan());
    println!("");

    loop {
        let unwrappedargs: String = Input::new().with_prompt(":").history_with(&mut history).interact_text().unwrap();
        let splitargs = split_unquoted_char(&unwrappedargs, ';').unwrap_quotes(false);
        let splitargcollection = splitargs.collect::<Vec<&str>>();
        
        //arg index
        let mut argcounter: i32 = 0;

        let mut grtotalflights: i32 = 0;
        let mut grtotaldist: f64 = 0.0;

        for args in &splitargcollection {
            let result = queryhandler(&airports, &runways, unit, version, &factypes, args.to_string(), argcounter, splitargcollection.len(), &latestversion);

            //increment argcounter
            argcounter += 1;

            //add up total flights and distance
            grtotalflights += result.1;
            grtotaldist += result.2;
            
            //exit command
            if result.0 == 1 {
                return;
            }

            else if result.0 == 2 {
                unit = "mi";
                println!("Units set to {}.", unit.cyan());
                println!("");
                break;
            }
            else if result.0 == 3 {
                unit = "nm";
                println!("Units set to {}.", unit.cyan());
                println!("");
                break;
            }
            else if result.0 == 4 {
                unit = "km";
                println!("Units set to {}.", unit.cyan());
                println!("");
                break;
            }
            //query handled, break
            else if result.0 == 5 {
                break;
            }

            else if result.0 == 6 {
                let mut grtotalflightssuffix: String = "flight".to_string();
                if grtotalflights != 1 {
                    grtotalflightssuffix = "flights".to_string();
                }
                let grpaddingcounter = paddingsafety(grtotalflightssuffix.chars().count()+grtotalflights.to_string().chars().count(), 103);
                let grtotaldistformat = format!("{:.1}", grtotaldist);
                if grtotalflights > 0 {
                    println!("{}     {} {}{:>grpaddingcounter$} {}", "Total:".red().bold(), grtotalflights.to_string().cyan().bold(), grtotalflightssuffix.bold(), grtotaldistformat.to_string().cyan().bold(), unit.bold());
                }
                else {
                    println!("{}     {} {}{:>95} {}", "Total:".red().bold(), "0".cyan().bold(), "flights".bold(), "0".cyan().bold(), unit.bold());
                }
                println!("");
            }
        }
    }
}