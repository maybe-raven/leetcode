type t
(** A collection of string frequency counts *)

val empty : t
(** The empty set of frequency counts *)

val touch : string -> t -> t
(** Bump the frequency count for the given string. *)

val to_list : t -> (string * int) list
(** Converts the set of frequency counts to an association list.
    The list is sorted by the frequency in decreasing order.
    A string shows up at most once, and the counts are >= 1. *)
