module CounterMap = Map.Make (String)

type t = int CounterMap.t

let empty = CounterMap.empty

let touch key map =
  let f = function None -> Some 1 | Some x -> Some (x + 1) in
  CounterMap.update key f map

let to_list = CounterMap.bindings
