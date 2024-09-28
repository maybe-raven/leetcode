let rec last = function [] -> None | [ x ] -> Some x | _ :: tl -> last tl

let rec last_two = function
  | [] | [ _ ] -> None
  | [ a; b ] -> Some (a, b)
  | _ :: tl -> last_two tl

let rec ith i l =
  match (i, l) with
  | _, [] -> raise (Failure "index out of bounds")
  | 0, x :: _ -> x
  | i, _ :: tl ->
      if i < 0 then raise (Failure "index out of bounds") else ith (i - 1) tl

let length l =
  let rec len n = function [] -> n | _ :: t -> len (n + 1) t in
  len 0 l

let rec rev list =
  let rec f acc = function [] -> acc | hd :: tl -> f (hd :: acc) tl in
  f [] list

let string_of_list to_string list =
  let rec aux acc = function
    | [] -> acc ^ "]"
    | [ x ] -> acc ^ to_string x ^ "]"
    | hd :: tl -> aux (acc ^ to_string hd ^ "; ") tl
  in
  aux "[" list

let is_palindrome l =
  let rec check = function
    | hd_norm :: tl_norm, hd_rev :: tl_rev ->
        if hd_rev = hd_norm then check (tl_norm, tl_rev) else false
    | _, _ -> true
  in

  let rec aux len rlen list rlist =
    if len = rlen then check (list, rlist)
    else if len = rlen + 1 then check (List.tl list, rlist)
    else List.(aux (len - 1) (rlen + 1) (tl list) (hd list :: rlist))
  in

  aux (length l) 0 l []

type 'a node = One of 'a | Many of 'a node list

let flatten list =
  let rec aux acc = function
    | One x -> x :: acc
    | Many [] -> acc
    | Many (hd :: tl) -> List.fold_left aux (aux acc hd) tl
  in
  List.rev (List.fold_left aux [] list)

let compress l =
  let rec aux acc = function
    | [] -> acc
    | [ x ] -> x :: acc
    | a :: tl -> if a = List.hd tl then aux acc tl else aux (a :: acc) tl
  in
  List.rev (aux [] l)
