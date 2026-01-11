# FlightDist
FlightDist is a versatile and fast flight distance calculator.

## Usage

Windows: Download and unzip release, run FlightDist.

### Term Formatting
Acceptable term formats in order of accuracy (highest to lowest):
- ICAO Code: ```KABE```
- IATA Code: ```ABE```
- FAA LID: ```ABE```
- Airport Name: ```Lehigh Valley International```
- City, State Name: ```Allentown, PA```
- City, Country Code: ```Allentown, US```
- City Only: ```Allentown```

### Search Function
Enter single term to use search function.

### Flight Distance Function
Enter terms separated by hyphens (-).

### Commands
- ```help```: Displays help screen.
- ```unit=mi```/```unit=km```/```unit=nm```: Set units for miles, kilometers, or nautical miles.
- ```exit```: Exits FlightDist.

## Example
```hnd-ksea-o'hare-cape town, za-Fort Lauderdale, FL

IATA/ICAO - Airport Name                                                          Distance        Total
-------------------------------------------------------------------------------------------------------
HND/RJTT - Tokyo Haneda International Airport                                       0.0 mi,      0.0 mi
SEA/KSEA - Seattle-Tacoma International Airport                                  4803.5 mi,   4803.5 mi
ORD/KORD - Chicago O'Hare International Airport                                  1720.6 mi,   6524.1 mi
CPT/FACT - Cape Town International Airport                                       8513.0 mi,  15037.1 mi
FLL/KFLL - Fort Lauderdale Hollywood International Airport                       7668.9 mi,  22706.0 mi
```