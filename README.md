Usage:
./parse_polygon [path] [OPTIONS]

  --delimiter, -d: Specify the csv delimiter. Default is ';'

  --inner_delimiter -i: Specify the delimiter between Polygon. Default is ','

  --output, -o: Specify the name of the kml file. Default is output.kml in the current directory

Polygon name (can be empty) | Coordinates

3. Construct DataStructure to store all polygons
 
A Polygon is defined like :
  - A name (optional) => autogenerate
  - A list of coordinates

Coordinates must be defined like this : ((lat lon, lat lon, ...))

A Multipolygon is just a list of Polygons

((lat lon, lat, lon,...)), ((lat lon, lat lon, ...))


4. Define a class to format the kml file (maybe there is a lib ?)
