# Discotech :globe_with_meridians: 
The perfect place to connect with that special service.

Discotech is drop-in service discovery and load balancing.  At a low level it works by hooking getaddrinfo and connect.  This allows for all kinds of high-performance and flexible patterns:
* Connection prefetching by detecting frequent identical connect requests and having a new socket ready to go for the next connect call without waiting for a round-trip.
* Drop-in SRV support for legacy systems by mapping from some SRV address -> magic ip in the getaddrinfo hook, and in the connect hook swapping the magic ip and arbitrary port to those for a selected back-end.
* Service discovery with caching and false-deconfiguration avoidance (don't delete all back-ends when the source of truth comes back from being unavailable and is suddenly empty).
* Load balancing by monitoring certain performance metrics and/or polling external responsiveness monitors and weighting discovered back-ends accordingly.

all while being completely transparent to most existing systems!

### The Dill Pickle
Discotech is what happens when engineers who have used distributed systems see existing open source service discovery and load balancing mechanisms and throw up in their mouths a little bit.
* DNS A-records are good for efficient distribution of names to TTL-respecting or short-lived clients.  Most open source infrastructure ignores DNS TTL's.  Engineers know how to write exponential backoffs, but they rarely re-resolve when their backend is unavailable.  If they do re-resolve during failure, but a back-end is only partially unavailable, you can have split configuration across your infrastructure until a deconfigured partially unavailable service is fully disabled.  A-records do not contain port information, so identical ports across all instances of a backend are common, imposing additional constraints on operation and utilization.
* DNS SRV records are adequate for service discovery for the currently small number of systems that support them.  They do not solve load balancing or re-resolution bugs for the few systems that support their use.
* The use of a local proxy/ambassador is nice in cases where you want to centralize your local service discovery and load balancing logic.  Typically a local proxy will listen on a port for a local client to connect and send a request, which will then be forwarded to a selected downstream.  In some cases local clients will try to connect to "some-service.some-cluster.some-dc.some-domain.some-tld" and a DNS system will return a specific magic IP address that triggers a forwarding rule (possibly using iptables/nftables/ipfw/pf/etc...) to forward to local-host:service-specific-port where the local proxy is listening.  Some proxies use zero-copy mechanisms like splice(2) to avoid a performance hit caused by copying the request.  There is still some added overhead, as every write to the socket from the client requires waiting on the proxy to be notified of new data and then calling splice.  The proxy is not able to intelligently set SPLICE_F_MORE as it doesn't know when the client is done sending, causing some optimization techniques to become impossible.  Another problem is you still need an external mechanism for mapping a desired service to a local proxy port.
* The use of forwarding logic like iptables is fine when you will only ever have a single back-end, and you require no load balancing logic.  Otherwise you will need to rely on external mechanisms.
* Userspace overlay networks may be convenient for certain development or prototyping situations, but they generally impose unacceptable latency and throughput hits.

### Configuration
* Strives to avoid unavailability amplification due to problems in the source of truth
* Receives updates from a configurable source: ZK, DNS SRV, flat files, etc...
* Ignores sharp drops (false-deconfiguration)
* Caches last known good results
