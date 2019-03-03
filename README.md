# Dirkeepeer

Highly configurable tool for organizing files
Watches directories and tries to sort appearing files according to rules

==========
## Configurations

Config file consists of directiries and rules for them

### dir %path%
Means that all following rules will be applied to files under %path%
### match %regexp% 
First part of rule, specifies which files it should apply to
### move %path%
Moves matched element to %path%


