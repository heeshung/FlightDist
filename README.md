# FlightDist

FlightDist is a versatile and fast flight distance calculator.

## Usage

**Windows/Linux**: Download the appropriate binary release, unzip, and run FlightDist.<br><br>
**Note: Versions _before_ v1.7.0 cannot use the ```update``` function due to a new directory structure.  Please manually download and install v1.7.0 or later if coming from 1.6.13 or earlier.**

### Search Function

Enter single search term to use search function.

### Flight Distance Function

Enter search terms separated by hyphens (-).<br>
Blocks of search terms can be delimited with semicolons (;).<br>
If a facility has an FAA LID, but not an ICAO code, the FAA LID will be shown in place of the ICAO code in yellow.

### Search Term Formatting

Acceptable search term formats in order of accuracy (highest to lowest):
- ICAO Code: ```KABE```
- IATA Code: ```ABE```
- FAA LID: ```ABE```
- Airport Name: ```Lehigh Valley International```
- City, State Name: ```Allentown, PA```
- City, Country Code: ```Allentown, US```
- City Only: ```Allentown```

Search terms can be wrapped in double quotes to perform a literal search (e.g. ```"Wilkes-Barre, PA"```).

### Navigation

Use up/down arrow to recall and navigate through past searches or queries.

### Commands

- ```help```: Display help screen.
- ```unit=mi```/```unit=km```/```unit=nm```: Set units for miles, kilometers, or nautical miles.
- ```random```: Find a random facility.
- ```update```: Update FlightDist.
- ```about```: Display about screen.
- ```exit```: Exits FlightDist.

## Flight Distance Example

```
:: HND-KSEA-o'hare;Cape Town, ZA-Fort Lauderdale, FL-S60

IATA/ICAO - Airport Name                                                                         Distance        Total
----------------------------------------------------------------------------------------------------------------------
HND/RJTT - Tokyo Haneda International Airport                                            [JP]      0.0 mi,      0.0 mi
SEA/KSEA - Seattle–Tacoma International Airport                                       [WA-US]   4803.5 mi,   4803.5 mi
ORD/KORD - Chicago O'Hare International Airport                                       [IL-US]   1720.6 mi,   6524.1 mi
----------------------------------------------------------------------------------------------------------------------
Subtotal:  2 flights                                                                                         6524.1 mi
----------------------------------------------------------------------------------------------------------------------
CPT/FACT - Cape Town International Airport                                               [ZA]      0.0 mi,      0.0 mi
FLL/KFLL - Fort Lauderdale Hollywood International Airport                            [FL-US]   7668.9 mi,   7668.9 mi
KEH/ S60 - Kenmore Air Harbor LLC Seaplane Base                                       [WA-US]   2721.0 mi,  10389.9 mi
----------------------------------------------------------------------------------------------------------------------
Subtotal:  2 flights                                                                                        10389.9 mi
======================================================================================================================
Total:     4 flights                                                                                        16914.0 mi
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

Runways:
[04/22] - 6627 x 148 ft
[06/24] - 11283 x 148 ft
[09/27] - 11329 x 148 ft
[18C/36C] - 10826 x 148 ft
[18L/36R] - 11155 x 148 ft
[18R/36L] - 12467 x 198 ft
```

Plane icon by Icons8