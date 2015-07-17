# Discotech :globe_with_meridians: 
The perfect place to connect with that special service.

### Configuration
* Strives to avoid unavailability amplification due to problems in adjacent layers
* Receives updates from an HA store
* Ignores sharp drops (false-deconfiguration)
* Caches last known good results

### Proxy
* Provides zero-copy proxy
* Weights its backends with care to allow new members to warm-up over time, and sick members to be avoided
* Maintains between a low- and high-water mark of connections to backends
