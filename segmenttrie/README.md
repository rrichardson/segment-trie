# Segment Trie

This is a specialized data structure which is intended to compile and store many strings for later lookup/matching. 
The shape of the strings for matching is something like `/admin/user/*/clock/sign-in/*`

This data structure aims to provide the fastest possible matching against such strings, while being able to provide the matches against wildcard segments as parameters. 
The intended consumers of this API are implementors of HTTP/2 servers, MQTT or other pub-sub systems. 

As mentioned, this is highly specialized these are the assumptions: 

* The strings segments will be hierarchical and quite redundant
* The system expects to match against the trie far, far more often than it updates the trie.
* Duplicate segments and entire strings are expected and handled appropriately.
* The individual segments are expected to be relatively low cardinality. 
* Segments with high cardinality aren't expected, but they should be handled relatively efficiently.
* The system aims to achive fast (1s of microseconds) lookup times even with millions of strings.

Non goals: 

* Speed or ease of updates.
* Multi-Thread (Sync) capability. 


```rust

let mut tt = SegmentTrie::new();
let (b1, b2, b3, b4) = SomeBaggage::new();
tt.insert(segments("this/is/a/test/of/the/emergency/broadcast/system"), b1);
tt.insert(segments("this/is/a/test/as/well/dont/panic"), b2);
tt.insert(segments("this/is/a/test/as/well/dont/*), b3);
tt.insert(segments("this/*", b4);

let matches = tt.match(segments("this/is/a/test/as/well/dont/freakout");
let matches = tt.match(segments("this/is/a/test/as/well/dont/panic");
let num = tt.prefix_match(segments("this/is/a/test/blah"));

```
