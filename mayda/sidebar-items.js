initSidebarItems({"mod":[["error","Defines the `mayda::Error` type. Currently only used for the return types of functions defined in the `Encode` trait, but intended to allow for more complex error handling in the future."],["monotone","`Monotone` encoding of integer arrays. Intended for cases where the entries are monotonically increasing. Implemented for all primitive integer types.  "],["uniform","`Uniform` encoding of integer arrays. Intended for cases where encoding and decoding speed is desired, or the probability distribution of the entries is uniform within certain bounds. Implemented for all primitive integer types."],["unimodal","`Unimodal` encoding of integer arrays. Intended for cases where information about the probability distribution of the entries is not known, and the presence of outliers reduces the compression ratio of the other types. Implemented for all primitive integer types."],["utility","Contains constants, enums, traits and functions used by all of the encoding types provided by the `mayda` crate."]]});