# FlightDist
FlightDist is a versatile and fast flight distance calculator.

## Usage
Windows: Download and unzip release, run FlightDist.

### Search Function
Enter single term to use search function.

### Flight Distance Function
Enter terms separated by hyphens (-).

### Term Formatting
Acceptable term formats in order of accuracy (highest to lowest):
- ICAO Code: ```KABE```
- IATA Code: ```ABE```
- FAA LID: ```ABE```
- Airport Name: ```Lehigh Valley International```
- City, State Name: ```Allentown, PA```
- City, Country Code: ```Allentown, US```
- City Only: ```Allentown```

### Navigation
Use up/down arrow to recall and navigate through past searches or queries.

### Commands
- ```help```: Displays help screen.
- ```unit=mi```/```unit=km```/```unit=nm```: Set units for miles, kilometers, or nautical miles.
- ```exit```: Exits FlightDist.

## Flight Distance Example
```
:: HND-KSEA-o'hare-Cape Town, ZA-Fort Lauderdale, FL

IATA/ICAO - Airport Name                                                                         Distance        Total
----------------------------------------------------------------------------------------------------------------------
HND/RJTT - Tokyo Haneda International Airport                                            [JP]      0.0 mi,      0.0 mi
SEA/KSEA - Seattle–Tacoma International Airport                                       [WA-US]   4803.5 mi,   4803.5 mi
ORD/KORD - Chicago O'Hare International Airport                                       [IL-US]   1720.6 mi,   6524.1 mi
CPT/FACT - Cape Town International Airport                                               [ZA]   8513.0 mi,  15037.1 mi
FLL/KFLL - Fort Lauderdale Hollywood International Airport                            [FL-US]   7668.9 mi,  22706.0 mi
```

## Search Example
```
:: ams

Airport Information
-------------------
Airport Name: Amsterdam Airport Schiphol
IATA Code: AMS
ICAO Code: EHAM
Local ID/FAA LID:
City: Amsterdam
State: NL-NH
Country: NL
Latitude: 52.308601
Longitude: 4.76389
Elevation: -11 ft
```