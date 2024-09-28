let max_probabilities (n : int) (edges : int list list) (succ_prob : float list)
    (start_node : int) (end_node : int) =
  let edge_map = Array.make n [] in

  let f prob = function
    | [ a; b ] ->
        edge_map.(a) <- (b, prob) :: edge_map.(a);
        edge_map.(b) <- (a, prob) :: edge_map.(b)
    | _ -> raise (Failure "malformed input")
  in

  List.iter2 f succ_prob edges;

  let memo = Array.make n 0.0 in
  memo.(start_node) <- 1.0;

  let q = Queue.create () in
  Queue.push start_node q;

  let rec search () =
    match Queue.take_opt q with
    | None -> ()
    | Some node ->
        let current_prob = memo.(node) in
        let f (target_node, target_prob) =
          let prob = target_prob *. current_prob in
          if memo.(target_node) < prob then (
            memo.(target_node) <- prob;
            Queue.push target_node q)
        in
        List.iter f edge_map.(node);

        search ()
  in

  search ();

  memo.(end_node)

let rec skip n list =
  match (n, list) with
  | 0, list -> list
  | _, [] -> raise (Failure "list is too short.")
  | n, _ :: tail -> skip (n - 1) tail

(* let full_int_within low high = *)
(*   let diff = high - low in *)
(*   Random.full_int diff + low *)

let rec choose_with_len n acc len list =
  if n == len then acc @ list
  else if n > len then invalid_arg "caller must ensure `n` <= `len`"
  else
    match (n, len, list) with
    | 0, 0, [] -> acc
    | _, _, [] | _, 0, _ ->
        invalid_arg "caller must ensure `len` == `List.length list`"
    | 0, _, _ -> acc
    | n, len, list ->
        let skip_n = Random.full_int len / n in
        let list = skip skip_n list in
        choose_with_len (n - 1) (List.hd list :: acc)
          (len - skip_n - 1)
          (List.tl list)

let choose n list = choose_with_len n [] (List.length list) list

let list =
  Seq.ints 0 |> Seq.take 10000
  |> Seq.flat_map (fun a ->
         Seq.ints a |> Seq.take (10000 - a) |> Seq.map (fun b -> [ a; b ]))
  |> List.of_seq

let seed = 8570148572038573;;

Random.init seed;

let edges = choose 100000 list in

let probs : float list =
  (fun () -> Random.float 1.0) |> Seq.forever |> Seq.take 100000 |> List.of_seq
in

let time f x =
  let t = Sys.time () in
  f x;
  Printf.printf "Execution time: %fs\n" (Sys.time () -. t)
in

let foo () =
  for _ = 0 to 100 do
    let _ = max_probabilities 10000 edges probs 0 9999 in
    ()
  done
in

time foo ()
